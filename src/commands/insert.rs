use futures::future::{BoxFuture, FutureExt};

use crate::protocol::{Database, DbValue};

// todo return errors on bad input vs using default values
pub fn insert_command(args: Vec<String>, db: Database) -> BoxFuture<'static, String>
{
    async move {
        let key = &args[0];
        let value = &args[1];

        db.write().await.insert(key.to_string(), DbValue::Text(value.to_owned()));
        "OK".to_string()
    }
    .boxed()
}
