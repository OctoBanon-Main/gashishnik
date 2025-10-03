use anyhow::Result;
use crate::io_stream::AsyncStream;
use crate::storage::Storage;
use tracing::info;
use crate::protocols::common;

pub async fn handle_unauth_message(
    socket: &mut (dyn AsyncStream),
    storage: &impl Storage,
    data: &[u8],
    client_ip: &str,
) -> Result<()> {
    let msg = String::from_utf8_lossy(data).trim_end_matches('\0').to_string();
    info!("Saving unauthenticated message from {}: {}", client_ip, msg);

    storage.save_message(None, Some(client_ip), &msg).await?;
    common::write_response(socket, &[0x00]).await?;

    Ok(())
}