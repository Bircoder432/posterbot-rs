use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::ChatId;

use crate::bot::{AppState, media};
use crate::db::models::NewMessage;
use crate::locales::L10n;

pub async fn handle_start(bot: &Bot, msg: &Message, state: &AppState, args: &str) -> Result<()> {
    let user_id = msg.from.as_ref().unwrap().id.0 as i64;
    let chat_id = msg.chat.id;
    let lang = state.db.get_language().await?;

    if let Some(parent_id_str) = args.strip_prefix("reply_") {
        if let Ok(parent_id) = parent_id_str.parse::<i64>() {
            if state.db.get_message_by_id(parent_id).await?.is_some() {
                state
                    .db
                    .set_user_state(user_id, "reply_mode", parent_id)
                    .await?;
                bot.send_message(chat_id, L10n::send_reply_to_post(lang))
                    .await?;
                return Ok(());
            }
        }
        bot.send_message(chat_id, L10n::invalid_reply_link(lang))
            .await?;
        return Ok(());
    }

    if state.db.is_banned(user_id).await? {
        bot.send_message(chat_id, L10n::user_banned(lang)).await?;
        return Ok(());
    }

    if state.db.is_admin(user_id).await? || user_id == state.config.owner_id {
        let text = if user_id == state.config.owner_id {
            L10n::owner_panel(lang)
        } else {
            L10n::mod_panel(lang)
        };
        bot.send_message(chat_id, text).await?;
    } else {
        bot.send_message(chat_id, L10n::welcome(lang)).await?;
    }

    Ok(())
}

pub async fn handle_reply_command(
    bot: &Bot,
    msg: &Message,
    state: &AppState,
    args: &str,
) -> Result<()> {
    let user_id = msg.from.as_ref().unwrap().id.0 as i64;
    let lang = state.db.get_language().await?;

    let parent_id: i64 = match args.trim().parse() {
        Ok(id) if id > 0 => id,
        _ => {
            bot.send_message(msg.chat.id, L10n::reply_usage(lang))
                .await?;
            return Ok(());
        }
    };

    if state.db.get_message_by_id(parent_id).await?.is_none() {
        bot.send_message(msg.chat.id, L10n::post_not_found(lang))
            .await?;
        return Ok(());
    }

    state
        .db
        .set_user_state(user_id, "reply_mode", parent_id)
        .await?;
    bot.send_message(msg.chat.id, L10n::send_reply_to_post(lang))
        .await?;
    Ok(())
}

pub async fn handle_proposal(bot: &Bot, msg: &Message, state: &AppState) -> Result<()> {
    let user_id = msg.from.as_ref().unwrap().id.0 as i64;
    let chat_id = msg.chat.id;
    let lang = state.db.get_language().await?;

    if state.db.is_banned(user_id).await? {
        bot.send_message(chat_id, L10n::user_banned(lang)).await?;
        return Ok(());
    }

    if msg.chat.is_group() || msg.chat.is_supergroup() {
        return Ok(());
    }

    if !has_content(msg) {
        return Ok(());
    }

    if state.db.message_exists(chat_id.0, msg.id.0 as i32).await? {
        return Ok(());
    }

    let (media_type, media_file_id) = media::extract_media_info(msg, lang);
    let message_text = media::extract_message_text(msg, lang);
    let media_group_id = msg.media_group_id().map(|s| s.to_string());

    let proposal_group_id = media_group_id
        .clone()
        .unwrap_or_else(|| format!("single_{}", uuid::Uuid::new_v4()));

    let new_msg = NewMessage {
        chat_id: chat_id.0,
        telegram_message_id: msg.id.0 as i32,
        sender_id: user_id,
        message_text,
        media_type,
        media_file_id,
        media_group_id,
        proposal_group_id,
        channel_id: state.config.channel_id,
        parent_message_id: None,
    };

    let inserted = state.db.save_message(&new_msg).await?;

    if inserted {
        bot.send_message(chat_id, L10n::proposal_accepted(lang))
            .await?;
        notify_admins(bot, state, &new_msg, lang).await?;
    }

    Ok(())
}

