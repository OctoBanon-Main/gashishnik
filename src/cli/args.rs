use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum Mode {
    RAC,
    WRAC
}

#[derive(Parser, Debug)]
#[command(name = "gashishnik", version, about = "High-performance RAC protocol server implementation in Rust")]
pub struct CliArgs {
    #[arg(short = 'a', long, help = "IP address to bind to")]
    pub address: String,

    #[arg(short = 'p', long, help = "Port to listen on (defaults depend on mode and TLS)")]
    pub port: Option<u16>,

    #[arg(long, help = "Run server in authentication-only mode")]
    pub auth_only: bool,

    #[arg(long, default_value = "rac", value_enum, help = "Protocol mode: RAC or WRAC")]
    pub mode: Mode,

    #[arg(long, help = "Database name (will be stored as <name>.db)")]
    pub database_name: Option<String>,

    #[arg(long, help = "Path to TLS certificate file (enables TLS)")]
    pub tls_cert: Option<String>,

    #[arg(long, help = "Path to TLS private key file (enables TLS)")]
    pub tls_key: Option<String>,
}

impl CliArgs {
    pub fn tls_enabled(&self) -> bool {
        self.tls_cert.is_some() && self.tls_key.is_some()
    }

    pub fn default_port(&self) -> u16 {
        match (self.mode, self.tls_enabled()) {
            (Mode::RAC, false) => 42666,
            (Mode::RAC, true) => 42667,
            (Mode::WRAC, false) => 52666,
            (Mode::WRAC, true) => 52667
        }
    }

    pub fn db_filename(&self) -> String {
        self.database_name
            .as_ref()
            .map(|name| format!("{name}.db"))
            .unwrap_or_else(|| "gashishnik.db".to_string())
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.address, self.port.unwrap_or_else(|| self.default_port()))
    }
}