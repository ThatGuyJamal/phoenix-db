use std::io::Error;

use futures::future::BoxFuture;
use futures::FutureExt;

use crate::protocol::{Database, DbKey, DbValue, NetActions, NetResponse};

pub fn delete_command(
    key: Option<DbKey>,
    _value: Option<DbValue>,
    db: Database,
) -> BoxFuture<'static, Result<NetResponse, Error>>
{
    async move {
        let response = match key {
            Some(k) => {
                let mut db_write = db.write().await;
                if db_write.remove(&k).is_some() {
                    NetResponse {
                        action: NetActions::Command,
                        value: Some("OK".to_string().into()),
                        error: None,
                    }
                } else {
                    NetResponse {
                        action: NetActions::Error,
                        value: None,
                        error: Some(format!("Key '{}' not found.", k)),
                    }
                }
            }
            None => NetResponse {
                action: NetActions::Error,
                value: None,
                error: Some("No key provided for delete.".to_string()),
            },
        };

        Ok(response)
    }
    .boxed()
}
