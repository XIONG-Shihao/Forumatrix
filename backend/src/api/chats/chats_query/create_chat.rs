/// Insert chat (ordered pair) if missing, then return the row.
/// - `me_id` and `peer_id` are user ids
/// - `enc_key_wrapped` is a 32-byte per-chat key wrapped with CHAT_MASTER_KEY
use super::types::ChatRow;
use crate::api::auth::auth_handler::utils::now_unix;
use crate::infra::db::Db;

use rand::RngCore;

pub async fn get_or_create_chat(
    db: &Db,
    caller_id: i64,
    peer_id: i64,
    _master_key: &[u8],
) -> Result<ChatRow, sqlx::Error> {
    let (user_lo, user_hi) = if caller_id < peer_id {
        (caller_id, peer_id)
    } else {
        (peer_id, caller_id)
    };

    // Try existing
    if let Some(row) = sqlx::query_as::<_, ChatRow>(
        r#"
        SELECT id, user_lo, user_hi, enc_key, key_nonce, created_at, last_msg_at
        FROM chats
        WHERE user_lo = ? AND user_hi = ?
        "#,
    )
    .bind(user_lo)
    .bind(user_hi)
    .fetch_optional(db)
    .await?
    {
        return Ok(row);
    }

    // Create new (stub-crypto: store random nonce; enc_key = random 32 bytes)
    let mut key_nonce = vec![0u8; 12];
    rand::thread_rng().fill_bytes(&mut key_nonce);

    let mut enc_key = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut enc_key);

    let now = now_unix();

    let res = sqlx::query(
        r#"
        INSERT INTO chats (user_lo, user_hi, enc_key, key_nonce, created_at, last_msg_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(user_lo)
    .bind(user_hi)
    .bind(&enc_key)
    .bind(&key_nonce)
    .bind(now)
    .bind(now)
    .execute(db)
    .await?;

    let id = res.last_insert_rowid();

    let row = sqlx::query_as::<_, ChatRow>(
        r#"
        SELECT id, user_lo, user_hi, enc_key, key_nonce, created_at, last_msg_at
        FROM chats
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(db)
    .await?;

    Ok(row)
}
