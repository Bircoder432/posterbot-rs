use anyhow::Context;
use posterbot::{bot, config::Config, db::Database};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config = Config::from_env().context("Failed to load configuration")?;

    if let Some(parent) = std::path::Path::new(&config.db_path).parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let db = Database::connect(&config.db_path)
        .await
        .context("Failed to connect to database")?;
    db.migrate().await.context("Failed to run migrations")?;
    db.ensure_admin(config.owner_id, &config.owner_name).await?;

    tracing::info!(bot = %config.bot_username, channel = config.channel_id, "Starting bot");
    bot::run(config, db).await
}
