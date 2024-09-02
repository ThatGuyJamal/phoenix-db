use std::collections::HashMap;
use std::error::Error;

use futures::future::BoxFuture;
use once_cell::sync::Lazy;

use crate::commands::delete::delete_command;
use crate::commands::insert::insert_command;
use crate::commands::lookup::lookup_command;
use crate::protocol::{Database, DbKey, DbValue, NetActions, NetCommand, NetResponse};

pub mod delete;
pub mod insert;
pub mod lookup;

pub struct ManyParams {
    key: Option<DbKey>,
    value: Option<DbValue>,
}

pub enum CommandArgs {
    Single(Option<DbKey>, Option<DbValue>),
    Many(Vec<ManyParams>),
}

// Command function type
type CommandFn = fn(CommandArgs, Database) -> BoxFuture<'static, Result<NetResponse, Box<dyn Error>>>;

// Static command lookup table
pub static COMMANDS: Lazy<HashMap<&'static str, CommandFn>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("INSERT", insert_command as CommandFn);
    map.insert("LOOKUP", lookup_command as CommandFn);
    map.insert("DELETE", delete_command as CommandFn);
    map
});

pub trait CommandExecutor {
    fn execute(&self, args: CommandArgs, db: Database) -> BoxFuture<'static, Result<NetResponse, Box<dyn Error>>>;
}

impl<F> CommandExecutor for F
where
    F: Fn(CommandArgs, Database) -> BoxFuture<'static, Result<NetResponse, Box<dyn Error>>> + Send + Sync + 'static,
{
    fn execute(&self, args: CommandArgs, db: Database) -> BoxFuture<'static, Result<NetResponse, Box<dyn Error>>> {
        self(args, db)
    }
}

pub async fn handler(command: NetCommand<'_>, db: Database) -> NetResponse {
    let command_name = command.name.to_uppercase();
    let keys: Option<Vec<DbKey>> = command.keys.map(|k_list| k_list.into_iter().map(|k| k.to_string()).collect());
    let values: Option<Vec<DbValue>> = command.values;

    match command_name.as_str() {
        "INSERT" | "LOOKUP" | "DELETE" => {
            if let (Some(key), Some(value)) = (
                keys.clone().and_then(|k| k.into_iter().next()),
                values.clone().and_then(|v| v.into_iter().next()),
            ) {
                // Single operation
                if let Some(command_handler) = COMMANDS.get(command_name.as_str()) {
                    match command_handler(CommandArgs::Single(Some(key), Some(value)), db).await {
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
                    NetResponse {
                        action: NetActions::Error,
                        value: None,
                        error: Some("Error: Unknown command.".to_string()),
                    }
                }
            } else {
                NetResponse {
                    action: NetActions::Error,
                    value: None,
                    error: Some("Error: Missing key or value for command.".to_string()),
                }
            }
        }
        "INSERT *" => {
            if let (Some(keys), Some(values)) = (keys, values) {
                let params: Vec<ManyParams> = keys
                    .into_iter()
                    .zip(values)
                    .map(|(key, value)| ManyParams {
                        key: Some(key),
                        value: Some(value),
                    })
                    .collect();

                if let Some(command_handler) = COMMANDS.get("INSERT *") {
                    match command_handler(CommandArgs::Many(params), db).await {
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
                    NetResponse {
                        action: NetActions::Error,
                        value: None,
                        error: Some("Error: Unknown command.".to_string()),
                    }
                }
            } else {
                NetResponse {
                    action: NetActions::Error,
                    value: None,
                    error: Some("Error: Missing keys or values for bulk insert.".to_string()),
                }
            }
        }
        _ => {
            NetResponse {
                action: NetActions::Error,
                value: None,
                error: Some("Error: Unknown command.".to_string()),
            }
        }
    }
}
