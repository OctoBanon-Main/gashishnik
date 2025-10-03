use anyhow::Result;
use sqlx::{sqlite::SqlitePool, FromRow};

#[derive(Debug, FromRow)]
pub struct Message {
    pub timestamp: String,
    pub username: Option<String>,
    pub ip_address: Option<String>,
    pub content: String,
}

pub async fn init_database(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            username TEXT,
            ip_address TEXT,
            content TEXT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}