use anyhow::Result;
use teloxide::prelude::*;

use crate::bot::AppState;
use crate::locales::{L10n, Locale};

pub async fn handle_set_language(
    bot: &Bot,
    msg: &Message,
    state: &AppState,
    args: &str,
) -> Result<()> {
    let user_id = msg.from.as_ref().unwrap().id.0 as i64;
    let lang = state.db.get_language().await?;

    if user_id != state.config.owner_id {
        bot.send_message(msg.chat.id, L10n::no_access(lang)).await?;
        return Ok(());
    }

    match Locale::parse(args.trim()) {
        Some(new_lang) => {
            state.db.set_language(new_lang).await?;
            bot.send_message(msg.chat.id, L10n::lang_updated(new_lang))
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, L10n::lang_invalid(lang))
                .await?;
        }
    }
    Ok(())
}

pub async fn handle_add_admin(
    bot: &Bot,
    msg: &Message,
    state: &AppState,
    args: &str,
) -> Result<()> {
    let user_id = msg.from.as_ref().unwrap().id.0 as i64;
    let lang = state.db.get_language().await?;

    if user_id != state.config.owner_id {
        bot.send_message(msg.chat.id, L10n::only_owner_add_admins(lang))
            .await?;
        return Ok(());
    }

    let target_id: i64 = match args.trim().parse() {
        Ok(id) if id > 0 => id,
        _ => {
            bot.send_message(msg.chat.id, L10n::add_admin_usage(lang))
                .await?;
            return Ok(());
        }
    };

    if target_id == state.config.owner_id {
        bot.send_message(msg.chat.id, L10n::already_owner(lang))
            .await?;
        return Ok(());
    }

    if state.db.is_admin(target_id).await? {
        bot.send_message(msg.chat.id, L10n::admin_already_exists(lang, target_id))
            .await?;
        return Ok(());
    }

    let user_name = resolve_user_name(bot, target_id).await;
    state.db.add_admin(target_id, &user_name).await?;

    tracing::info!(admin_id = target_id, added_by = user_id, "Admin added");

    bot.send_message(msg.chat.id, L10n::admin_added(lang, &user_name))
        .await?;

    let _ = bot
        .send_message(ChatId(target_id), L10n::admin_added_notification(lang))
        .await;

    Ok(())
}

pub async fn handle_remove_admin(
    bot: &Bot,
    msg: &Message,
    state: &AppState,
    args: &str,
) -> Result<()> {
    let user_id = msg.from.as_ref().unwrap().id.0 as i64;
    let lang = state.db.get_language().await?;

    if user_id != state.config.owner_id {
        bot.send_message(msg.chat.id, L10n::only_owner_remove_admins(lang))
            .await?;
        return Ok(());
    }

    let target_id: i64 = match args.trim().parse() {
        Ok(id) if id > 0 => id,
        _ => {
            bot.send_message(msg.chat.id, L10n::remove_admin_usage(lang))
                .await?;
            return Ok(());
        }
    };

    if target_id == state.config.owner_id {
        bot.send_message(msg.chat.id, L10n::cannot_remove_owner(lang))
            .await?;
        return Ok(());
    }

    if !state.db.is_admin(target_id).await? {
        bot.send_message(msg.chat.id, L10n::admin_not_found(lang))
            .await?;
        return Ok(());
    }

    state.db.remove_admin(target_id).await?;
    tracing::info!(admin_id = target_id, removed_by = user_id, "Admin removed");

    bot.send_message(msg.chat.id, L10n::admin_removed(lang, target_id))
        .await?;

    let _ = bot
        .send_message(ChatId(target_id), L10n::admin_removed_notification(lang))
        .await;

    Ok(())
}

pub async fn handle_admins(bot: &Bot, msg: &Message, state: &AppState) -> Result<()> {
    let user_id = msg.from.as_ref().unwrap().id.0 as i64;
    let lang = state.db.get_language().await?;

    if user_id != state.config.owner_id {
        bot.send_message(msg.chat.id, L10n::only_owner_list_admins(lang))
            .await?;
        return Ok(());
    }

    let admins = state.db.get_admins().await?;

    if admins.is_empty() {
        bot.send_message(msg.chat.id, L10n::no_admins(lang)).await?;
        return Ok(());
    }

    let mut list = L10n::admins_list_header(lang, state.config.owner_id);
    for (i, admin) in admins.iter().enumerate() {
        if admin.user_id == state.config.owner_id {
            continue;
        }
        list.push_str(&format!(
            "{}. {} (ID: {})\n",
            i + 1,
            admin.user_name,
            admin.user_id
        ));
    }

    bot.send_message(msg.chat.id, list).await?;
    Ok(())
}

pub async fn handle_banned(bot: &Bot, msg: &Message, state: &AppState) -> Result<()> {
    let user_id = msg.from.as_ref().unwrap().id.0 as i64;
    let lang = state.db.get_language().await?;

    if user_id != state.config.owner_id && !state.db.is_admin(user_id).await? {
        bot.send_message(msg.chat.id, L10n::no_access(lang)).await?;
        return Ok(());
    }

    let records = state.db.get_active_ban_records().await?;

    if records.is_empty() {
        bot.send_message(msg.chat.id, L10n::no_active_bans(lang))
            .await?;
        return Ok(());
    }

    let mut list = L10n::active_bans_header(lang).to_string();
    for (i, record) in records.iter().enumerate() {
        list.push_str(&L10n::ban_record_entry(
            lang,
            i + 1,
            &record.ban_id,
            &record.reason,
            &record.created_at.format("%d.%m.%Y %H:%M").to_string(),
        ));
    }

    bot.send_message(msg.chat.id, list).await?;
    Ok(())
}

async fn resolve_user_name(bot: &Bot, user_id: i64) -> String {
    match bot.get_chat(ChatId(user_id)).await {
        Ok(chat) => chat
            .username()
            .or_else(|| chat.first_name())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("user_{user_id}")),
        Err(e) => {
            tracing::warn!(user_id, error = %e, "Failed to resolve user name");
            format!("user_{user_id}")
        }
    }
}
