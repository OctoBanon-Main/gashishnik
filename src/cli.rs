use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "gashishnik", version, about = "High-performance RAC protocol server implementation in Rust")]
pub struct CliArgs {
    #[arg(short = 'i', long = "bind-ip")]
    pub bind_ip: String,
    
    #[arg(short = 'p', long = "bind-port")]
    pub bind_port: u16,

    #[arg(long)]
    pub auth_only: bool,

    #[arg(long)]
    pub tls_cert: Option<String>,

    #[arg(long)]
    pub tls_key: Option<String>,
}