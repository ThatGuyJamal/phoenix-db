use std::time::Duration;

use tokio::time::{interval, Instant};

use crate::protocol::Database;

/// A background task that periodically cleans up expired entries in the database.
///
/// This function runs an infinite loop, using a configurable interval to determine how often
/// the cleanup should occur. During each iteration, it acquires a write lock on the database,
/// checks the expiration times of all entries, and removes those that have expired based on
/// their `expires_at` timestamp.
///
/// The task will continue running indefinitely, ensuring that expired entries are regularly
/// removed from the database without requiring manual intervention.
///
/// # Arguments
///
/// * `db` - A reference to the database instance (`Database`) that the cleanup task operates on.
/// * `check_interval` - The duration to wait between each cleanup iteration.
pub async fn cleanup_task(db: Database, check_interval: Duration)
{
    let mut interval = interval(check_interval);

    loop {
        interval.tick().await;

        let mut db = db.write().await;
        let now = Instant::now();

        db.retain(|_, v| match v.expires_at() {
            // Remove expired entries
            Some(expiry) if now >= expiry => false,
            // Keep non-expired entries
            _ => true,
        });
    }
}
