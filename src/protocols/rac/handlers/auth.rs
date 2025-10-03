use anyhow::Result;
use tokio::net::TcpStream;
use crate::storage::Storage;
use tracing::{info, warn};
use crate::protocols::common;

pub async fn handle_auth_message(
    socket: &mut TcpStream,
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

    let (username, password, message) = (parts[0], parts[1], parts[2]);

    if !storage.user_exists(username).await? {
        warn!("Auth failed: user {} not found", username);
        common::write_response(socket, &[0x01]).await?;
        return Ok(());
    }

    if !storage.verify_user(username, password).await? {
        warn!("Auth failed: wrong password for user {}", username);
        common::write_response(socket, &[0x02]).await?;
        return Ok(());
    }

    info!("User {} authenticated successfully, saving message", username);
    storage.save_message(Some(username), Some(client_ip), message).await?;
    common::write_response(socket, &[0x00]).await?;
    
    Ok(())
}

pub async fn handle_registration(
    socket: &mut TcpStream, 
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
        common::write_response(socket, &[0x01]).await?;
        return Ok(());
    }

    storage.create_user(username, password).await?;
    info!("User {} registered successfully", username);
    common::write_response(socket, &[0x00]).await?;
    
    Ok(())
}