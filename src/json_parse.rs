use std::fs;
use serde_json;


pub fn find_callers_of_function<'a>(function_name: &'a str, function_calls: &'a serde_json::Value) -> Vec<&'a String> {
    let mut callers = Vec::new();
    if let serde_json::Value::Object(obj) = function_calls {
        for (function, calls) in obj {
            if let serde_json::Value::Array(calls_array) = calls {
                if calls_array.iter().any(|call| call == function_name) {
                    callers.push(function);
                }
            }
        }
    }
    callers
}

pub fn parse_function_calls_json(json_file_path: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let json_content = fs::read_to_string(json_file_path)?;
    let function_calls: serde_json::Value = serde_json::from_str(&json_content)?;
    Ok(function_calls)
}

