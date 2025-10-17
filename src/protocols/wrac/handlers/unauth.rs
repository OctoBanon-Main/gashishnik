use anyhow::Result;
use crate::protocols::common::sanitize::sanitize_message;
use crate::server::storage::Storage;
use tracing::info;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use futures_util::SinkExt;
use crate::server::io_stream::AsyncStream;
type BoxedStream = Box<dyn AsyncStream + Send + Sync>;

pub async fn handle_unauth_message(
    ws: &mut WebSocketStream<BoxedStream>,
    storage: &impl Storage,
    data: &[u8],
    client_ip: &str,
) -> Result<()> {
    let msg_raw = String::from_utf8_lossy(data).trim_end_matches('\0').to_string();
    let msg_clean = sanitize_message(&msg_raw);

    info!("Saving unauthenticated message from {}: {}", client_ip, msg_clean);

    storage.save_message(None, Some(client_ip), &msg_clean).await?;
    ws.send(Message::Binary(vec![0x00].into())).await?;

    Ok(())
}