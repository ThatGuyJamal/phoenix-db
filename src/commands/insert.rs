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
            CommandArgs::Single(Some(key), Some(value), ..) => {
                let mut db_write = db.write().await;
                db_write.insert(key, value);
                NetResponse {
                    action: NetActions::Command,
                    value: Some("OK".to_string().into()),
                    error: None,
                }
            }
            // Handle case where no key is provided
            CommandArgs::Single(None, _, ..) => NetResponse {
                action: NetActions::Error,
                value: None,
                error: Some("No key provided for insert.".to_string()),
            },
            // Handle case where no value is provided
            CommandArgs::Single(_, None, ..) => NetResponse {
                action: NetActions::Error,
                value: None,
                error: Some("No value provided for insert.".to_string()),
            },
            // Handle bulk insertions
            CommandArgs::Many(args) => {
                let mut temp_map: HashMap<DbKey, DbValue> = HashMap::new();
                let mut insert_errors: Vec<String> = Vec::new();

                for a in args {
                    match (a.key, a.value, a.ttl) {
                        (Some(key), Some(value), ..) => {
                            temp_map.insert(
                                key,
                                DbValue {
                                    value,
                                    expires_in: a.ttl,
                                },
                            );
                        }
                        (Some(key), None, ..) => {
                            insert_errors.push(format!("Missing value for key: {}", key));
                        }
                        (None, Some(_), ..) => {
                            insert_errors.push("Key is missing for provided value".to_string());
                        }
                        (None, None, ..) => {
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
                        action: NetActions::Error,
                        value: None,
                        error: Some(insert_errors.join(", ")),
                    }
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

    use crate::commands::insert::insert_command;
    use crate::commands::CommandArgs;
    use crate::protocol::{Database, DbValue, NetActions};

    // Helper function to create a new in-memory database
    fn create_fake_db() -> Database
    {
        Arc::new(RwLock::new(HashMap::new()))
    }

    #[tokio::test]
    async fn test_single_insert()
    {
        let db = create_fake_db();
        let key = "test_key".to_string();
        let data = DbValue {
            value: json!("test_value"),
            expires_in: None,
        };

        let args = CommandArgs::Single(Some(key.clone()), Some(data.clone()));
        let response = insert_command(args, db.clone()).await.unwrap();

        // Check that the response indicates success
        assert_eq!(response.action, NetActions::Command);
        assert_eq!(response.value, Some("OK".to_string().into()));
        assert!(response.error.is_none());

        // Check that the value was inserted correctly
        let db_read = db.read().await;
        assert_eq!(db_read.get(&key), Some(&data));
    }

    #[tokio::test]
    async fn test_single_insert_missing_key()
    {
        let db = create_fake_db();
        let data = DbValue {
            value: json!("test_value"),
            expires_in: None,
        };

        let args = CommandArgs::Single(None, Some(data));
        let response = insert_command(args, db.clone()).await.unwrap();

        // Check that the response indicates an error
        assert_eq!(response.action, NetActions::Error);
        assert_eq!(response.value, None);
        assert_eq!(response.error, Some("No key provided for insert.".to_string()));
    }

    #[tokio::test]
    async fn test_single_insert_missing_value()
    {
        let db = create_fake_db();
        let key = "test_key".to_string();

        let args = CommandArgs::Single(Some(key), None);
        let response = insert_command(args, db.clone()).await.unwrap();

        // Check that the response indicates an error
        assert_eq!(response.action, NetActions::Error);
        assert_eq!(response.value, None);
        assert_eq!(response.error, Some("No value provided for insert.".to_string()));
    }

    #[tokio::test]
    async fn test_bulk_insert()
    {
        let db = create_fake_db();
        let key1 = "key1".to_string();
        let key2 = "key2".to_string();
        let data = DbValue {
            value: json!("value1"),
            expires_in: None,
        };
        let data2 = DbValue {
            value: json!("value2"),
            expires_in: None,
        };

        let args = CommandArgs::Many(vec![
            crate::commands::CommandParams {
                key: Some(key1.clone()),
                value: Some(data.value.clone()),
                ttl: None,
            },
            crate::commands::CommandParams {
                key: Some(key2.clone()),
                value: Some(data2.value.clone()),
                ttl: None,
            },
        ]);

        let response = insert_command(args, db.clone()).await.unwrap();

        // Check that the response indicates success
        assert_eq!(response.action, NetActions::Command);
        assert_eq!(response.value, Some("OK".to_string().into()));
        assert!(response.error.is_none());

        // Check that the values were inserted correctly
        let db_read = db.read().await;
        assert_eq!(db_read.get(&key1), Some(&data));
        assert_eq!(db_read.get(&key2), Some(&data2));
    }
}
