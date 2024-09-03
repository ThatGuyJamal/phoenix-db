mod commands;
mod net;
mod protocol;
use std::collections::HashMap;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use protocol::{DbEngine, DbMetadata};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, RwLock};

use crate::net::handle_stream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>
{
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let listener = TcpListener::bind(socket).await?;

    println!("Listening on {}", socket.to_string());

    let engine = DbEngine {
        connection: Arc::new(RwLock::new(HashMap::new())),
        metadata: DbMetadata::default(),
    };

    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        while let Some((stream, db)) = rx.recv().await {
            tokio::spawn(handle_stream(stream, db));
        }
    });

    loop {
        let (stream, _) = listener.accept().await?;
        tx.send((stream, Arc::clone(&engine.connection))).await?;
    }
}
