use anyhow::Result;
use tokio::net::TcpListener;
use crate::protocols::common::info::build_protocol_info_packet;
use crate::{server::storage::Storage, protocols::commands::Command};
use tracing::{info, warn};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

use std::boxed::Box;
use tokio_rustls::TlsAcceptor;

use crate::server::io_stream::AsyncStream;
type BoxedStream = Box<dyn AsyncStream + Send + Sync>;

use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use futures_util::{StreamExt, SinkExt};

use super::handlers;

pub async fn run_server(
    bind_addr: &str,
    storage: impl Storage,
    auth_only: bool,
    tls_acceptor: Option<Arc<TlsAcceptor>>,
    shutdown: CancellationToken,
) -> Result<()> {
    let storage = Arc::new(storage);
    let listener = TcpListener::bind(bind_addr).await?;
    info!("WRAC server bound to {}, auth_only={}", bind_addr, auth_only);

    loop {
        tokio::select! {
            conn = listener.accept() => {
                match conn {
                    Ok((socket, addr)) => {
                        let storage = storage.clone();
                        let ip = addr.ip().to_string();
                        info!("New client connected: {}", ip);

                        let acceptor = tls_acceptor.clone();
                        tokio::spawn(async move {
                            let boxed: BoxedStream = match acceptor {
                                Some(acceptor) => {
                                    match acceptor.accept(socket).await {
                                        Ok(tls_stream) => Box::new(tls_stream),
                                        Err(e) => { warn!("TLS handshake failed for {}: {:?}", ip, e); return; }
                                    }
                                }
                                None => Box::new(socket),
                            };

                            let ws = match accept_async(boxed).await {
                                Ok(ws) => ws,
                                Err(e) => { warn!("WebSocket handshake failed for {}: {:?}", ip, e); return; }
                            };

                            if let Err(e) = handle_ws_client(ws, storage, ip.clone(), auth_only).await {
                                warn!("Client {} disconnected with error: {:?}", ip, e);
                            } else {
                                info!("Client {} disconnected", ip);
                            }
                        });
                    }
                    Err(e) => warn!("Accept error: {:?}", e),
                }
            }
            _ = shutdown.cancelled() => {
                info!("Shutdown signal received, stopping WRAC server loop...");
                break;
            }
        }
    }
    Ok(())
}

async fn handle_ws_client(
    mut ws: WebSocketStream<BoxedStream>,
    storage: Arc<impl Storage>,
    client_ip: String,
    auth_only: bool,
) -> Result<()> {
    while let Some(incoming) = ws.next().await {
    let msg = match incoming {
        Ok(Message::Binary(b)) => b,
        Ok(Message::Text(_)) => {
            warn!("{} sent text while binary expected", client_ip);
            continue;
        }
        Ok(Message::Close(_)) => break,
        Ok(Message::Ping(p)) => {
            ws.send(Message::Pong(p)).await?;
            continue;
        }
        Ok(Message::Pong(_)) => continue,
        Ok(Message::Frame(_)) => continue,
        Err(e) => {
            warn!("recv error from {}: {:?}", client_ip, e);
            break;
        }
    };

        match Command::try_from(msg[0]) {
            Ok(Command::GetMessages) => {
                handlers::handle_get_messages(&mut ws, &*storage, &msg).await?;
            }
            Ok(Command::SendUnauthenticated) => {
                if auth_only {
                    warn!("Rejected unauthenticated message from {} (auth_only)", client_ip);
                    ws.send(Message::Binary(vec![0xFF].into())).await?;
                } else {
                    handlers::handle_unauth_message(&mut ws, &*storage, &msg[1..], &client_ip).await?;
                }
            }
            Ok(Command::SendAuthenticated) => {
                handlers::handle_auth_message(&mut ws, &*storage, &msg[1..], &client_ip).await?;
            }
            Ok(Command::Register) => {
                handlers::handle_registration(&mut ws, &*storage, &msg[1..]).await?;
            }
            Ok(Command::Info) => {
                let packet = build_protocol_info_packet(0x03);
                ws.send(Message::Binary(packet.into())).await?;
            }
            Err(_) => warn!("Unknown command from {}: 0x{:02x}", client_ip, msg[0]),
        }
    }
    Ok(())
}