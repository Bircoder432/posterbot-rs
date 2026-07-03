CREATE TABLE IF NOT EXISTS messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    chat_id INTEGER NOT NULL,
    telegram_message_id INTEGER NOT NULL,
    sender_id INTEGER NOT NULL,
    message_text TEXT NOT NULL DEFAULT '',
    media_type TEXT NOT NULL DEFAULT 'text',
    media_file_id TEXT NOT NULL DEFAULT '',
    media_group_id TEXT,
    proposal_group_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    channel_id INTEGER NOT NULL DEFAULT 0,
    parent_message_id INTEGER,
    channel_message_id INTEGER,
    UNIQUE(chat_id, telegram_message_id)
);

CREATE INDEX IF NOT EXISTS idx_messages_status ON messages(status);
CREATE INDEX IF NOT EXISTS idx_messages_group ON messages(proposal_group_id);
CREATE INDEX IF NOT EXISTS idx_messages_sender ON messages(sender_id);

CREATE TABLE IF NOT EXISTS admins (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL UNIQUE,
    user_name TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS banned (
    user_id INTEGER PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS ban_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ban_id TEXT NOT NULL UNIQUE,
    user_id INTEGER NOT NULL,
    reason TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL,
    active INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS user_states (
    user_id INTEGER PRIMARY KEY,
    state TEXT NOT NULL DEFAULT 'none',
    temp_target_id INTEGER NOT NULL DEFAULT 0
);
