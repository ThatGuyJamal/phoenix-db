use std::collections::HashMap;
use std::error::Error;

use futures::future::{BoxFuture, FutureExt};

use crate::commands::CommandArgs;
use crate::protocol::{Database, DbKey, DbValue, NetActions, NetResponse};

/// Executes an insert command on the database.
///
/// This function handles both single key-value insertions and bulk insertions based on the provided `CommandArgs`.
/// It updates the database with the given key-value pairs and returns a `NetResponse` indicating success or errors.
///
/// # Arguments
///
/// * `args` - The arguments for the command, which could be a single key-value pair or multiple key-value pairs.
/// * `db` - The database instance used for insertions.
///
/// # Returns
///
/// A `BoxFuture` that resolves to a `Result` containing a `NetResponse`. The response indicates the success
/// or failure of the insertion operation.
pub fn insert_command(args: CommandArgs, db: Database) -> BoxFuture<'static, Result<NetResponse, Box<dyn Error + Send>>>
{
    async move {
        let response = match args {
            // Handle single key-value insertion
            CommandArgs::Single(Some(key), Some(value)) => {
                let mut db_write = db.write().await;
                db_write.insert(key, value);
                NetResponse {
                    action: NetActions::Command,
                    value: Some("OK".to_string().into()),
                    error: None,
                }
            }
            // Handle case where no key is provided
            CommandArgs::Single(None, _) => NetResponse {
                action: NetActions::Error,
                value: None,
                error: Some("No key provided for insert.".to_string()),
            },
            // Handle case where no value is provided
            CommandArgs::Single(_, None) => NetResponse {
                action: NetActions::Error,
                value: None,
                error: Some("No value provided for insert.".to_string()),
            },
            // Handle bulk insertions
            CommandArgs::Many(args) => {
                let mut temp_map: HashMap<DbKey, DbValue> = HashMap::new();
                let mut insert_errors = Vec::new();

                for a in args {
                    match (a.key, a.value) {
                        (Some(key), Some(value)) => {
                            temp_map.insert(key, value);
                        }
                        (Some(key), None) => {
                            insert_errors.push(format!("Missing value for key: {}", key));
                        }
                        (None, Some(_)) => {
                            insert_errors.push("Key is missing for provided value".to_string());
                        }
                        (None, None) => {
                            insert_errors.push("Both key and value are missing".to_string());
                        }
                    }
                }

                if insert_errors.is_empty() {
                    let mut db_lock = db.write().await;
                    db_lock.extend(temp_map);
                    NetResponse {
                        action: NetActions::Command,
                        value: Some("OK".to_string().into()),
                        error: None,
                    }
                } else {
                    NetResponse {
                        action: NetActions::Command,
                        value: Some("OK".to_string().into()),
                        error: Some(insert_errors.join(", ")),
                    }
                }
            }
        };

        Ok(response)
    }
    .boxed()
}
