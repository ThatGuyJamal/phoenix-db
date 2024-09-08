use std::sync::Arc;
use std::time::Duration;

use crate::protocol::DbEngine;

pub mod tcp;
pub mod ttl;

pub async fn execute(engine: Arc<DbEngine>) -> Result<(), Box<dyn std::error::Error>>
{
    // Manages TTL key clean-up
    tokio::spawn(async move {
        ttl::execute(engine.connection.clone(), Duration::from_secs(60)).await;
    });

    Ok(())
}
