use anyhow::{Context, Result};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    time::Duration,
};
use tracing::{info, warn};

pub type Db = SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
}

fn path_from_dsn(database_url: &str) -> Option<PathBuf> {
    // sqlite:///abs/path.db or sqlite://rel/path.db
    if let Some(rest) = database_url.strip_prefix("sqlite://") {
        // "rest" is the path segment
        return Some(PathBuf::from(rest));
    }
    // file:/abs/path.db?... (SQLite's URI form)
    if let Some(rest) = database_url.strip_prefix("file:") {
        let without_query = rest.split('?').next().unwrap_or(rest);
        return Some(PathBuf::from(without_query));
    }
    None
}

fn ensure_parent_dir(p: &Path) -> Result<()> {
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("creating sqlite parent dir: {}", parent.display()))?;
    }
    Ok(())
}

fn temp_write_test(dir: &Path) -> Result<()> {
    let test_path = dir.join(".perm_test");
    let mut f = fs::File::create(&test_path)
        .with_context(|| format!("creating temp file: {}", test_path.display()))?;
    f.write_all(b"ok")?;
    drop(f);
    fs::remove_file(&test_path).ok();
    Ok(())
}

pub async fn init_state() -> Result<AppState> {
    dotenvy::dotenv().ok();

    let raw_dsn =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://./data/app.db".to_string());
    info!("DATABASE_URL(raw) = {}", raw_dsn);

    // Derive a concrete filesystem path (for directory checks & temp write)
    let maybe_path = path_from_dsn(&raw_dsn);
    if let Some(p) = &maybe_path {
        info!("Derived sqlite path: {}", p.display());
        ensure_parent_dir(p)?;
        if let Some(parent) = p.parent() {
            temp_write_test(parent)
                .with_context(|| format!("write-test in dir: {}", parent.display()))?;
        }
    } else {
        warn!("Could not derive a filesystem path from DSN; skipping dir checks");
    }

    // Prefer file: URI with explicit mode=rwc to force create if missing.
    // If caller already used file:, keep it; otherwise convert sqlite:// to file:
    let dsn = if raw_dsn.starts_with("file:") {
        // ensure has mode=rwc
        if raw_dsn.contains("mode=") {
            raw_dsn
        } else {
            format!(
                "{}{}mode=rwc",
                raw_dsn,
                if raw_dsn.contains('?') { '&' } else { '?' }
            )
        }
    } else if let Some(p) = maybe_path {
        format!("file:{}?mode=rwc&cache=shared", p.display())
    } else {
        // Fallback: use raw
        raw_dsn
    };
    info!("DATABASE_URL(effective) = {}", dsn);

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&dsn)
        .await
        .with_context(|| format!("connecting to {}", dsn))?;

    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("migrations applied");

    crate::infra::seed::seed_admin_if_requested(&pool).await?;
    Ok(AppState { db: pool })
}
