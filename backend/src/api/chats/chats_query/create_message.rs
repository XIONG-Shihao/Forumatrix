use super::types::MessageRow;
use crate::infra::db::Db;

use crate::api::auth::auth_handler::utils::now_unix;
use rand::RngCore;
/// Insert an encrypted message (caller ensures membership).
/// Returns the stored row.
pub async fn insert_message(
    db: &Db,
    chat_id: i64,
    sender_id: i64,
    plaintext: &[u8],
    _master_key: &[u8],
) -> Result<MessageRow, sqlx::Error> {
    let mut nonce = vec![0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);

    // STUB: ciphertext = plaintext (swap for AEAD later)
    let ciphertext = plaintext.to_vec();
    let now = now_unix();

    let res = sqlx::query(
        r#"
        INSERT INTO chat_messages (chat_id, sender_id, nonce, ciphertext, created_at)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(chat_id)
    .bind(sender_id)
    .bind(&nonce)
    .bind(&ciphertext)
    .bind(now)
    .execute(db)
    .await?;

    // bump last_msg_at
    sqlx::query("UPDATE chats SET last_msg_at = ? WHERE id = ?")
        .bind(now)
        .bind(chat_id)
        .execute(db)
        .await?;

    let id = res.last_insert_rowid();
    sqlx::query_as::<_, MessageRow>(
        r#"
        SELECT id, chat_id, sender_id, nonce, ciphertext, created_at, read_at
        FROM chat_messages
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(db)
    .await
}
