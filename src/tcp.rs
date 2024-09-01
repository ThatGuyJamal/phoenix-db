use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::protocol::{Database, NetCommand};

pub async fn handle_stream(mut stream: TcpStream, db: Database) -> Result<(), String>
{
    let client_addr = stream
        .peer_addr()
        .unwrap_or_else(|_| "unknown address".to_string().parse().unwrap());

    println!("Connected to client: {}", client_addr);

    let mut buffer = vec![0; 1024];

    loop {
        // Read the incoming data
        match stream.read(&mut buffer).await {
            Ok(size) => {
                if size == 0 {
                    // Client disconnected
                    println!("Client disconnected: {}", client_addr);
                    return Ok(());
                }

                // Deserialize the incoming data into a Command struct
                match serde_json::from_slice::<NetCommand>(&buffer[..size]) {
                    Ok(command) => {
                        let response = crate::commands::handler(command, db.clone()).await;

                        // Serialize the response before sending it back
                        match serde_json::to_string(&response) {
                            Ok(response_json) => {
                                if let Err(e) = stream.write_all(response_json.as_bytes()).await {
                                    eprintln!("Failed to write to stream: {}", e);
                                    return Err(format!("Failed to write to stream: {}", e));
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to serialize response: {}", e);
                                return Err(format!("Failed to serialize response: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to deserialize command: {}", e);
                        return Err(format!("Failed to deserialize command: {}", e));
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                return Err(format!("Failed to read from stream: {}", e));
            }
        }
    }
}
