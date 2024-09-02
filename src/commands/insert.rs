use futures::future::{BoxFuture, FutureExt};
use std::collections::HashMap;
use std::error::Error;

use crate::commands::CommandArgs;
use crate::protocol::{Database, DbKey, DbValue, NetActions, NetResponse};

pub fn insert_command(args: CommandArgs, db: Database) -> BoxFuture<'static, Result<NetResponse, Box<dyn Error>>> {
    async move {
        let response = match args {
            CommandArgs::Single(Some(key), Some(value)) => {
                let mut db_write = db.write().await;
                db_write.insert(key, value);
                NetResponse {
                    action: NetActions::Command,
                    value: Some("OK".to_string().into()),
                    error: None,
                }
            }
            CommandArgs::Single(None, _) => NetResponse {
                action: NetActions::Error,
                value: None,
                error: Some("No key provided for insert.".to_string()),
            },
            CommandArgs::Single(_, None) => NetResponse {
                action: NetActions::Error,
                value: None,
                error: Some("No value provided for insert.".to_string()),
            },
            CommandArgs::Many(args) => {
                // A temp map is used, so we only need to lock the db once per write.
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
                        error: Option::from(insert_errors.join(", ")),
                    }
                }
            }
        };

        Ok(response)
    }
        .boxed()
}