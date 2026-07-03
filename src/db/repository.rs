use anyhow::Result;
use chrono::Utc;
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

use super::models::{Admin, BanRecord, Message, NewMessage, Proposal, UserState};
use crate::locales::Locale;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn connect(db_path: &str) -> Result<Self> {
        let options = SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    // ── Settings / Language ───────────────────────────────────

    pub async fn get_language(&self) -> Result<Locale> {
        let row: Option<(String,)> =
            sqlx::query_as("SELECT value FROM settings WHERE key = 'language'")
                .fetch_optional(&self.pool)
                .await?;
        Ok(row.and_then(|(v,)| Locale::parse(&v)).unwrap_or(Locale::En))
    }

    pub async fn set_language(&self, lang: Locale) -> Result<()> {
        sqlx::query(
            "INSERT INTO settings (key, value) VALUES ('language', ?)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        )
        .bind(lang.as_str())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // ── Messages ──────────────────────────────────────────────

    pub async fn save_message(&self, msg: &NewMessage) -> Result<bool> {
        let now = Utc::now().naive_utc();
        let result = sqlx::query(
            "INSERT OR IGNORE INTO messages
             (chat_id, telegram_message_id, sender_id, message_text,
              media_type, media_file_id, media_group_id, proposal_group_id,
              created_at, status, channel_id, parent_message_id)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'pending', ?, ?)",
        )
        .bind(msg.chat_id)
        .bind(msg.telegram_message_id)
        .bind(msg.sender_id)
        .bind(&msg.message_text)
        .bind(&msg.media_type)
        .bind(&msg.media_file_id)
        .bind(&msg.media_group_id)
        .bind(&msg.proposal_group_id)
        .bind(now)
        .bind(msg.channel_id)
        .bind(msg.parent_message_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn message_exists(&self, chat_id: i64, telegram_message_id: i32) -> Result<bool> {
        let row: Option<(i64,)> =
            sqlx::query_as("SELECT 1 FROM messages WHERE chat_id = ? AND telegram_message_id = ?")
                .bind(chat_id)
                .bind(telegram_message_id)
                .fetch_optional(&self.pool)
                .await?;
        Ok(row.is_some())
    }

    pub async fn get_message_by_id(&self, id: i64) -> Result<Option<Message>> {
        let msg: Option<Message> = sqlx::query_as("SELECT * FROM messages WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(msg)
    }

    pub async fn get_next_pending_proposal(&self) -> Result<Option<Proposal>> {
        let group: Option<(String,)> = sqlx::query_as(
            "SELECT proposal_group_id FROM messages
             WHERE status = 'pending'
             GROUP BY proposal_group_id
             ORDER BY MIN(created_at) ASC
             LIMIT 1",
        )
        .fetch_optional(&self.pool)
        .await?;

        match group {
            Some((gid,)) => self.get_proposal_by_group_id(&gid).await,
            None => Ok(None),
        }
    }

    pub async fn get_proposal_by_group_id(&self, group_id: &str) -> Result<Option<Proposal>> {
        let messages: Vec<Message> =
            sqlx::query_as("SELECT * FROM messages WHERE proposal_group_id = ? ORDER BY id ASC")
                .bind(group_id)
                .fetch_all(&self.pool)
                .await?;

        if messages.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Proposal {
                group_id: group_id.to_string(),
                messages,
            }))
        }
    }

    pub async fn update_proposal_status(&self, group_id: &str, status: &str) -> Result<()> {
        sqlx::query("UPDATE messages SET status = ? WHERE proposal_group_id = ?")
            .bind(status)
            .bind(group_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_proposal(&self, group_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM messages WHERE proposal_group_id = ?")
            .bind(group_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_channel_message_id(
        &self,
        group_id: &str,
        channel_message_id: i32,
    ) -> Result<()> {
        sqlx::query("UPDATE messages SET channel_message_id = ? WHERE proposal_group_id = ?")
            .bind(channel_message_id)
            .bind(group_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // ── Admins ────────────────────────────────────────────────

    pub async fn is_admin(&self, user_id: i64) -> Result<bool> {
        let row: Option<(i64,)> = sqlx::query_as("SELECT 1 FROM admins WHERE user_id = ?")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.is_some())
    }

    pub async fn add_admin(&self, user_id: i64, user_name: &str) -> Result<()> {
        sqlx::query("INSERT OR IGNORE INTO admins (user_id, user_name) VALUES (?, ?)")
            .bind(user_id)
            .bind(user_name)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn remove_admin(&self, user_id: i64) -> Result<()> {
        sqlx::query("DELETE FROM admins WHERE user_id = ?")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_admins(&self) -> Result<Vec<Admin>> {
        let admins: Vec<Admin> = sqlx::query_as("SELECT * FROM admins ORDER BY id ASC")
            .fetch_all(&self.pool)
            .await?;
        Ok(admins)
    }

    pub async fn ensure_admin(&self, user_id: i64, user_name: &str) -> Result<()> {
        self.add_admin(user_id, user_name).await
    }

    // ── Bans ──────────────────────────────────────────────────

    pub async fn ban_user(&self, user_id: i64) -> Result<()> {
        sqlx::query("INSERT OR IGNORE INTO banned (user_id) VALUES (?)")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn is_banned(&self, user_id: i64) -> Result<bool> {
        let row: Option<(i64,)> = sqlx::query_as("SELECT 1 FROM banned WHERE user_id = ?")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.is_some())
    }

    pub async fn pardon_user(&self, user_id: i64) -> Result<()> {
        sqlx::query("DELETE FROM banned WHERE user_id = ?")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn create_ban_record(&self, user_id: i64, reason: &str) -> Result<String> {
        let ban_id = generate_ban_id();
        let now = Utc::now().naive_utc();
        sqlx::query(
            "INSERT INTO ban_records (ban_id, user_id, reason, created_at, active)
             VALUES (?, ?, ?, ?, 1)",
        )
        .bind(&ban_id)
        .bind(user_id)
        .bind(reason)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(ban_id)
    }

    pub async fn get_ban_record(&self, ban_id: &str) -> Result<Option<BanRecord>> {
        let record: Option<BanRecord> =
            sqlx::query_as("SELECT * FROM ban_records WHERE ban_id = ? AND active = 1")
                .bind(ban_id)
                .fetch_optional(&self.pool)
                .await?;
        Ok(record)
    }

    pub async fn get_active_ban_records(&self) -> Result<Vec<BanRecord>> {
        let records: Vec<BanRecord> =
            sqlx::query_as("SELECT * FROM ban_records WHERE active = 1 ORDER BY created_at DESC")
                .fetch_all(&self.pool)
                .await?;
        Ok(records)
    }

    pub async fn deactivate_ban(&self, ban_id: &str) -> Result<()> {
        sqlx::query("UPDATE ban_records SET active = 0 WHERE ban_id = ?")
            .bind(ban_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // ── User State ────────────────────────────────────────────

    pub async fn set_user_state(&self, user_id: i64, state: &str, target_id: i64) -> Result<()> {
        sqlx::query(
            "INSERT INTO user_states (user_id, state, temp_target_id)
             VALUES (?, ?, ?)
             ON CONFLICT(user_id) DO UPDATE SET state = excluded.state, temp_target_id = excluded.temp_target_id",
        )
        .bind(user_id)
        .bind(state)
        .bind(target_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_user_state(&self, user_id: i64) -> Result<Option<UserState>> {
        let state: Option<UserState> =
            sqlx::query_as("SELECT * FROM user_states WHERE user_id = ?")
                .bind(user_id)
                .fetch_optional(&self.pool)
                .await?;
        Ok(state)
    }

    pub async fn clear_user_state(&self, user_id: i64) -> Result<()> {
        sqlx::query("UPDATE user_states SET state = 'none', temp_target_id = 0 WHERE user_id = ?")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

fn generate_ban_id() -> String {
    let id = uuid::Uuid::new_v4().simple().to_string().to_uppercase();
    format!("BAN-{}", &id[..6])
}
