use crate::commands::CommandArgs;
use crate::protocol::{Database, DbValue, NetActions, NetResponse};
use futures::future::{BoxFuture, FutureExt};
use std::error::Error;

pub fn lookup_command(args: CommandArgs, db: Database) -> BoxFuture<'static, Result<NetResponse, Box<dyn Error>>> {
    async move {
        let response = match args {
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
            CommandArgs::Single(None, _) => NetResponse {
                action: NetActions::Error,
                value: None,
                error: Some("No key provided for lookup.".to_string()),
            },
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
