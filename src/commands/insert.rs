use futures::future::{BoxFuture, FutureExt};

use crate::protocol::Database;

pub fn insert_command(args: Vec<String>, db: Database) -> BoxFuture<'static, String>
{
    async move {
        if args.len() != 2 {
            return "Usage: INSERT <key> <value>".to_string();
        }
        let key = args[0].clone();
        let value = args[1].parse::<usize>().unwrap_or(0);

        let mut db = db.write().await;
        db.insert(key, value);
        "OK".to_string()
    }
    .boxed()
}
