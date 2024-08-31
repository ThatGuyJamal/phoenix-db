use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::protocol::Database;

pub async fn handle_connection(mut stream: TcpStream, db: Database)
{
    let mut buffer = vec![0; 1024];

    match stream.read(&mut buffer).await {
        Ok(size) => {
            // If the buffer is empty, the client has disconnected
            if size == 0 {
                return;
            }

            // Handle the incoming command
            if let Ok(command_str) = String::from_utf8(buffer[..size].to_vec()) {
                let response = crate::commands::handler(command_str, db).await;
                stream.write_all(response.as_bytes()).await.unwrap();
            }
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
        }
    }
}
