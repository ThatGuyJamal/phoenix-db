use std::io::Error;

use futures::future::{BoxFuture, FutureExt};

use crate::protocol::{Database, DbKey, DbValue, NetActions, NetResponse};

pub fn insert_command(
    key: Option<DbKey>,
    value: Option<DbValue>,
    db: Database,
) -> BoxFuture<'static, Result<NetResponse, Error>>
{
    async move {
        let response = match (key, value) {
            (Some(k), Some(v)) => {
                let mut db_write = db.write().await;
                db_write.insert(k, v);
                NetResponse {
                    action: NetActions::Command,
                    value: Some("OK".to_string().into()),
                    error: None,
                }
            }
            (None, _) => NetResponse {
                action: NetActions::Error,
                value: None,
                error: Some("No key provided for insert.".to_string()),
            },
            (_, None) => NetResponse {
                action: NetActions::Error,
                value: None,
                error: Some("No value provided for insert.".to_string()),
            },
        };

        Ok(response)
    }
    .boxed()
}
