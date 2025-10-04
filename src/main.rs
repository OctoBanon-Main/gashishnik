use anyhow::Result;
use clap::Parser;
use tokio_util::sync::CancellationToken;
use std::path::Path;
use tracing_subscriber;
use tracing::info;

use gashishnik_server::server::db::init_database;
use gashishnik_server::server::storage::SqliteStorage;
use gashishnik_server::protocols::rac::run_server;
use sqlx::sqlite::SqlitePoolOptions;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    let args = gashishnik_server::cli::CliArgs::parse();
    let bind_addr = args.bind_addr();

    let db_path = "gashishnik.db";

    if !Path::new(&db_path).exists() {
        info!("Database file {} not found, creating...", db_path);
        std::fs::File::create(&db_path)?;
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&format!("sqlite:{}", db_path))
        .await?;

    init_database(&pool).await?;

    let storage = SqliteStorage::new(pool);

    let shutdown_token = CancellationToken::new();
    let shutdown_token_child = shutdown_token.clone();

    tokio::spawn(async move {
        shutdown_signal().await;
        shutdown_token_child.cancel();
    });

    let tls_acceptor = args
        .tls_cert
        .as_ref()
        .zip(args.tls_key.as_ref())
        .map(|(cert, key)| {
            info!("Enabling TLS with certificate '{}' and key '{}'", cert, key);
            gashishnik_server::server::tls::load_tls_acceptor(cert, key)
        })
        .transpose()?;


    info!(
        "Starting server on {} (TLS: {})",
        bind_addr,
        args.tls_enabled().then(|| "enabled").unwrap_or("disabled")
    );

    info!(
        "Server started {}",
        args.auth_only.then(|| "in auth-only mode").unwrap_or("in normal mode")
    );

    run_server(
        &bind_addr,
        storage,
        args.auth_only,
        tls_acceptor,
        shutdown_token.clone(),
    )
    .await?;
    Ok(())
}