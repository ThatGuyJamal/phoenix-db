use std::error::Error;

use futures::future::BoxFuture;
use futures::FutureExt;

use crate::commands::CommandArgs;
use crate::protocol::{Database, DbValue, NetActions, NetResponse};

/// Executes a delete command on the database.
///
/// This function handles both single key deletions and bulk deletions based on the provided `CommandArgs`.
/// It removes the specified key-value pairs from the database and returns a `NetResponse` indicating success or errors.
///
/// # Arguments
///
/// * `args` - The arguments for the command, which could be a single key or multiple keys for bulk deletion.
/// * `db` - The database instance used for deletion.
///
/// # Returns
///
/// A `BoxFuture` that resolves to a `Result` containing a `NetResponse`. The response indicates the success
/// or failure of the deletion operation.
pub fn delete_command(args: CommandArgs, db: Database) -> BoxFuture<'static, Result<NetResponse, Box<dyn Error + Send>>>
{
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
            // Returns the deleted keys
            CommandArgs::Many(pairs) => {
                let mut db_write = db.write().await;
                let mut results = vec![];
                for pair in pairs {
                    if let Some(key) = pair.key {
                        if db_write.remove(&key).is_some() {
                            results.push(key);
                        }
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

#[cfg(test)]
mod test
{
    use std::collections::HashMap;
    use std::sync::Arc;

    use tokio::sync::RwLock;

    use super::*;

    // Helper function to create a new in-memory database
    fn create_fake_db() -> Database
    {
        Arc::new(RwLock::new(HashMap::new()))
    }

    #[tokio::test]
    async fn test_single_delete_existing_key()
    {
        let db = create_fake_db();
        let key = "test_key".to_string();
        let value = DbValue::String("test_value".to_string());

        {
            let mut db_write = db.write().await;
            db_write.insert(key.clone(), value.clone());
        }

        let args = CommandArgs::Single(Some(key.clone()), None);
        let response = delete_command(args, db.clone()).await.unwrap();

        // Check that the response indicates success and the key is removed
        assert_eq!(response.action, NetActions::Command);
        assert_eq!(response.value, Some("OK".to_string().into()));
        assert!(response.error.is_none());

        let db_read = db.read().await;
        assert!(db_read.get(&key).is_none());
    }

    #[tokio::test]
    async fn test_single_delete_missing_key()
    {
        let db = create_fake_db();
        let key = "non_existent_key".to_string();

        let args = CommandArgs::Single(Some(key.clone()), None);
        let response = delete_command(args, db.clone()).await.unwrap();

        // Check that the response indicates an error due to the missing key
        assert_eq!(response.action, NetActions::Error);
        assert_eq!(response.value, None);
        assert_eq!(response.error, Some(format!("Key '{}' not found.", key)));
    }

    #[tokio::test]
    async fn test_single_delete_no_key_provided()
    {
        let db = create_fake_db();
        let args = CommandArgs::Single(None, None);
        let response = delete_command(args, db.clone()).await.unwrap();

        // Check that the response indicates an error due to missing key
        assert_eq!(response.action, NetActions::Error);
        assert_eq!(response.value, None);
        assert_eq!(response.error, Some("No key provided for delete.".to_string()));
    }

    #[tokio::test]
    async fn test_bulk_delete()
    {
        let db = create_fake_db();
        let key1 = "key1".to_string();
        let key2 = "key2".to_string();
        let value1 = DbValue::String("value1".to_string());
        let value2 = DbValue::String("value2".to_string());

        {
            let mut db_write = db.write().await;
            db_write.insert(key1.clone(), value1.clone());
            db_write.insert(key2.clone(), value2.clone());
        }

        let args = CommandArgs::Many(vec![
            crate::commands::ManyParams {
                key: Some(key1.clone()),
                value: None,
            },
            crate::commands::ManyParams {
                key: Some(key2.clone()),
                value: None,
            },
        ]);

        let response = delete_command(args, db.clone()).await.unwrap();

        // Check that the response indicates success and the keys are removed
        assert_eq!(response.action, NetActions::Command);
        assert_eq!(
            response.value,
            Some(DbValue::Array(vec![
                DbValue::String(key1.clone()),
                DbValue::String(key2.clone())
            ]))
        );
        assert!(response.error.is_none());

        let db_read = db.read().await;
        assert!(db_read.get(&key1).is_none());
        assert!(db_read.get(&key2).is_none());
    }

    #[tokio::test]
    async fn test_bulk_delete_missing_keys()
    {
        let db = create_fake_db();
        let key1 = "key1".to_string();
        let key2 = "key2".to_string();
        let value1 = DbValue::String("value1".to_string());

        {
            let mut db_write = db.write().await;
            db_write.insert(key1.clone(), value1.clone());
        }

        let args = CommandArgs::Many(vec![
            crate::commands::ManyParams {
                key: Some(key1.clone()),
                value: None,
            },
            crate::commands::ManyParams {
                key: Some(key2.clone()),
                value: None,
            },
        ]);

        let response = delete_command(args, db.clone()).await.unwrap();

        // Check that the response indicates success for the key that was deleted and error for the missing key
        assert_eq!(response.action, NetActions::Command);
        assert_eq!(response.value, Some(DbValue::Array(vec![DbValue::String(key1.clone()),])));
        assert!(response.error.is_none());

        let db_read = db.read().await;
        assert!(db_read.get(&key1).is_none());
        assert!(db_read.get(&key2).is_none()); // key2 was missing, so should still be absent
    }
}
