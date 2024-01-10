use std::fs;
use serde_json;

pub fn parse_function_calls_json(json_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_content = fs::read_to_string(json_file_path)?;
    let function_calls: serde_json::Value = serde_json::from_str(&json_content)?;

    if let serde_json::Value::Object(obj) = function_calls {
        for (function, calls) in obj {
            println!("Function: {}", function);
            println!("Calls: {:?}", calls);
        }
    }

    Ok(())
}

