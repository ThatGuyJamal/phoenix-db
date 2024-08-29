use std::collections::HashMap;
use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::RwLock;

pub type Db = Arc<RwLock<HashMap<String, String>>>;

/**
 * Handle incoming commands from the client and respond to them
 *
 * `stream` - The TCP stream from the client
 * `db` - The database
 */
pub async fn handle_commands(stream: TcpStream, db: Db)
{
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    let mut buffer = String::new();

    while match reader.read_line(&mut buffer).await {
        Ok(0) => false, // Connection closed
        Ok(_) => {
            let command: Vec<&str> = buffer.trim().split_whitespace().collect();

            #[allow(unused_assignments)]
            let mut response = String::new();

            match command.as_slice() {
                ["SET", key, value] => {
                    let mut db = db.write().await; // Acquire write lock
                    db.insert(key.to_string(), value.to_string());
                    response = "OK\n".to_string();
                }
                ["GET", key] => {
                    let db = db.read().await; // Acquire read lock
                    if let Some(value) = db.get(*key) {
                        response = format!("VALUE {}\n", value);
                    } else {
                        response = "NOT FOUND\n".to_string();
                    }
                }
                ["DEL", key] => {
                    let mut db = db.write().await; // Acquire write lock
                    if db.remove(*key).is_some() {
                        response = "OK\n".to_string();
                    } else {
                        response = "NOT FOUND\n".to_string();
                    }
                }
                ["LIST"] => {
                    let db = db.read().await; // Acquire read lock
                    let keys: Vec<String> = db.keys().cloned().collect();
                    if keys.is_empty() {
                        response = "EMPTY\n".to_string();
                    } else {
                        response = format!("KEYS {}\n", keys.join(", "));
                    }
                }
                ["EXIT"] => {
                    response = "EXITING\n".to_string();
                    if let Err(_) = writer.write_all(response.as_bytes()).await {
                        return;
                    }
                    return; // Exit the server
                }
                ["HELP"] => {
                    response = "Commands:\nSET key value - Set the value of a key\nGET key - Get the value of a key\nDEL \
                                key - Delete the value of a key\nLIST - List all keys\nEXIT - Exit the database\nHELP - \
                                List all commands\n"
                        .to_string();
                }
                _ => response = "ERROR Unknown command\n".to_string(),
            }

            if let Err(_) = writer.write_all(response.as_bytes()).await {
                return;
            }
            buffer.clear(); // Clear buffer for the next command
            true
        }
        Err(_) => false, // Error reading from stream
    } {}
}
