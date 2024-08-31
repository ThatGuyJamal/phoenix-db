use futures::future::{BoxFuture, FutureExt};

use crate::protocol::Database;

// todo return errors on bad input vs using default values
pub fn insert_command(_args: Vec<String>, _db: Database) -> BoxFuture<'static, String>
{
    async move { todo!("Implement insert command") }.boxed()
}
