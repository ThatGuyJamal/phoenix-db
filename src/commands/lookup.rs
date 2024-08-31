use futures::future::{BoxFuture, FutureExt};

use crate::protocol::Database;

pub fn lookup_command(args: Vec<String>, db: Database) -> BoxFuture<'static, String>
{
    async move {
        if args.len() != 1 {
            return "Usage: LOOKUP <key>".to_string();
        }
        let key = args[0].clone();

        let db = db.read().await;
        match db.get(&key) {
            Some(value) => format!("{}", value),
            None => format!("NONE"),
        }
    }
    .boxed()
}
