use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use futures::future::BoxFuture;
use once_cell::sync::Lazy;
use serde_json::Value;

use crate::commands::delete::delete_command;
use crate::commands::insert::insert_command;
use crate::commands::lookup::lookup_command;
use crate::protocol::{Database, DbKey, DbValue, NetActions, NetCommand, NetResponse};

pub mod delete;
pub mod insert;
pub mod lookup;

/// Represents parameters for commands that require multiple keys and values.
pub struct CommandParams
{
    pub key: Option<DbKey>,
    pub value: Option<Value>,
    pub ttl: Option<Duration>,
}

/// Represents the arguments that can be passed to a command, either a single key-value pair or multiple pairs.
pub enum CommandArgs
{
    Single(Option<DbKey>, Option<DbValue>),
    Many(Vec<CommandParams>),
}

/// Trait that defines the interface for executing commands.
pub trait CommandExecutor: Send + Sync
{
    /// Executes a command with the given arguments and database.
    /// Returns a future that resolves to a `NetResponse`.
    fn execute(&self, args: CommandArgs, db: Database) -> BoxFuture<'static, Result<NetResponse, Box<dyn Error + Send>>>;
}

impl<F> CommandExecutor for F
where
    F: Fn(CommandArgs, Database) -> BoxFuture<'static, Result<NetResponse, Box<dyn Error + Send>>> + Send + Sync + 'static,
{
    fn execute(&self, args: CommandArgs, db: Database) -> BoxFuture<'static, Result<NetResponse, Box<dyn Error + Send>>>
    {
        self(args, db)
    }
}

// Map for storing command executors
pub static COMMANDS: Lazy<HashMap<&'static str, Arc<dyn CommandExecutor>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("INSERT", Arc::new(insert_command) as Arc<dyn CommandExecutor>);
    map.insert("INSERT *", Arc::new(insert_command) as Arc<dyn CommandExecutor>);
    map.insert("LOOKUP", Arc::new(lookup_command) as Arc<dyn CommandExecutor>);
    map.insert("LOOKUP *", Arc::new(lookup_command) as Arc<dyn CommandExecutor>);
    map.insert("DELETE", Arc::new(delete_command) as Arc<dyn CommandExecutor>);
    map.insert("DELETE *", Arc::new(delete_command) as Arc<dyn CommandExecutor>);
    map
});

