use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::RwLock;
use tokio::time::Instant;

use crate::cli::Cli;

/// Represents the database engine, managing the connection and metadata.
#[derive(Debug)]
pub struct DbEngine
{
    /// The database connection, providing access to the data storage.
    pub connection: Database,
    /// The database configuration created on start up.
    #[allow(dead_code)]
    pub db_config: Cli,
}
/// Type alias for the database, using an `Arc<RwLock<HashMap<DbKey, DbValue>>>` to provide concurrent read/write access.
pub type Database = Arc<RwLock<HashMap<DbKey, DbValue>>>;

/// Type alias for the keys in the database, represented as strings.
pub type DbKey = String;

/// Type alias for the Json values
pub type JsonValue = Value;

/// A value stored in the database
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct DbValue
{
    /// Any data type that supports json
    pub value: JsonValue,
    /// When this data expires. If none, the data will need manual deletion.
    pub expires_in: Option<Duration>,
}

impl DbValue
{
    /// Serde cant deserialize Instants, so we use this to convert the duration to instant at runtime.
    pub fn expires_at(&self) -> Option<Instant>
    {
        self.expires_in.map(|duration| Instant::now() + duration)
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
    /// Optional list of data explorations
    pub ttls: Option<Vec<Duration>>,
}

/// Represents the response sent back to a client after processing a command.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NetResponse
{
    /// The action performed, indicating whether the command was successful or if there was an error.
    pub action: NetActions,
    /// Optional value returned by the command, if applicable.
    pub value: Option<JsonValue>,
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
