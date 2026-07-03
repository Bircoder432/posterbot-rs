use anyhow::{Result, anyhow};
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, MaybeInaccessibleMessage};

use crate::bot::{AppState, media};
use crate::locales::{L10n, Locale};

pub async fn handle_proposals(bot: &Bot, msg: &Message, state: &AppState) -> Result<()> {
    let user_id = msg.from.as_ref().unwrap().id.0 as i64;
    let lang = state.db.get_language().await?;

    if !is_authorized(state, user_id).await? {
        bot.send_message(msg.chat.id, L10n::no_access(lang)).await?;
        return Ok(());
    }

    show_next_proposal(bot, msg.chat.id, state, lang).await
}

pub async fn handle_pardon(bot: &Bot, msg: &Message, state: &AppState, args: &str) -> Result<()> {
    let user_id = msg.from.as_ref().unwrap().id.0 as i64;
    let lang = state.db.get_language().await?;

    if !is_authorized(state, user_id).await? {
        bot.send_message(msg.chat.id, L10n::no_access(lang)).await?;
        return Ok(());
    }

    let ban_id = args.trim();
    if ban_id.is_empty() {
        bot.send_message(msg.chat.id, L10n::pardon_usage(lang))
            .await?;
        return Ok(());
    }

    match state.db.get_ban_record(ban_id).await? {
        None => {
            bot.send_message(msg.chat.id, L10n::ban_not_found(lang))
                .await?;
        }
        Some(record) => {
            state.db.pardon_user(record.user_id).await?;
            state.db.deactivate_ban(ban_id).await?;

            tracing::info!(ban_id = %ban_id, user_id = record.user_id, "Ban pardoned");

            let _ = bot
                .send_message(ChatId(record.user_id), L10n::access_restored(lang))
                .await;
            bot.send_message(msg.chat.id, L10n::ban_deactivated(lang, ban_id))
                .await?;
        }
    }

    Ok(())
}

pub async fn handle_callback(bot: &Bot, q: &CallbackQuery, state: &AppState) -> Result<()> {
    let user_id = q.from.id.0 as i64;
    let lang = state.db.get_language().await?;

    if !is_authorized(state, user_id).await? {
        bot.answer_callback_query(q.id.clone())
            .text(L10n::no_access(lang))
            .await?;
        return Ok(());
    }

    let (chat_id, message_id) = match &q.message {
        Some(MaybeInaccessibleMessage::Regular(msg)) => (msg.chat.id, msg.id),
        _ => return Ok(()),
    };

    let data = q.data.as_deref().unwrap_or("");

    if data == "next" {
        show_next_proposal(bot, chat_id, state, lang).await?;
        bot.answer_callback_query(q.id.clone())
            .text(L10n::next_btn(lang))
            .await?;
        return Ok(());
    }

    if let Some(id_str) = data.strip_prefix("approve_") {
        let id: i64 = id_str.parse().map_err(|_| anyhow!("Invalid callback id"))?;
        handle_approve(bot, chat_id, id, q, state, lang).await?;
    } else if let Some(id_str) = data.strip_prefix("reject_") {
        let id: i64 = id_str.parse().map_err(|_| anyhow!("Invalid callback id"))?;
        handle_reject(bot, chat_id, id, q, state, lang).await?;
    } else if let Some(id_str) = data.strip_prefix("reason_") {
        let sender_id: i64 = id_str.parse().map_err(|_| anyhow!("Invalid callback id"))?;
        handle_reason(bot, chat_id, sender_id, q, state, lang).await?;
    } else if let Some(id_str) = data.strip_prefix("ban_reason_") {
        let sender_id: i64 = id_str.parse().map_err(|_| anyhow!("Invalid callback id"))?;
        handle_ban_reason(bot, chat_id, sender_id, q, state, lang).await?;
    }

    let _ = (message_id,);
    Ok(())
}

async fn is_authorized(state: &AppState, user_id: i64) -> Result<bool> {
    Ok(user_id == state.config.owner_id || state.db.is_admin(user_id).await?)
}

async fn show_next_proposal(
    bot: &Bot,
    chat_id: ChatId,
    state: &AppState,
    lang: Locale,
) -> Result<()> {
    match state.db.get_next_pending_proposal().await? {
        None => {
            bot.send_message(chat_id, L10n::no_new_proposals(lang))
                .await?;
        }
        Some(proposal) => {
            bot.send_message(
                chat_id,
                L10n::proposal_items_count(lang, proposal.messages.len()),
            )
            .await?;

            if let Err(e) = media::send_for_moderation(bot, chat_id, &proposal, lang).await {
                bot.send_message(chat_id, L10n::failed_display_media(lang, &e.to_string()))
                    .await?;
            }

            let first = proposal.first();
            let text = L10n::proposal_action_header(
                lang,
                first.id,
                &first.created_at.format("%d.%m.%Y %H:%M").to_string(),
            );

            let kb = InlineKeyboardMarkup::new(vec![vec![
                InlineKeyboardButton::callback(
                    L10n::approve_btn(lang),
                    format!("approve_{}", first.id),
                ),
                InlineKeyboardButton::callback(
                    L10n::reject_btn(lang),
                    format!("reject_{}", first.id),
                ),
            ]]);

            bot.send_message(chat_id, text).reply_markup(kb).await?;
        }
    }
    Ok(())
}

