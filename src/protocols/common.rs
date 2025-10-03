use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::io_stream::AsyncStream;

pub const BUFFER_SIZE: usize = 1024;
pub const TIMEOUT_SECS: u64 = 10;

pub async fn read_message(socket: &mut (dyn AsyncStream)) -> Result<Vec<u8>> {
    let mut buffer = vec![0u8; BUFFER_SIZE];
    let n = tokio::time::timeout(
        std::time::Duration::from_secs(TIMEOUT_SECS),
        socket.read(&mut buffer),
    ).await??;

    if n == 0 {
        anyhow::bail!("Connection closed");
    }

    Ok(buffer[..n].to_vec())
}

pub async fn write_response(socket: &mut (dyn AsyncStream), data: &[u8]) -> Result<()> {
    socket.write_all(data).await?;
    socket.flush().await?;
    Ok(())
}