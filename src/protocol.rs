use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use futures::future::BoxFuture;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

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

/// Other information about data strored in the database
#[derive(PartialEq, Debug)]
pub struct DbMetadata
{
    version: String,
    username: String,
    password: String,
    debug_mode: bool,
}

/// The main database type
pub type Database = Arc<RwLock<HashMap<DbKey, DbValue>>>;

pub type DbKey = String;
pub type DbValue = usize;

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

// Command function type
type CommandFn = fn(Vec<String>, Database) -> BoxFuture<'static, String>;

// Static command lookup table
pub static COMMANDS: Lazy<HashMap<&'static str, CommandFn>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("INSERT", insert_command as CommandFn);
    map.insert("LOOKUP", lookup_command as CommandFn);
    // Add more commands here
    map
});
