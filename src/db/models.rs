use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Message {
    pub id: i64,
    pub chat_id: i64,
    pub telegram_message_id: i32,
    pub sender_id: i64,
    pub message_text: String,
    pub media_type: String,
    pub media_file_id: String,
    pub media_group_id: Option<String>,
    pub proposal_group_id: String,
    pub created_at: NaiveDateTime,
    pub status: String,
    pub channel_id: i64,
    pub parent_message_id: Option<i64>,
    pub channel_message_id: Option<i32>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Admin {
    pub id: i64,
    pub user_id: i64,
    pub user_name: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct BanRecord {
    pub id: i64,
    pub ban_id: String,
    pub user_id: i64,
    pub reason: String,
    pub created_at: NaiveDateTime,
    pub active: bool,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserState {
    pub user_id: i64,
    pub state: String,
    pub temp_target_id: i64,
}

/// A proposal groups one or more messages belonging to the same logical submission.
/// For single messages, the group has one item. For media groups, all items share
/// the same `proposal_group_id` (derived from Telegram's `media_group_id`).
#[derive(Debug, Clone)]
pub struct Proposal {
    pub group_id: String,
    pub messages: Vec<Message>,
}

impl Proposal {
    pub fn first(&self) -> &Message {
        &self.messages[0]
    }
}

/// DTO for inserting a new message row.
#[derive(Debug, Clone)]
pub struct NewMessage {
    pub chat_id: i64,
    pub telegram_message_id: i32,
    pub sender_id: i64,
    pub message_text: String,
    pub media_type: String,
    pub media_file_id: String,
    pub media_group_id: Option<String>,
    pub proposal_group_id: String,
    pub channel_id: i64,
    pub parent_message_id: Option<i64>,
}
