mod commands;
mod net;
mod protocol;

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use protocol::{DbEngine, DbMetadata};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, RwLock};

use crate::net::handle_stream;
use crate::net::ttl::cleanup_task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let listener = TcpListener::bind(socket).await?;

    println!("Listening on {}", socket.to_string());

    let engine = Arc::new(DbEngine {
        connection: Arc::new(RwLock::new(HashMap::new())),
        metadata: DbMetadata::default(),
    });

    let (tx, mut rx) = mpsc::channel(1024);

    // Spawn cleanup task
    let engine_clone = engine.clone();
    tokio::spawn(async move {
        cleanup_task(engine_clone.connection.clone(), Duration::from_secs(60)).await;
    });

    // Spawn task to handle streams
    tokio::spawn(async move {
        while let Some((stream, db)) = rx.recv().await {
            tokio::spawn(handle_stream(stream, db));
        }
    });

    // Main loop to accept connections and send to channel
    loop {
        let (stream, _) = listener.accept().await?;
        tx.send((stream, engine.connection.clone())).await?;
    }
}
