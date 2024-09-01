use crate::protocol::{Database, DbKey, DbValue, NetActions, NetCommand, NetResponse, COMMANDS};

pub mod delete;
pub mod insert;
pub mod lookup;

pub async fn handler(command: NetCommand<'_>, db: Database) -> NetResponse
{
    let command_name = command.name.to_uppercase();
    let key: Option<DbKey> = command.key.map(|k| k.to_string());
    let value: Option<DbValue> = command.value;

    if let Some(command_handler) = COMMANDS.get(command_name.as_str()) {
        match command_handler(key, value, db).await {
            Ok(res) => NetResponse {
                action: NetActions::Command,
                value: res.value,
                error: None,
            },
            Err(err_msg) => NetResponse {
                action: NetActions::Error,
                value: None,
                error: Some(err_msg.to_string()),
            },
        }
    } else {
        eprintln!("Unknown command received: {}", command_name);
        NetResponse {
            action: NetActions::Error,
            value: None,
            error: Some("Error: Unknown command.".to_string()),
        }
    }
}
