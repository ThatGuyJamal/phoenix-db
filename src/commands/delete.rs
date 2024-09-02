use std::error::Error;

use futures::future::BoxFuture;
use futures::FutureExt;

use crate::commands::CommandArgs;
use crate::protocol::{Database, DbValue, NetActions, NetResponse};

/// Executes a delete command on the database.
///
/// This function handles both single key deletions and bulk deletions based on the provided `CommandArgs`.
/// It removes the specified key-value pairs from the database and returns a `NetResponse` indicating success or errors.
///
/// # Arguments
///
/// * `args` - The arguments for the command, which could be a single key or multiple keys for bulk deletion.
/// * `db` - The database instance used for deletion.
///
/// # Returns
///
/// A `BoxFuture` that resolves to a `Result` containing a `NetResponse`. The response indicates the success
/// or failure of the deletion operation.
pub fn delete_command(args: CommandArgs, db: Database) -> BoxFuture<'static, Result<NetResponse, Box<dyn Error + Send>>>
{
    async move {
        let response = match args {
            CommandArgs::Single(Some(key), _) => {
                let mut db_write = db.write().await;
                if db_write.remove(&key).is_some() {
                    NetResponse {
                        action: NetActions::Command,
                        value: Some("OK".to_string().into()),
                        error: None,
                    }
                } else {
                    NetResponse {
                        action: NetActions::Error,
                        value: None,
                        error: Some(format!("Key '{}' not found.", key)),
                    }
                }
            }
            CommandArgs::Single(None, _) => NetResponse {
                action: NetActions::Error,
                value: None,
                error: Some("No key provided for delete.".to_string()),
            },
            CommandArgs::Many(pairs) => {
                let mut db_write = db.write().await;
                let mut results = vec![];
                for pair in pairs {
                    if let Some(key) = pair.key {
                        if db_write.remove(&key).is_some() {
                            results.push(key);
                        }
                    } else {
                        return Ok(NetResponse {
                            action: NetActions::Error,
                            value: None,
                            error: Some("Missing key in bulk delete.".to_string()),
                        });
                    }
                }
                NetResponse {
                    action: NetActions::Command,
                    value: Some(DbValue::Array(results.into_iter().map(|key| DbValue::String(key)).collect())),
                    error: None,
                }
            }
        };

        Ok(response)
    }
    .boxed()
}
