use std::collections::HashMap;
use std::sync::Arc;

use phoenix_common::log;
use phoenix_server::{handle_commands, Db};
use tokio::net::TcpListener;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> std::io::Result<()>
{
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    let db: Db = Arc::new(RwLock::new(HashMap::new()));

    log("Server started on 127.0.0.1:7878");

    loop {
        let (stream, _) = listener.accept().await?;
        let db = Arc::clone(&db);
        tokio::spawn(async move {
            handle_commands(stream, db).await;
        });
    }
}
