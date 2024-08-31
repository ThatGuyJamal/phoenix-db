use crate::protocol::{Database, COMMANDS};

pub mod insert;
pub mod lookup;

pub async fn handler(command_str: String, db: Database) -> String
{
    // Split the command into parts
    let parts: Vec<&str> = command_str.trim().split_whitespace().collect();

    // Check if the command is valid
    if parts.is_empty() {
        return "Invalid command".to_string();
    }

    let command_name = parts[0].to_uppercase();
    let args: Vec<String> = parts[1..].iter().map(|&s| s.to_string()).collect();

    if let Some(command) = COMMANDS.get(command_name.as_str()) {
        (command)(args, db).await
    } else {
        "Unknown command".to_string()
    }
}
