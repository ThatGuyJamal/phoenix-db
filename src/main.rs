mod commands;
mod protocol;
mod tcp;

use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use protocol::{DbEngine, DbMetadata};
use tcp::handle_connection;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, RwLock};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>
{
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    let engine = DbEngine {
        connection: Arc::new(RwLock::new(HashMap::new())),
        metadata: DbMetadata::default(),
    };

    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        while let Some((stream, db)) = rx.recv().await {
            tokio::spawn(handle_connection(stream, db));
        }
    });

    loop {
        let (stream, _) = listener.accept().await?;
        tx.send((stream, Arc::clone(&engine.connection))).await?;
    }
}
