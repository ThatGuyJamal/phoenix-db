use futures::future::{BoxFuture, FutureExt};

use crate::protocol::{Database, DbValue};

pub fn lookup_command(args: Vec<String>, db: Database) -> BoxFuture<'static, String>
{
    async move {
        if args.len() != 1 {
            return "Usage: LOOKUP <key>".to_string();
        }

        match db.read().await.get(&args[0]) {
            Some(value) => format_db_value(value),
            None => "NONE".to_string(),
        }
    }
    .boxed()
}

fn format_db_value(value: &DbValue) -> String
{
    match value {
        DbValue::Integer(i) => i.to_string(),
        DbValue::Float(f) => f.to_string(),
        DbValue::Boolean(b) => b.to_string(),
        DbValue::Text(t) => format!("\"{}\"", t),
        DbValue::List(list) => {
            let formatted_items: Vec<String> = list.iter().map(|item| format_db_value(item)).collect();
            format!("[{}]", formatted_items.join(", "))
        }
        DbValue::Map(map) => {
            let formatted_entries: Vec<String> = map.iter().map(|(k, v)| format!("{}: {}", k, format_db_value(v))).collect();
            format!("{{{}}}", formatted_entries.join(", "))
        }
        DbValue::Void => "VOID".to_string(),
    }
}
