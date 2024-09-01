use crate::protocol::{Database, DbKey, DbValue, NetResponse};
use futures::future::{BoxFuture, FutureExt};
use std::io::Error;

pub fn lookup_command(key: Option<DbKey>, _value: Option<DbValue>, db: Database) -> BoxFuture<'static, Result<NetResponse, Error>> {
    async move {
        let response = match key {
            Some(k) => {
                let db_read = db.read().await;
                match db_read.get(&k) {
                    Some(value) => NetResponse {
                        value: Some(value.to_owned()),
                        error: None,
                    },
                    None => NetResponse {
                        value: None,
                        error: Some(format!("Key '{}' not found.", k)),
                    },
                }
            }
            None => NetResponse {
                value: None,
                error: Some("No key provided for lookup.".to_string()),
            },
        };

        Ok(response)
    }
        .boxed()
}
