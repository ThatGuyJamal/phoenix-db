use crate::commands::CommandArgs;
use crate::protocol::{Database, DbValue, NetActions, NetResponse};
use futures::future::BoxFuture;
use futures::FutureExt;
use std::error::Error;

pub fn delete_command(args: CommandArgs, db: Database) -> BoxFuture<'static, Result<NetResponse, Box<dyn Error>>> {
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