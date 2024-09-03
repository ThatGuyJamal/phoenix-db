use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::RwLock;

/// Represents the database engine, managing the connection and metadata.
pub struct DbEngine<'a>
{
    /// The database connection, providing access to the data storage.
    pub connection: Database,
    /// Metadata related to the database, such as version and credentials.
    #[allow(dead_code)] // TODO: Use this field in the future.
    pub metadata: DbMetadata<'a>,
}

/// Type alias for the database, using an `Arc<RwLock<HashMap<DbKey, DbValue>>>` to provide concurrent read/write access.
pub type Database = Arc<RwLock<HashMap<DbKey, DbValue>>>;

/// Type alias for the keys in the database, represented as strings.
pub type DbKey = String;

/// Type alias for the values in the database, represented as JSON values.
pub type DbValue = Value;

/// Represents metadata about the database, including version and credentials.
#[derive(PartialEq, Debug)]
pub struct DbMetadata<'a>
{
    /// The version of the database.
    pub version: &'a str,
    /// The port used for connecting to the database.
    pub port: u16,
    /// The remote address to start the server on.
    pub addr: &'a str,
    /// The username used for accessing the database.
    pub username: &'a str,
    /// The password used for accessing the database.
    pub password: &'a str,
    /// Flag indicating whether debug mode is enabled.
    pub debug_mode: bool,
}

impl Default for DbMetadata<'_>
{
    /// Provides a default instance of `DbMetadata` with initial values.
    fn default() -> Self
    {
        DbMetadata {
            version: "0.1.0",
            port: 6969,
            addr: "127.0.0.1",
            username: "root",
            password: "admin",
            debug_mode: false,
        }
    }
}

/// Represents a command sent over the network to be processed by the server.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NetCommand<'a>
{
    /// The name of the command.
    pub name: &'a str,
    /// Optional list of keys associated with the command.
    pub keys: Option<Vec<&'a str>>,
    /// Optional list of values associated with the command.
    pub values: Option<Vec<DbValue>>,
}

/// Represents the response sent back to a client after processing a command.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NetResponse
{
    /// The action performed, indicating whether the command was successful or if there was an error.
    pub action: NetActions,
    /// Optional value returned by the command, if applicable.
    pub value: Option<DbValue>,
    /// Optional error message, if an error occurred during command processing.
    pub error: Option<String>,
}

/// Enum representing possible network actions in response to commands.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum NetActions
{
    /// Indicates that a command was processed successfully.
    Command,
    /// Indicates that an error occurred while processing a command.
    Error,
}