async fn handle_approve(
    bot: &Bot,
    chat_id: ChatId,
    msg_id: i64,
    q: &CallbackQuery,
    state: &AppState,
    lang: Locale,
) -> Result<()> {
    let msg = state
        .db
        .get_message_by_id(msg_id)
        .await?
        .ok_or_else(|| anyhow!("Message not found"))?;

    let group_id = msg.proposal_group_id.clone();

    let proposal = state
        .db
        .get_proposal_by_group_id(&group_id)
        .await?
        .ok_or_else(|| anyhow!("Proposal not found"))?;

    let reply_to = if let Some(parent_id) = proposal.first().parent_message_id {
        match state.db.get_message_by_id(parent_id).await? {
            Some(parent) if parent.channel_message_id.is_some() => parent.channel_message_id,
            _ => None,
        }
    } else {
        None
    };

    match media::publish(
        bot,
        state.config.channel_id,
        &proposal,
        &state.config.bot_username,
        reply_to,
        lang,
    )
    .await
    {
        Ok(Some(channel_msg_id)) => {
            state
                .db
                .update_channel_message_id(&group_id, channel_msg_id)
                .await?;
            state
                .db
                .update_proposal_status(&group_id, "approved")
                .await?;

            tracing::info!(
                proposal_id = proposal.first().id,
                "Proposal approved and published"
            );

            bot.answer_callback_query(q.id.clone())
                .text(L10n::published(lang))
                .await?;
            delete_callback_message(bot, chat_id, q).await?;
            show_next_proposal(bot, chat_id, state, lang).await?;
        }
        Ok(None) => {
            bot.answer_callback_query(q.id.clone())
                .text(L10n::published_no_id(lang))
                .await?;
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to publish proposal");
            bot.answer_callback_query(q.id.clone())
                .text(L10n::failed_publish(lang))
                .await?;
        }
    }

    Ok(())
}

async fn handle_reject(
    bot: &Bot,
    chat_id: ChatId,
    msg_id: i64,
    q: &CallbackQuery,
    state: &AppState,
    lang: Locale,
) -> Result<()> {
    let msg = state
        .db
        .get_message_by_id(msg_id)
        .await?
        .ok_or_else(|| anyhow!("Message not found"))?;

    let sender_id = msg.sender_id;
    let group_id = msg.proposal_group_id.clone();

    state
        .db
        .update_proposal_status(&group_id, "rejected")
        .await?;
    state.db.delete_proposal(&group_id).await?;

    bot.answer_callback_query(q.id.clone())
        .text(L10n::rejected(lang))
        .await?;
    delete_callback_message(bot, chat_id, q).await?;

    let kb = InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::callback(L10n::reason_btn(lang), format!("reason_{sender_id}")),
        InlineKeyboardButton::callback(L10n::next_btn(lang), "next"),
        InlineKeyboardButton::callback(L10n::ban_btn(lang), format!("ban_reason_{sender_id}")),
    ]]);

    bot.send_message(chat_id, L10n::choose_action(lang))
        .reply_markup(kb)
        .await?;

    Ok(())
}

async fn handle_reason(
    bot: &Bot,
    chat_id: ChatId,
    sender_id: i64,
    q: &CallbackQuery,
    state: &AppState,
    lang: Locale,
) -> Result<()> {
    let admin_id = q.from.id.0 as i64;
    bot.send_message(chat_id, L10n::enter_rejection_reason(lang))
        .await?;
    state
        .db
        .set_user_state(admin_id, "reason", sender_id)
        .await?;
    bot.answer_callback_query(q.id.clone())
        .text(L10n::enter_reason_callback(lang))
        .await?;
    Ok(())
}

async fn handle_ban_reason(
    bot: &Bot,
    chat_id: ChatId,
    sender_id: i64,
    q: &CallbackQuery,
    state: &AppState,
    lang: Locale,
) -> Result<()> {
    let admin_id = q.from.id.0 as i64;
    bot.send_message(chat_id, L10n::enter_ban_reason(lang))
        .await?;
    state
        .db
        .set_user_state(admin_id, "ban_reason", sender_id)
        .await?;
    bot.answer_callback_query(q.id.clone())
        .text(L10n::enter_reason_callback(lang))
        .await?;
    Ok(())
}

async fn delete_callback_message(bot: &Bot, chat_id: ChatId, q: &CallbackQuery) -> Result<()> {
    if let Some(MaybeInaccessibleMessage::Regular(msg)) = &q.message {
        bot.delete_message(chat_id, msg.id).await?;
    }
    Ok(())
}
