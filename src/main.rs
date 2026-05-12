use std::fs;

use serde_json::Map;

fn main()-> Result<(), Box<dyn std::error::Error>> {
    let file1 = fs::read_to_string("./testcases/test-1.json")?;
    let file2 = fs::read_to_string("./testcases/test-2.json")?;

    let value1 = serde_json::from_str(&file1);
    let value2 = serde_json::from_str(&file2);

    let mut map = Map::new();

    if let Value::Object(value1) = json1 {
        map.extend(value1);
    }

    if let Value::Object(value2) = json2 {
        map.extend(value2);
    }

    let result = Value::Object(map);

    println!("{}", serde_json::to_string_pretty(result));

    Ok(())
}
