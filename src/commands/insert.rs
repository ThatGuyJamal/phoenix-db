use crate::protocol::{Database, DbKey, DbValue, NetResponse};
use futures::future::{BoxFuture, FutureExt};
use std::io::Error;

pub fn insert_command(key: Option<DbKey>, value: Option<DbValue>, db: Database) -> BoxFuture<'static, Result<NetResponse, Error>> {
    async move {
        let response = match (key, value) {
            (Some(k), Some(v)) => {
                let mut db_write = db.write().await;
                db_write.insert(k.clone(), v);
                NetResponse {
                    value: Some(format!("Successfully inserted key '{}'.", k).into()),
                    error: None,
                }
            }
            (None, _) => NetResponse {
                value: None,
                error: Some("No key provided for insert.".to_string()),
            },
            (_, None) => NetResponse {
                value: None,
                error: Some("No value provided for insert.".to_string()),
            },
        };

        Ok(response)
    }
        .boxed()
}
