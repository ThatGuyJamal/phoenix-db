use std::error::Error;

use futures::future::{BoxFuture, FutureExt};

use crate::commands::CommandArgs;
use crate::protocol::{Database, JsonValue, NetActions, NetResponse};

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
            CommandArgs::Single(Some(key), _, ..) => {
                let db_read = db.read().await;
                match db_read.get(&key) {
                    Some(data) => NetResponse {
                        action: NetActions::Command,
                        value: Some(data.value.to_owned()),
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
            CommandArgs::Single(None, _, ..) => NetResponse {
                action: NetActions::Error,
                value: None,
                error: Some("No key provided for lookup.".to_string()),
            },
            // Handle bulk lookup
            CommandArgs::Many(pairs) => {
                let db_read = db.read().await;
                let mut results = Vec::new();

                for pair in pairs {
                    if let Some(key) = pair.key {
                        if let Some(data) = db_read.get(&key) {
                            results.push(data.value.to_owned());
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
                    value: Some(JsonValue::Array(results)),
                    error: None,
                }
            }
        };

        Ok(response)
    }
    .boxed()
}

#[cfg(test)]
mod test
{
    use std::collections::HashMap;
    use std::sync::Arc;

    use serde_json::json;
    use tokio::sync::RwLock;

    use super::*;
    use crate::protocol::DbValue;

    // Helper function to create a new in-memory database
    fn create_fake_db() -> Database
    {
        Arc::new(RwLock::new(HashMap::new()))
    }

    #[tokio::test]
    async fn test_single_lookup_existing_key()
    {
        let db = create_fake_db();
        let key = "test_key".to_string();
        let data = DbValue {
            value: json!("test_value"),
            expires_in: None,
        };

        {
            let mut db_write = db.write().await;
            db_write.insert(key.clone(), data.clone());
        }

        let args = CommandArgs::Single(Some(key.clone()), None);
        let response = lookup_command(args, db.clone()).await.unwrap();

        // Check that the response indicates success and returns the correct value
        assert_eq!(response.action, NetActions::Command);
        assert_eq!(response.value, Some(data.value));
        assert!(response.error.is_none());
    }

    #[tokio::test]
    async fn test_single_lookup_missing_key()
    {
        let db = create_fake_db();
        let key = "non_existent_key".to_string();

        let args = CommandArgs::Single(Some(key), None);
        let response = lookup_command(args, db.clone()).await.unwrap();

        // Check that the response indicates success but with no value
        assert_eq!(response.action, NetActions::Command);
        assert_eq!(response.value, None);
        assert!(response.error.is_none());
    }

    #[tokio::test]
    async fn test_single_lookup_no_key_provided()
    {
        let db = create_fake_db();
        let args = CommandArgs::Single(None, None);
        let response = lookup_command(args, db.clone()).await.unwrap();

        // Check that the response indicates an error due to missing key
        assert_eq!(response.action, NetActions::Error);
        assert_eq!(response.value, None);
        assert_eq!(response.error, Some("No key provided for lookup.".to_string()));
    }

    #[tokio::test]
    async fn test_bulk_lookup()
    {
        let db = create_fake_db();
        let key1 = "key1".to_string();
        let key2 = "key2".to_string();
        let value1 = DbValue {
            value: json!("value1"),
            expires_in: None,
        };

        let value2 = DbValue {
            value: json!("value2"),
            expires_in: None,
        };

        {
            let mut db_write = db.write().await;
            db_write.insert(key1.clone(), value1.clone());
            db_write.insert(key2.clone(), value2.clone());
        }

        let args = CommandArgs::Many(vec![
            crate::commands::CommandParams {
                key: Some(key1.clone()),
                value: None,
                ttl: None,
            },
            crate::commands::CommandParams {
                key: Some(key2.clone()),
                value: None,
                ttl: None,
            },
        ]);

        let response = lookup_command(args, db.clone()).await.unwrap();

        // Check that the response indicates success and returns the correct values
        assert_eq!(response.action, NetActions::Command);
        assert_eq!(response.value, Some(JsonValue::Array(vec![value1.value, value2.value])));
        assert!(response.error.is_none());
    }

    #[tokio::test]
    async fn test_bulk_lookup_missing_keys()
    {
        let db = create_fake_db();
        let key1 = "key1".to_string();
        let value1 = DbValue {
            value: json!("value1"),
            expires_in: None,
        };

        {
            let mut db_write = db.write().await;
            db_write.insert(key1.clone(), value1.clone());
        }

        let args = CommandArgs::Many(vec![
            crate::commands::CommandParams {
                key: Some(key1.clone()),
                value: None,
                ttl: None,
            },
            crate::commands::CommandParams {
                key: None,
                value: None,
                ttl: None,
            },
        ]);

        let response = lookup_command(args, db.clone()).await.unwrap();

        // Check that the response indicates an error due to missing key
        assert_eq!(response.action, NetActions::Error);
        assert_eq!(response.value, None);
        assert_eq!(response.error, Some("Missing key in bulk lookup.".to_string()));

        // Check that only valid lookups were successful
        let expected_value = JsonValue::Array(vec![value1.value]);
        let response = lookup_command(
            CommandArgs::Many(vec![crate::commands::CommandParams {
                key: Some(key1),
                value: None,
                ttl: None,
            }]),
            db.clone(),
        )
        .await
        .unwrap();

        assert_eq!(response.value, Some(expected_value));
    }
}
