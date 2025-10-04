use async_trait::async_trait;
use anyhow::Result;
use sqlx::SqlitePool;
use chrono::Local;
use sqlx::Row;

use super::db::Message;

#[async_trait]
pub trait Storage: Send + Sync + 'static {
    async fn save_message(&self, username: Option<&str>, ip: Option<&str>, content: &str) -> Result<()>;
    async fn get_messages(&self) -> Result<Vec<Message>>;
    async fn user_exists(&self, username: &str) -> Result<bool>;
    async fn verify_user(&self, username: &str, password: &str) -> Result<bool>;
    async fn create_user(&self, username: &str, password: &str) -> Result<()>;
}

#[derive(Clone)]
pub struct SqliteStorage {
    pool: SqlitePool,
}

impl SqliteStorage {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn save_message(&self, username: Option<&str>, ip: Option<&str>, content: &str) -> Result<()> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        sqlx::query(
            "INSERT INTO messages (timestamp, username, ip_address, content) VALUES (?, ?, ?, ?)"
        )
        .bind(&timestamp)
        .bind(username)
        .bind(ip)
        .bind(content)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_messages(&self) -> Result<Vec<Message>> {
        let messages = sqlx::query_as::<_, Message>(
            "SELECT id, timestamp, username, ip_address, content FROM messages ORDER BY id"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(messages)
    }

    async fn user_exists(&self, username: &str) -> Result<bool> {
        let exists = sqlx::query("SELECT id FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;
        Ok(exists.is_some())
    }

    async fn verify_user(&self, username: &str, password: &str) -> Result<bool> {
        if let Some(row) = sqlx::query("SELECT password_hash FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?
        {
            let hash: String = row.get("password_hash");
            Ok(bcrypt::verify(password, &hash).unwrap_or(false))
        } else {
            Ok(false)
        }
    }

    async fn create_user(&self, username: &str, password: &str) -> Result<()> {
        let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        sqlx::query(
            "INSERT INTO users (username, password_hash, created_at) VALUES (?, ?, ?)"
        )
        .bind(username)
        .bind(&password_hash)
        .bind(&timestamp)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}