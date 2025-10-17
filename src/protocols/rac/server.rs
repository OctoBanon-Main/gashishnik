use anyhow::Result;
use tokio::net::TcpListener;
use crate::{protocols::common::info::build_protocol_info_packet, server::storage::Storage};
use tracing::{info, warn};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

use super::handlers;
use crate::protocols::commands::Command;
use super::common;

use std::boxed::Box;
use tokio_rustls::TlsAcceptor;

use crate::server::io_stream::AsyncStream;
type BoxedStream = Box<dyn AsyncStream>;

pub async fn run_server(
    bind_addr: &str,
    storage: impl Storage,
    auth_only: bool,
    tls_acceptor: Option<Arc<TlsAcceptor>>,
    shutdown: CancellationToken,
) -> Result<()> {
    let storage = Arc::new(storage);
    let listener = TcpListener::bind(bind_addr).await?;

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
                                        Err(e) => {
                                            warn!("TLS handshake failed for {}: {:?}", ip, e);
                                            return;
                                        }
                                    }
                                }
                                None => Box::new(socket),
                            };

                            if let Err(e) = handle_client(boxed, storage, ip.clone(), auth_only).await {
                                warn!("Client {} disconnected with error: {:?}", ip, e);
                            } else {
                                info!("Client {} disconnected", ip);
                            }
                        });
                    }
                    Err(e) => {
                        warn!("Accept error: {:?}", e);
                    }
                }
            }
            _ = shutdown.cancelled() => {
                info!("Shutdown signal received, stopping server loop...");
                break;
            }
        }
    }
    Ok(())
}

async fn handle_client(
    mut socket: BoxedStream,
    storage: Arc<impl Storage>,
    client_ip: String,
    auth_only: bool,
) -> Result<()> {
    let data = common::read_message(&mut socket).await?;

    match Command::try_from(data[0]) {
        Ok(Command::GetMessages) => handlers::handle_get_messages(&mut socket, &*storage).await?,
        Ok(Command::SendUnauthenticated) => {
            if auth_only {
                warn!("Rejected unauthenticated message from {} (auth_only enabled)", client_ip);
                common::write_response(&mut socket, &[0xFF]).await?;
            } else {
                handlers::handle_unauth_message(&mut socket, &*storage, &data[1..], &client_ip).await?;
            }
        }
        Ok(Command::SendAuthenticated) => {
            handlers::handle_auth_message(&mut socket, &*storage, &data[1..], &client_ip).await?;
        }
        Ok(Command::Register) => {
            handlers::handle_registration(&mut socket, &*storage, &data[1..]).await?;
        }
        Ok(Command::Info) => {
            let packet = build_protocol_info_packet(0x03); // RAC/WRAC v2.0
            common::write_response(&mut socket, &packet).await?;
        }
        Err(_) => warn!("Unknown command from {}: 0x{:02x}", client_ip, data[0]),
    }

    Ok(())
}
