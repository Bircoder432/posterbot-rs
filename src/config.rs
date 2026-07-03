use anyhow::{Context, Result};
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub bot_token: String,
    pub owner_id: i64,
    pub owner_name: String,
    pub channel_id: i64,
    pub bot_username: String,
    pub db_path: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            bot_token: env::var("TELEGRAM_BOT_TOKEN").context("TELEGRAM_BOT_TOKEN not set")?,
            owner_id: env::var("OWNER_ID")
                .context("OWNER_ID not set")?
                .parse()
                .context("OWNER_ID must be a number")?,
            owner_name: env::var("OWNER_NAME").unwrap_or_else(|_| "Owner".into()),
            channel_id: env::var("CHANNEL_ID")
                .context("CHANNEL_ID not set")?
                .parse()
                .context("CHANNEL_ID must be a number")?,
            bot_username: env::var("BOT_USERNAME").context("BOT_USERNAME not set")?,
            db_path: env::var("DB_PATH").unwrap_or_else(|_| "data/bot.db".into()),
        })
    }
}
