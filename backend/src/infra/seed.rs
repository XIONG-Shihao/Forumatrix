// src/infra/seed.rs
use anyhow::{anyhow, Result};
use sqlx::Row;

use crate::infra::db::Db;

// Hashing via your existing helper
use crate::api::auth::password::hash_password;

/// If DEV_SEED_ADMIN is set (1/true), ensure there is at least one admin user.
/// - If ADMIN_EMAIL exists, promote it to admin.
/// - Else create a new admin user with the given creds.
/// Safe for repeated runs.
pub async fn seed_admin_if_requested(db: &Db) -> Result<()> {
    let seed = std::env::var("DEV_SEED_ADMIN").unwrap_or_default();
    if seed != "1" && seed.to_lowercase() != "true" {
        return Ok(());
    }

    // Already have an admin? Done.
    if let Some(row) = sqlx::query("SELECT id FROM users WHERE is_admin = 1 LIMIT 1")
        .fetch_optional(db)
        .await?
    {
        let id: i64 = row.get("id");
        tracing::info!("DEV_SEED_ADMIN: admin already present (id={})", id);
        return Ok(());
    }

    let email = std::env::var("ADMIN_EMAIL").unwrap_or_else(|_| "admin@example.com".to_string());
    let username = std::env::var("ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let password = std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "AdminPass123!".to_string());

    // ⬇️ Map argon2's password_hash::Error into anyhow::Error explicitly
    let hash = hash_password(&password).map_err(|e| anyhow!("hash_password failed: {e}"))?;

    // If user exists by email, promote to admin
    if let Some(row) = sqlx::query("SELECT id FROM users WHERE email = ? LIMIT 1")
        .bind(&email)
        .fetch_optional(db)
        .await?
    {
        let id: i64 = row.get("id");
        sqlx::query("UPDATE users SET is_admin = 1 WHERE id = ?")
            .bind(id)
            .execute(db)
            .await?;
        tracing::info!(
            "DEV_SEED_ADMIN: promoted existing user {} (id={}) to admin",
            email,
            id
        );
        return Ok(());
    }

    // Else create a new admin user
    let res = sqlx::query(
        r#"
        INSERT INTO users (email, password_hash, username, is_active, is_admin)
        VALUES (?, ?, ?, 1, 1)
        "#,
    )
    .bind(&email)
    .bind(&hash)
    .bind(&username)
    .execute(db)
    .await?;

    let id = res.last_insert_rowid();
    tracing::info!("DEV_SEED_ADMIN: created admin user {} (id={})", email, id);
    Ok(())
}
