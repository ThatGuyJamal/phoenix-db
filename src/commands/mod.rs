use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use futures::future::BoxFuture;
use once_cell::sync::Lazy;

use crate::commands::delete::delete_command;
use crate::commands::insert::insert_command;
use crate::commands::lookup::lookup_command;
use crate::protocol::{Database, DbKey, DbValue, NetActions, NetCommand, NetResponse};

pub mod delete;
pub mod insert;
pub mod lookup;

/// Represents parameters for commands that require multiple keys and values.
pub struct ManyParams
{
    pub key: Option<DbKey>,
    pub value: Option<DbValue>,
}

/// Represents the arguments that can be passed to a command, either a single key-value pair or multiple pairs.
pub enum CommandArgs
{
    Single(Option<DbKey>, Option<DbValue>),
    Many(Vec<ManyParams>),
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
    map.insert("LOOKUP", Arc::new(lookup_command) as Arc<dyn CommandExecutor>);
    map.insert("DELETE", Arc::new(delete_command) as Arc<dyn CommandExecutor>);
    map
});

/// Executes the command using the corresponding command executor.
/// Returns a `NetResponse` indicating the success or failure of the command.
async fn execute_command(command_name: &str, args: CommandArgs, db: Database) -> NetResponse
{
    if let Some(command_executor) = COMMANDS.get(command_name) {
        match command_executor.execute(args, db).await {
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
}

/// Handles the `INSERT` command. Requires a single key and value.
/// Returns a `NetResponse` indicating the result of the `INSERT` command.
async fn handle_insert(keys: Option<Vec<DbKey>>, values: Option<Vec<DbValue>>, db: Database) -> NetResponse
{
    if let (Some(key), Some(value)) = (
        keys.and_then(|k| k.into_iter().next()),
        values.and_then(|v| v.into_iter().next()),
    ) {
        execute_command("INSERT", CommandArgs::Single(Some(key), Some(value)), db).await
    } else {
        NetResponse {
            action: NetActions::Error,
            value: None,
            error: Some("Error: Missing key or value for INSERT command.".to_string()),
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

/// Handles the `INSERT *` command, which supports bulk insertion of key-value pairs.
/// Requires both keys and values to be provided.
/// Returns a `NetResponse` indicating the result of the bulk `INSERT` command.
async fn handle_insert_bulk(keys: Option<Vec<DbKey>>, values: Option<Vec<DbValue>>, db: Database) -> NetResponse
{
    if let (Some(keys), Some(values)) = (keys, values) {
        let params: Vec<ManyParams> = keys
            .into_iter()
            .zip(values)
            .map(|(key, value)| ManyParams {
                key: Some(key),
                value: Some(value),
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

/// Main handler for processing commands.
/// Matches the command name and delegates to the appropriate handler function.
/// Returns a `NetResponse` based on the execution result of the command.
pub async fn handler(command: NetCommand<'_>, db: Database) -> NetResponse
{
    let command_name = command.name.to_uppercase();
    let keys: Option<Vec<DbKey>> = command.keys.map(|k_list| k_list.into_iter().map(|k| k.to_string()).collect());
    let values: Option<Vec<DbValue>> = command.values;

    match command_name.as_str() {
        "INSERT" => handle_insert(keys, values, db).await,
        "LOOKUP" => handle_lookup(keys, db).await,
        "DELETE" => handle_delete(keys, db).await,
        "INSERT *" => handle_insert_bulk(keys, values, db).await,
        _ => NetResponse {
            action: NetActions::Error,
            value: None,
            error: Some("Error: Unknown command.".to_string()),
        },
    }
}