/// Executes the command using the corresponding command executor.
/// Returns a `NetResponse` indicating the success or failure of the command.
async fn execute_command(command_name: &str, args: CommandArgs, db: Database) -> NetResponse
{
    if let Some(command_executor) = COMMANDS.get(command_name) {
        match command_executor.execute(args, db).await {
            Ok(res) => res.into(),
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
}

/// Handles the `INSERT` command. Requires a single key and value.
/// Returns a `NetResponse` indicating the result of the `INSERT` command.
async fn handle_insert(keys: Option<Vec<DbKey>>, values: Option<Vec<DbValue>>, db: Database) -> NetResponse
{
    if let (Some(key), Some(data)) = (
        keys.and_then(|k| k.into_iter().next()),
        values.and_then(|v| v.into_iter().next()),
    ) {
        execute_command(
            "INSERT",
            CommandArgs::Single(
                Some(key),
                Some(DbValue {
                    value: data.value,
                    expires_in: data.expires_in,
                }),
            ),
            db,
        )
        .await
    } else {
        NetResponse {
            action: NetActions::Error,
            value: None,
            error: Some("Error: Missing key or value for INSERT command.".to_string()),
        }
    }
}

/// Handles the `INSERT *` command, which supports bulk insertion of key-value pairs.
/// Requires both keys and values to be provided.
/// Returns a `NetResponse` indicating the result of the bulk `INSERT` command.
async fn handle_insert_bulk(keys: Option<Vec<DbKey>>, values: Option<Vec<DbValue>>, db: Database) -> NetResponse
{
    if let (Some(keys), Some(values)) = (keys, values) {
        let params: Vec<CommandParams> = keys
            .into_iter()
            .zip(values)
            .map(|(key, value)| CommandParams {
                key: Some(key),
                value: Some(value.value),
                ttl: value.expires_in,
            })
            .collect();

        execute_command("INSERT *", CommandArgs::Many(params), db).await
    } else {
        NetResponse {
            action: NetActions::Error,
            value: None,
            error: Some("Error: Missing keys or values for bulk insert.".to_string()),
        }
    }
}

/// Handles the `LOOKUP` command. Requires a single key.
/// Returns a `NetResponse` indicating the result of the `LOOKUP` command.
async fn handle_lookup(keys: Option<Vec<DbKey>>, db: Database) -> NetResponse
{
    if let Some(key) = keys.and_then(|k| k.into_iter().next()) {
        execute_command("LOOKUP", CommandArgs::Single(Some(key), None), db).await
    } else {
        NetResponse {
            action: NetActions::Error,
            value: None,
            error: Some("Error: Missing key for LOOKUP command.".to_string()),
        }
    }
}

/// Handles the `LOOKUP *` command, which supports bulk lookups of multiple keys.
/// Requires a list of keys to be provided.
/// Returns a `NetResponse` indicating the result of the bulk `LOOKUP` command.
async fn handle_lookup_bulk(keys: Option<Vec<DbKey>>, db: Database) -> NetResponse
{
    if let Some(keys) = keys {
        let params: Vec<CommandParams> = keys
            .into_iter()
            .map(|key| CommandParams {
                key: Some(key),
                value: None,
                ttl: None,
            })
            .collect();
        execute_command("LOOKUP *", CommandArgs::Many(params), db).await
    } else {
        NetResponse {
            action: NetActions::Error,
            value: None,
            error: Some("Error: Missing keys for bulk lookup.".to_string()),
        }
    }
}

/// Handles the `DELETE` command. Requires a single key.
/// Returns a `NetResponse` indicating the result of the `DELETE` command.
async fn handle_delete(keys: Option<Vec<DbKey>>, db: Database) -> NetResponse
{
    if let Some(key) = keys.and_then(|k| k.into_iter().next()) {
        execute_command("DELETE", CommandArgs::Single(Some(key), None), db).await
    } else {
        NetResponse {
            action: NetActions::Error,
            value: None,
            error: Some("Error: Missing key for DELETE command.".to_string()),
        }
    }
}

/// Handles the `DELETE *` command, which supports bulk deletion of multiple keys.
/// Requires a list of keys to be provided.
/// Returns a `NetResponse` indicating the result of the bulk `DELETE` command.
async fn handle_delete_bulk(keys: Option<Vec<DbKey>>, db: Database) -> NetResponse
{
    if let Some(keys) = keys {
        let params: Vec<CommandParams> = keys
            .into_iter()
            .map(|key| CommandParams {
                key: Some(key),
                value: None,
                ttl: None,
            })
            .collect();
        execute_command("DELETE *", CommandArgs::Many(params), db).await
    } else {
        NetResponse {
            action: NetActions::Error,
            value: None,
            error: Some("Error: Missing keys for bulk delete.".to_string()),
        }
    }
}

/// Main handler for processing commands.
/// Matches the command name and delegates to the appropriate handler function.
/// Returns a `NetResponse` based on the execution result of the command.
pub async fn handler(command: NetCommand<'_>, db: Database) -> NetResponse
{
    let command_name = command.name.to_uppercase();
    let keys: Option<Vec<DbKey>> = command.keys.map(|k_list| k_list.into_iter().map(|k| k.to_string()).collect());

    // Map values to DbValue with optional TTL
    let values: Option<Vec<DbValue>> = if let Some(vals) = command.values {
        Some(
            vals.into_iter()
                .zip(command.ttls.unwrap())  // Handle TTLs
                .map(|(val, ttl)| DbValue {
                    value: val.value,
                    expires_in: Option::from(ttl),  // This now works as expires_in expects Option<Duration>
                })
                .collect(),
        )
    } else {
        None
    };

    match command_name.as_str() {
        "INSERT" => handle_insert(keys, values, db).await,
        "LOOKUP" => handle_lookup(keys, db).await,
        "DELETE" => handle_delete(keys, db).await,
        "INSERT *" => handle_insert_bulk(keys, values, db).await,
        "LOOKUP *" => handle_lookup_bulk(keys, db).await,
        "DELETE *" => handle_delete_bulk(keys, db).await,
        _ => NetResponse {
            action: NetActions::Error,
            value: None,
            error: Some("Error: Unknown command.".to_string()),
        },
    }
}
