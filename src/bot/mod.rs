pub mod admin;
pub mod media;
pub mod moderation;
pub mod proposals;

use std::sync::Arc;

use teloxide::dispatching::UpdateFilterExt;
use teloxide::prelude::*;

use crate::config::Config;
use crate::db::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub config: Arc<Config>,
}

pub async fn run(config: Config, db: Database) -> anyhow::Result<()> {
    let bot = Bot::new(&config.bot_token);
    let state = AppState {
        db,
        config: Arc::new(config),
    };

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(handle_message))
        .branch(Update::filter_callback_query().endpoint(handle_callback));

    let mut dispatcher = Dispatcher::builder(bot.clone(), handler)
        .dependencies(dptree::deps![state])
        .enable_ctrlc_handler()
        .build();

    dispatcher.dispatch().await;
    Ok(())
}

type R = anyhow::Result<()>;
type ResponseResult = std::result::Result<(), teloxide::RequestError>;

async fn handle_message(bot: Bot, msg: Message, state: AppState) -> ResponseResult {
    if let Err(e) = dispatch_message(&bot, &msg, &state).await {
        tracing::error!(error = %e, "Message handler error");
    }
    Ok(())
}

async fn handle_callback(bot: Bot, q: CallbackQuery, state: AppState) -> ResponseResult {
    if let Err(e) = moderation::handle_callback(&bot, &q, &state).await {
        tracing::error!(error = %e, "Callback handler error");
    }
    Ok(())
}

async fn dispatch_message(bot: &Bot, msg: &Message, state: &AppState) -> R {
    let Some(from) = &msg.from else {
        return Ok(());
    };
    let user_id = from.id.0 as i64;

    if msg.text().is_some_and(|t| t.starts_with('/')) {
        return dispatch_command(bot, msg, state).await;
    }

    if let Some(us) = state.db.get_user_state(user_id).await? {
        match us.state.as_str() {
            "reply_mode" => {
                return proposals::handle_reply_content(bot, msg, state, us.temp_target_id).await;
            }
            "reason" => {
                return proposals::handle_send_reason(bot, msg, state, us.temp_target_id).await;
            }
            "ban_reason" => {
                return proposals::handle_send_ban_reason(bot, msg, state, us.temp_target_id).await;
            }
            _ => {}
        }
    }

    proposals::handle_proposal(bot, msg, state).await
}

async fn dispatch_command(bot: &Bot, msg: &Message, state: &AppState) -> R {
    let text = msg.text().unwrap_or("");
    let mut parts = text.splitn(2, char::is_whitespace);
    let cmd = parts
        .next()
        .unwrap_or("")
        .trim_start_matches('/')
        .to_lowercase();
    let args = parts.next().unwrap_or("").trim();

    tracing::info!(user_id = msg.from.as_ref().map(|u| u.id.0), cmd = %cmd, "Command received");

    match cmd.as_str() {
        "start" => proposals::handle_start(bot, msg, state, args).await,
        "proposals" => moderation::handle_proposals(bot, msg, state).await,
        "addadmin" => admin::handle_add_admin(bot, msg, state, args).await,
        "removeadmin" => admin::handle_remove_admin(bot, msg, state, args).await,
        "admins" => admin::handle_admins(bot, msg, state).await,
        "banned" => admin::handle_banned(bot, msg, state).await,
        "pardon" => moderation::handle_pardon(bot, msg, state, args).await,
        "reply" => proposals::handle_reply_command(bot, msg, state, args).await,
        "lang" => admin::handle_set_language(bot, msg, state, args).await,
        _ => Ok(()),
    }
}