pub async fn handle_reply_content(
    bot: &Bot,
    msg: &Message,
    state: &AppState,
    parent_id: i64,
) -> Result<()> {
    let user_id = msg.from.as_ref().unwrap().id.0 as i64;
    let chat_id = msg.chat.id;
    let lang = state.db.get_language().await?;

    if state.db.message_exists(chat_id.0, msg.id.0 as i32).await? {
        state.db.clear_user_state(user_id).await?;
        return Ok(());
    }

    let (media_type, media_file_id) = media::extract_media_info(msg, lang);
    let message_text = media::extract_message_text(msg, lang);
    let media_group_id = msg.media_group_id().map(|s| s.to_string());

    let proposal_group_id = media_group_id
        .clone()
        .unwrap_or_else(|| format!("single_{}", uuid::Uuid::new_v4()));

    let new_msg = NewMessage {
        chat_id: chat_id.0,
        telegram_message_id: msg.id.0 as i32,
        sender_id: user_id,
        message_text,
        media_type,
        media_file_id,
        media_group_id,
        proposal_group_id,
        channel_id: state.config.channel_id,
        parent_message_id: Some(parent_id),
    };

    match state.db.save_message(&new_msg).await {
        Ok(true) => {
            state.db.clear_user_state(user_id).await?;
            bot.send_message(chat_id, L10n::reply_accepted(lang))
                .await?;
            notify_admins(bot, state, &new_msg, lang).await?;
        }
        Ok(false) => {
            // Duplicate, silently ignore
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to save reply");
            bot.send_message(chat_id, L10n::error_sending_reply(lang))
                .await?;
            state.db.clear_user_state(user_id).await?;
        }
    }

    Ok(())
}

pub async fn handle_send_reason(
    bot: &Bot,
    msg: &Message,
    state: &AppState,
    target_user_id: i64,
) -> Result<()> {
    let admin_id = msg.from.as_ref().unwrap().id.0 as i64;
    let lang = state.db.get_language().await?;
    let reason = msg.text().unwrap_or("No reason provided");

    let _ = bot
        .send_message(ChatId(target_user_id), L10n::rejected_reason(lang, reason))
        .await;

    state.db.clear_user_state(admin_id).await?;
    bot.send_message(msg.chat.id, L10n::reason_sent(lang))
        .await?;
    Ok(())
}

pub async fn handle_send_ban_reason(
    bot: &Bot,
    msg: &Message,
    state: &AppState,
    target_user_id: i64,
) -> Result<()> {
    let admin_id = msg.from.as_ref().unwrap().id.0 as i64;
    let lang = state.db.get_language().await?;
    let reason = msg.text().unwrap_or("No reason provided");

    match state.db.create_ban_record(target_user_id, reason).await {
        Ok(ban_id) => {
            state.db.ban_user(target_user_id).await?;
            state.db.clear_user_state(admin_id).await?;

            let _ = bot
                .send_message(
                    ChatId(target_user_id),
                    L10n::user_banned_appeal(lang, &ban_id),
                )
                .await;
            bot.send_message(msg.chat.id, L10n::user_banned_success(lang))
                .await?;
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to ban user");
            bot.send_message(msg.chat.id, L10n::error_banning_user(lang))
                .await?;
            state.db.clear_user_state(admin_id).await?;
        }
    }

    Ok(())
}

fn has_content(msg: &Message) -> bool {
    msg.text().is_some()
        || msg.photo().is_some()
        || msg.document().is_some()
        || msg.video().is_some()
        || msg.video_note().is_some()
        || msg.audio().is_some()
        || msg.voice().is_some()
        || msg.sticker().is_some()
}

async fn notify_admins(
    bot: &Bot,
    state: &AppState,
    msg: &NewMessage,
    lang: crate::locales::Locale,
) -> Result<()> {
    let admins = state.db.get_admins().await?;
    let notification = L10n::new_proposal_notif(
        lang,
        &msg.proposal_group_id,
        &msg.message_text,
        &msg.media_type,
    );

    for admin in admins {
        let _ = bot.send_message(ChatId(admin.user_id), &notification).await;
    }
    Ok(())
}
