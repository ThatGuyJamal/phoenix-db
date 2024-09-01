use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::protocol::{Database, NetActions, NetCommand, NetResponse};

pub async fn handle_stream(mut stream: TcpStream, db: Database) -> Result<(), String>
{
    let mut buffer = vec![0; 1024];

    match stream.read(&mut buffer).await {
        Ok(size) => {
            if size == 0 {
                // Client disconnected
                return Ok(());
            }

            // Deserialize the incoming data into a NetCommand struct
            match serde_json::from_slice::<NetCommand>(&buffer[..size]) {
                Ok(command) => {
                    // Handle the command and get a NetResponse
                    let response: NetResponse = crate::commands::handler(command, db).await;

                    // Serialize the NetResponse before sending it back
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
                    // Send an error response to the client
                    let error_response = NetResponse {
                        action: NetActions::Error,
                        value: None,
                        error: Some(format!("Failed to deserialize command: {}", e)),
                    };
                    let error_response_json = serde_json::to_string(&error_response).unwrap_or_else(|e| {
                        serde_json::to_string(&NetResponse {
                            action: NetActions::Error,
                            value: None,
                            error: Some(format!("Serialization error: {}", e)),
                        })
                        .unwrap()
                    });
                    if let Err(e) = stream.write_all(error_response_json.as_bytes()).await {
                        eprintln!("Failed to write error response to stream: {}", e);
                        return Err(format!("Failed to write error response to stream: {}", e));
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
            return Err(format!("Failed to read from stream: {}", e));
        }
    }

    Ok(())
}
