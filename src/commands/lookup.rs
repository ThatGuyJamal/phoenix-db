use std::error::Error;

use futures::future::{BoxFuture, FutureExt};

use crate::commands::CommandArgs;
use crate::protocol::{Database, DbValue, NetActions, NetResponse};

/// Executes a lookup command on the database.
///
/// This function handles both single key lookups and bulk lookups based on the provided `CommandArgs`.
/// It retrieves the corresponding values from the database and formats them into a `NetResponse`.
///
/// # Arguments
///
/// * `args` - The arguments for the command, which could be a single key or multiple key-value pairs.
/// * `db` - The database instance used for lookups.
///
/// # Returns
///
/// A `BoxFuture` that resolves to a `Result` containing a `NetResponse`. The response indicates the success
/// or failure of the lookup operation.
pub fn lookup_command(args: CommandArgs, db: Database) -> BoxFuture<'static, Result<NetResponse, Box<dyn Error + Send>>>
{
    async move {
        // Match on the provided command arguments to determine the appropriate action
        let response = match args {
            // Handle single key lookup
            CommandArgs::Single(Some(key), _) => {
                let db_read = db.read().await;
                match db_read.get(&key) {
                    Some(value) => NetResponse {
                        action: NetActions::Command,
                        value: Some(value.to_owned()),
                        error: None,
                    },
                    None => NetResponse {
                        action: NetActions::Command,
                        value: None,
                        error: None,
                    },
                }
            }
            // Handle case where no key is provided
            CommandArgs::Single(None, _) => NetResponse {
                action: NetActions::Error,
                value: None,
                error: Some("No key provided for lookup.".to_string()),
            },
            // Handle bulk lookup
            CommandArgs::Many(pairs) => {
                let db_read = db.read().await;
                let mut results = vec![];

                for pair in pairs {
                    if let Some(key) = pair.key {
                        if let Some(value) = db_read.get(&key) {
                            results.push(value.to_owned());
                        }
                    } else {
                        return Ok(NetResponse {
                            action: NetActions::Error,
                            value: None,
                            error: Some("Missing key in bulk lookup.".to_string()),
                        });
                    }
                }

                NetResponse {
                    action: NetActions::Command,
                    value: Some(DbValue::Array(results)),
                    error: None,
                }
            }
        };

        Ok(response)
    }
    .boxed()
}
