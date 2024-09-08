use clap::Parser;

/// Represents the command-line arguments for the server configuration
#[derive(Parser, Debug, Clone)]
#[command(name = "Server Engine")]
#[command(about = "A CLI for the server engine", long_about = None)]
pub struct Cli
{
    /// The port to bind the server to
    #[arg(short = 'p', long, default_value_t = 6969)]
    pub(crate) port: u16,

    /// The address to bind the server to
    #[arg(short = 'a', long, default_value = "127.0.0.1")]
    pub(crate) addr: String,

    /// Optional username for authentication
    #[arg(short = 'u', long)]
    pub(crate) username: Option<String>,

    /// Optional password for authentication
    #[arg(short = 'w', long)]
    pub(crate) password: Option<String>,

    /// Enable debug mode
    #[arg(short = 'd', long, default_value_t = false)]
    pub(crate) debug_mode: bool,

    /// Log level (error, warn, info, debug, trace)
    #[arg(short = 'l', long, default_value = "info")]
    pub(crate) log_level: String,
}
