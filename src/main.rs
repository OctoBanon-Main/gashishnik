use std::{path::Path, sync::Arc};

use anyhow::Result;
use clap::Parser;
use sqlx::sqlite::SqlitePoolOptions;
use tokio::fs::File;
use tokio_util::sync::CancellationToken;
use tokio_rustls::TlsAcceptor;
use tracing::info;
use tracing_subscriber;

use gashishnik_server::{
    cli::{self, shutdown_signal},
    protocols::rac::run_server,
    server::{
        db::init_database,
        storage::SqliteStorage,
        tls::load_tls_acceptor,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    setup_tracing();
    let args = cli::CliArgs::parse();

    let storage: SqliteStorage = setup_database(&args.db_filename()).await?;
    let tls_acceptor = setup_tls(&args)?;

    let shutdown = CancellationToken::new();
    spawn_shutdown_listener(&shutdown).await;

    info!(
        "Starting server on {} (TLS: {}) {}",
        args.bind_addr(),
        args.tls_enabled().then(|| "enabled").unwrap_or("disabled"),
        args.auth_only.then(|| "in auth-only mode").unwrap_or("")
    );

    run_server(
        &args.bind_addr(),
        storage,
        args.auth_only,
        tls_acceptor,
        shutdown,
    )
    .await?;
    Ok(())
}

fn setup_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();
}

async fn setup_database(path: &str) -> Result<SqliteStorage> {
    if !Path::new(path).exists() {
        info!("Database {} not found, creating...", path);
        File::create(path).await?;
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&format!("sqlite:{}", path))
        .await?;

    init_database(&pool).await?;
    Ok(SqliteStorage::new(pool))
}

fn setup_tls(args: &cli::CliArgs) -> Result<Option<Arc<TlsAcceptor>>> {
    args.tls_cert
        .as_ref()
        .zip(args.tls_key.as_ref())
        .map(|(cert, key)| {
            info!("Enabling TLS with certificate '{}' and key '{}'", cert, key);
            load_tls_acceptor(cert, key)
        })
        .transpose()
}

async fn spawn_shutdown_listener(token: &CancellationToken) {
    let child = token.child_token();
    tokio::spawn(async move {
        shutdown_signal().await;
        child.cancel();
    });
}