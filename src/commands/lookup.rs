use std::io::Error;

use futures::future::{BoxFuture, FutureExt};

use crate::protocol::{Database, DbKey, DbValue, NetActions, NetResponse};

pub fn lookup_command(
    key: Option<DbKey>,
    _value: Option<DbValue>,
    db: Database,
) -> BoxFuture<'static, Result<NetResponse, Error>>
{
    async move {
        let response = match key {
            Some(k) => {
                let db_read = db.read().await;
                match db_read.get(&k) {
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
            None => NetResponse {
                action: NetActions::Error,
                value: None,
                error: Some("No key provided for lookup.".to_string()),
            },
        };

        Ok(response)
    }
    .boxed()
}
