use anyhow::Result;
use crate::server::storage::Storage;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};
use futures_util::SinkExt;
use crate::server::io_stream::AsyncStream;

type BoxedStream = Box<dyn AsyncStream + Send + Sync>;

pub async fn handle_get_messages(
    ws: &mut WebSocketStream<BoxedStream>,
    storage: &impl Storage,
    packet: &[u8],
) -> Result<()> {
    let messages = storage.get_messages().await?;
    let text = messages
        .iter()
        .map(|msg| match (&msg.username, &msg.ip_address) {
            (Some(u), Some(ip)) => format!("[{}] {{{}}} <{}> {}\n", msg.timestamp, ip, u, msg.content),
            (Some(u), None)     => format!("[{}] <{}> {}\n", msg.timestamp, u, msg.content),
            (None, Some(ip))    => format!("[{}] {{{}}} (UNAUTHENTICATED) {}\n", msg.timestamp, ip, msg.content),
            (None, None)        => format!("[{}] (UNAUTHENTICATED) {}\n", msg.timestamp, msg.content),
        })
        .collect::<String>();

    let bytes = text.as_bytes();
    let total_size = bytes.len();

    if packet.len() == 1 && packet[0] == 0x00 {
        let len_str = total_size.to_string();
        ws.send(Message::Binary(len_str.into_bytes().into())).await?;
        return Ok(());
    }

    if packet.len() >= 2 && packet[0] == 0x00 && packet[1] == 0x01 {
        ws.send(Message::Binary(bytes.to_vec().into())).await?;
        return Ok(());
    }

    if packet.len() >= 2 && packet[0] == 0x00 && packet[1] == 0x02 {
        let offset_slice = &packet[2..];
        let offset_str = match std::str::from_utf8(offset_slice) {
            Ok(s) => s.trim_matches(char::from(0)).trim(),
            Err(_) => return Ok(()),
        };
        if offset_str.is_empty() {
            return Ok(());
        }

        if let Ok(last_size) = offset_str.parse::<usize>() {
            if last_size < total_size {
                let slice = &bytes[last_size..];
                ws.send(Message::Binary(slice.to_vec().into())).await?;
            }
        }
        return Ok(());
    }

    Ok(())
}