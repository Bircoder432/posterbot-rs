use posterbot::db::Database;
use posterbot::db::models::{NewMessage, UserState};
use tempfile::TempDir;

async fn setup_db() -> (Database, TempDir) {
    let dir = TempDir::new().unwrap();
    let db_path = dir.path().join("test.db");
    let db_path_str = db_path.to_str().unwrap();
    let db = Database::connect(db_path_str).await.unwrap();
    db.migrate().await.unwrap();
    (db, dir)
}

#[tokio::test]
async fn test_save_message_prevents_duplicates() {
    let (db, _dir) = setup_db().await;

    let msg = NewMessage {
        chat_id: 1,
        telegram_message_id: 100,
        sender_id: 50,
        message_text: "Test".into(),
        media_type: "text".into(),
        media_file_id: "".into(),
        media_group_id: None,
        proposal_group_id: "g1".into(),
        channel_id: 2,
        parent_message_id: None,
    };

    let inserted_first = db.save_message(&msg).await.unwrap();
    assert!(inserted_first);

    let inserted_second = db.save_message(&msg).await.unwrap();
    assert!(!inserted_second);
}

#[tokio::test]
async fn test_get_next_pending_proposal_groups_items() {
    let (db, _dir) = setup_db().await;

    // Simulate a media group (2 items with same proposal_group_id)
    let msg1 = NewMessage {
        chat_id: 1,
        telegram_message_id: 101,
        sender_id: 50,
        message_text: "Photo 1".into(),
        media_type: "photo".into(),
        media_file_id: "fid1".into(),
        media_group_id: Some("tg_group_1".into()),
        proposal_group_id: "tg_group_1".into(),
        channel_id: 2,
        parent_message_id: None,
    };
    let msg2 = NewMessage {
        chat_id: 1,
        telegram_message_id: 102,
        sender_id: 50,
        message_text: "Photo 2".into(),
        media_type: "photo".into(),
        media_file_id: "fid2".into(),
        media_group_id: Some("tg_group_1".into()),
        proposal_group_id: "tg_group_1".into(),
        channel_id: 2,
        parent_message_id: None,
    };
    // Simulate a separate single proposal
    let msg3 = NewMessage {
        chat_id: 1,
        telegram_message_id: 103,
        sender_id: 50,
        message_text: "Text".into(),
        media_type: "text".into(),
        media_file_id: "".into(),
        media_group_id: None,
        proposal_group_id: "single_1".into(),
        channel_id: 2,
        parent_message_id: None,
    };

    db.save_message(&msg1).await.unwrap();
    db.save_message(&msg2).await.unwrap();
    db.save_message(&msg3).await.unwrap();

    let proposal = db.get_next_pending_proposal().await.unwrap().unwrap();

    assert_eq!(proposal.messages.len(), 2);
    assert_eq!(proposal.group_id, "tg_group_1");
}

#[tokio::test]
async fn test_user_state_upsert() {
    let (db, _dir) = setup_db().await;

    db.set_user_state(99, "reason", 100).await.unwrap();

    let state: UserState = db.get_user_state(99).await.unwrap().unwrap();
    assert_eq!(state.state, "reason");
    assert_eq!(state.temp_target_id, 100);

    // Update existing state
    db.set_user_state(99, "ban_reason", 200).await.unwrap();

    let state_updated: UserState = db.get_user_state(99).await.unwrap().unwrap();
    assert_eq!(state_updated.state, "ban_reason");
    assert_eq!(state_updated.temp_target_id, 200);

    db.clear_user_state(99).await.unwrap();
    let state_cleared: UserState = db.get_user_state(99).await.unwrap().unwrap();
    assert_eq!(state_cleared.state, "none");
    assert_eq!(state_cleared.temp_target_id, 0);
}

#[tokio::test]
async fn test_ban_flow() {
    let (db, _dir) = setup_db().await;

    let ban_id = db.create_ban_record(123, "Spam").await.unwrap();

    assert!(db.is_banned(123).await.unwrap() == false); // Not banned yet, only record created

    db.ban_user(123).await.unwrap();
    assert!(db.is_banned(123).await.unwrap());

    let record = db.get_ban_record(&ban_id).await.unwrap().unwrap();
    assert_eq!(record.user_id, 123);
    assert_eq!(record.reason, "Spam");

    db.deactivate_ban(&ban_id).await.unwrap();
    assert!(db.get_ban_record(&ban_id).await.unwrap().is_none());
}
