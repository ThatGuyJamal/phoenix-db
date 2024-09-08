mod cli;
mod commands;
mod protocol;

mod services;

mod server;

use std::collections::HashMap;
use std::sync::Arc;

use clap::Parser;
use protocol::DbEngine;
use tokio::sync::RwLock;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    // Parse CLI arguments
    let args = Cli::parse();

    // Convert log level string to `tracing::Level`
    let log_level = match args.log_level.to_lowercase().as_str() {
        "error" => Level::ERROR,
        "warn" => Level::WARN,
        "info" => Level::INFO,
        "debug" => Level::DEBUG,
        "trace" => Level::TRACE,
        _ => Level::INFO, // Default to INFO if the input is invalid
    };

    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let engine = Arc::new(DbEngine {
        connection: Arc::new(RwLock::new(HashMap::new())),
        db_config: args.clone(),
    });

    services::execute(engine.clone()).await?;
    server::execute(&args, &engine).await?;

    Ok(())
}
