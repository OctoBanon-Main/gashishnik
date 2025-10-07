use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "gashishnik", version, about = "High-performance RAC protocol server implementation in Rust")]
pub struct CliArgs {
    #[arg(short = 'i', long = "bind-ip")]
    pub bind_ip: String,
    
    #[arg(short = 'p', long = "bind-port")]
    pub bind_port: Option<u16>,

    #[arg(long)]
    pub auth_only: bool,

    #[arg(long)]
    pub database_name: Option<String>,

    #[arg(long)]
    pub tls_cert: Option<String>,

    #[arg(long)]
    pub tls_key: Option<String>,
}

impl CliArgs {
    pub fn tls_enabled(&self) -> bool {
        self.tls_cert.is_some() && self.tls_key.is_some()
    }

    pub fn default_port(&self) -> u16 {
        self.tls_enabled().then(|| 42667).unwrap_or(42666)
    }

    pub fn db_filename(&self) -> String {
        self.database_name
            .as_ref()
            .map(|name| format!("{name}.db"))
            .unwrap_or_else(|| "gashishnik.db".to_string())
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.bind_ip, self.bind_port.unwrap_or_else(|| self.default_port()))
    }
}