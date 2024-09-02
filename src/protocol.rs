use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::RwLock;

/// The database engine
pub struct DbEngine
{
    /// The database connection
    pub connection: Database,
    /// Useful data for the database
    #[allow(dead_code)] // todo - use this
    pub metadata: DbMetadata,
}

pub type Database = Arc<RwLock<HashMap<DbKey, DbValue>>>;

/// Other information about data stored in the database
#[derive(PartialEq, Debug)]
pub struct DbMetadata
{
    version: String,
    username: String,
    password: String,
    debug_mode: bool,
}

impl Default for DbMetadata
{
    fn default() -> Self
    {
        DbMetadata {
            version: "0.0.1".to_string(),
            username: "root".to_string(),
            password: "admin".to_string(),
            debug_mode: false,
        }
    }
}

pub type DbKey = String;
pub type DbValue = Value;

/// The command struct that the tcp server expects to compute
#[derive(Serialize, Deserialize, Debug)]
pub struct NetCommand<'a>
{
    pub name: &'a str,
    pub keys: Option<Vec<&'a str>>,
    pub values: Option<Vec<DbValue>>,
}

/// The data sent back to a connected client after a command
#[derive(Serialize, Deserialize, Debug)]
pub struct NetResponse
{
    pub action: NetActions,
    pub value: Option<DbValue>,
    pub error: Option<String>,
}

/// Actions ran on the network
#[derive(Serialize, Deserialize, Debug)]
pub enum NetActions
{
    Command,
    Error,
}
