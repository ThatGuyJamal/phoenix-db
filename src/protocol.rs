use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Error;
use std::sync::Arc;

use futures::future::BoxFuture;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::RwLock;

use crate::commands::delete::delete_command;
use crate::commands::insert::insert_command;
use crate::commands::lookup::lookup_command;

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
    pub key: Option<&'a str>,
    pub value: Option<DbValue>,
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

// Command function type
type CommandFn = fn(Option<DbKey>, Option<DbValue>, Database) -> BoxFuture<'static, Result<NetResponse, Error>>;

// Static command lookup table
pub static COMMANDS: Lazy<HashMap<&'static str, CommandFn>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("INSERT", insert_command as CommandFn);
    map.insert("LOOKUP", lookup_command as CommandFn);
    map.insert("DELETE", delete_command as CommandFn);
    // Add more commands here
    map
});
