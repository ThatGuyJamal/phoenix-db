pub mod ttl;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::protocol::{Database, NetActions, NetCommand, NetResponse};

/// Handles a single client connection over a TCP stream.
///
/// This function reads commands from the client, processes them using the `handler` function,
/// and sends back responses or error messages. It runs in a loop until the client disconnects.
///
/// # Arguments
///
/// * `stream` - The TCP stream representing the client connection.
/// * `db` - The database instance used to process commands.
///
/// # Returns
///
/// A `Result` indicating success or failure of handling the stream. Errors are returned as `String`.
pub async fn handle_stream(mut stream: TcpStream, db: Database) -> Result<(), String>
{
    let client_addr = stream
        .peer_addr()
        .unwrap_or_else(|_| "unknown address".to_string().parse().unwrap());

    println!("Connected to client: {}", client_addr);

    let mut buffer = vec![0; 1024];

    loop {
        match stream.read(&mut buffer).await {
            Ok(size) => {
                if size == 0 {
                    // Client has disconnected
                    println!("Client disconnected: {}", client_addr);

                    return Ok(());
                }

                // Deserialize the incoming data into a `NetCommand` struct
                match serde_json::from_slice::<NetCommand>(&buffer[..size]) {
                    Ok(command) => {
                        // Process the command and get the response
                        let response = crate::commands::handler(command, db.clone()).await;

                        // Serialize the response to JSON format
                        match serde_json::to_string(&response) {
                            Ok(response_json) => {
                                // Write the response back to the client
                                if let Err(e) = stream.write_all(response_json.as_bytes()).await {
                                    eprintln!("Failed to write to stream: {}", e);
                                    send_error_response(&mut stream, &e.to_string()).await?;
                                    return Err(format!("Failed to write to stream: {}", e));
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to serialize response: {}", e);
                                send_error_response(&mut stream, &e.to_string()).await?;
                                return Err(format!("Failed to serialize response: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to deserialize command: {}", e);
                        send_error_response(&mut stream, &e.to_string()).await?;
                        return Err(format!("Failed to deserialize command: {}", e));
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                send_error_response(&mut stream, &e.to_string()).await?;
                return Err(format!("Failed to read from stream: {}", e));
            }
        }
    }
}

/// Sends an error response back to the client.
///
/// This function creates a `NetResponse` indicating an error and sends it over the TCP stream.
///
/// # Arguments
///
/// * `stream` - The TCP stream representing the client connection.
/// * `error_message` - The error message to include in the response.
///
/// # Returns
///
/// A `Result` indicating success or failure of sending the error response. Errors are returned as `String`.
async fn send_error_response(stream: &mut TcpStream, error_message: &str) -> Result<(), String>
{
    // Create an error response with the provided error message
    let error_response = NetResponse {
        action: NetActions::Error,
        value: None,
        error: Some(error_message.to_string()),
    };

    // Serialize the error response to JSON format
    match serde_json::to_string(&error_response) {
        Ok(response_json) => {
            // Write the error response back to the client
            if let Err(e) = stream.write_all(response_json.as_bytes()).await {
                eprintln!("Failed to write error response to stream: {}", e);
                return Err(format!("Failed to write error response to stream: {}", e));
            }
        }
        Err(e) => {
            eprintln!("Failed to serialize error response: {}", e);
            return Err(format!("Failed to serialize error response: {}", e));
        }
    }

    Ok(())
}
