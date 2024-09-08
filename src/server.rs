use std::net::SocketAddr;

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{debug, info};

use crate::cli::Cli;
use crate::protocol::{Database, DbEngine};
use crate::services::tcp;

pub async fn execute(args: &Cli, engine: &DbEngine) -> Result<(), Box<dyn std::error::Error>>
{
    let socket = SocketAddr::new(args.addr.parse().unwrap(), args.port);
    let listener = TcpListener::bind(socket).await?;

    let (tx, mut rx): (Sender<(TcpStream, Database)>, Receiver<(TcpStream, Database)>) = mpsc::channel(1024);

    // Spawn task to handle streams
    tokio::spawn(async move {
        debug!("Starting TCP Service");
        while let Some((stream, db)) = rx.recv().await {
            tokio::spawn(tcp::execute(stream, db));
        }
    });

    info!("Listening on {}", socket.to_string());

    // Main loop to accept connections and send to channel
    loop {
        let (stream, _) = listener.accept().await?;
        tx.send((stream, engine.connection.clone())).await?;
    }
}
