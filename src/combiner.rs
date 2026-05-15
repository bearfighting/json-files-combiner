use std::path::Path;

use anyhow::Result;
use serde_json::{Map, Value};
use tokio::fs;

pub async fn combiner(paths: &[&Path]) -> Result<Value> {
    let mut map = Map::new();

    for path in paths {
        let file = fs::read_to_string(&path).await?;
        let value: Value = serde_json::from_str(&file)?;

        if let Value::Object(obj) = value {
            map.extend(obj);
        }
    }

    Ok(Value::Object(map))
}
