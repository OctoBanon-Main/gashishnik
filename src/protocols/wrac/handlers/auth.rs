use anyhow::Result;
use crate::protocols::common::sanitize::sanitize_message;
use crate::server::storage::Storage;
use tracing::{info, warn};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use futures_util::SinkExt;
use crate::server::io_stream::AsyncStream;
type BoxedStream = Box<dyn AsyncStream + Send + Sync>;

pub async fn handle_auth_message(
    ws: &mut WebSocketStream<BoxedStream>,
    storage: &impl Storage,
    data: &[u8],
    client_ip: &str,
) -> Result<()> {
    let msg_str = String::from_utf8_lossy(data);
    let parts: Vec<&str> = msg_str.trim_end_matches('\0').split('\n').collect();

    if parts.len() < 3 {
        warn!("Not enough data for authenticated message from {}", client_ip);
        return Ok(());
    }

    let (username, password, message_raw) = (parts[0], parts[1], parts[2]);

    if !storage.user_exists(username).await? {
        warn!("Auth failed: user {} not found", username);
        ws.send(Message::Binary(vec![0x01].into())).await?;
        return Ok(());
    }

    if !storage.verify_user(username, password).await? {
        warn!("Auth failed: wrong password for user {}", username);
        ws.send(Message::Binary(vec![0x02].into())).await?;
        return Ok(());
    }

    let message_clean = sanitize_message(message_raw);

    info!("User {} authenticated successfully, saving message", message_clean);
    storage.save_message(Some(username), Some(client_ip), &message_clean).await?;
    ws.send(Message::Binary(vec![0x00].into())).await?;

    Ok(())
}

pub async fn handle_registration(
    ws: &mut WebSocketStream<BoxedStream>,
    storage: &impl Storage,
    data: &[u8]
) -> Result<()> {
    let msg_str = String::from_utf8_lossy(data);
    let parts: Vec<&str> = msg_str.trim_end_matches('\0').split('\n').collect();

    if parts.len() < 2 {
        warn!("Not enough data for registration");
        return Ok(());
    }

    let (username, password) = (parts[0], parts[1]);

    if storage.user_exists(username).await? {
        warn!("Registration failed: user {} already exists", username);
        ws.send(Message::Binary(vec![0x01].into())).await?;
        return Ok(());
    }

    storage.create_user(username, password).await?;
    info!("User {} registered successfully", username);
    ws.send(Message::Binary(vec![0x00].into())).await?;

    Ok(())
}
