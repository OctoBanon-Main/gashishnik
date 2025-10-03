use anyhow::Result;
use tokio::net::TcpStream;
use crate::storage::Storage;
use crate::protocols::common;

pub async fn handle_get_messages(socket: &mut TcpStream, storage: &impl Storage) -> Result<()> {
    let messages = storage.get_messages().await?;

    let text = messages.iter()
        .map(|msg| match (&msg.username, &msg.ip_address) {
            (Some(u), Some(ip)) => format!("[{}] {{{}}} <{}> {}\n", msg.timestamp, ip, u, msg.content),
            (Some(u), None) => format!("[{}] <{}> {}\n", msg.timestamp, u, msg.content),
            (None, Some(ip)) => format!("[{}] {{{}}} (UNAUTHENTICATED) {}\n", msg.timestamp, ip, msg.content),
            (None, None) => format!("[{}] (UNAUTHENTICATED) {}\n", msg.timestamp, msg.content),
        })
        .collect::<String>();

    let bytes = text.as_bytes();
    let size_msg = format!("{}\0", bytes.len());
    common::write_response(socket, size_msg.as_bytes()).await?;

    let response = common::read_message(socket).await?;
    if !response.is_empty() {
        match response[0] {
            0x01 => {
                common::write_response(socket, bytes).await?;
            }
            0x02 => {
                let offset_str = String::from_utf8_lossy(&response[1..]);
                if let Ok(offset) = offset_str.trim_end_matches('\0').parse::<usize>() {
                    if offset < bytes.len() {
                        common::write_response(socket, &bytes[offset..]).await?;
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}