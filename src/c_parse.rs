use std::fs;
use regex::Regex;
use glob::glob;
use std::env;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;



fn parse_functions(file_path: &str) -> Vec<String> {
    /// Parses the functions from the given file path and returns a vector of function names.
    /// 
    /// # Arguments
    /// 
    /// * `file_path` - A string slice that holds the path of the file to parse.
    /// 
    /// # Returns
    /// 
    /// A vector of function names parsed from the file.
    let content = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let re = Regex::new(r"(?:void|int|char|double|float|bool)\s+(\w+)\s*\(.*?\)\s*\{").unwrap();
    let functions = re.captures_iter(&content)
        .map(|cap| cap[1].to_string())
        .collect();
    
    functions
}

fn parse_macros(file_path: &str) -> Vec<String> {
    /// Parses the macros from the given file path and returns a vector of macro names.
    /// 
    /// # Arguments
    /// 
    /// * `file_path` - A string slice that holds the path of the file to parse.
    /// 
    /// # Returns
    /// 
    /// A vector of macro names parsed from the file.
    let content = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let re = Regex::new(r"#define\s+(\w+)\s+(.*)").unwrap();
    let macros = re.captures_iter(&content)
        .map(|cap| cap[1].to_string())
        .collect();
    
    macros
}


fn parse_functions_and_calls(file_path: &str) -> HashMap<String, Vec<String>> {
    let content = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    // Regex to match function definitions.
    let func_def_re = Regex::new(r"(?:void|int|char|double|float|bool)\s+(\w+)\s*\(.*?\)\s*\{").unwrap();
    // Regex to match function calls within a function body.
    let func_call_re = Regex::new(r"\b(\w+)\s*\(").unwrap();

    let mut function_calls_map = HashMap::new();

    // Iterate over function definitions.
    for cap in func_def_re.captures_iter(&content) {
        let func_name = cap[1].to_string();
        let start_index = cap.get(0).unwrap().end();
        let rest_of_content = &content[start_index..];
        // Find the matching closing brace of the function.
        let mut brace_count = 1;
        let mut end_index = start_index;
        for c in rest_of_content.chars() {
            match c {
                '{' => brace_count += 1,
                '}' => brace_count -= 1,
                _ => {}
            }
            end_index += c.len_utf8();
            if brace_count == 0 {
                break;
            }
        }

        // Extract the function body.
        let function_body = &content[start_index..end_index - 1];

        // Find all function calls within the function body.
        let mut calls = Vec::new();
        for call_cap in func_call_re.captures_iter(function_body) {
            calls.push(call_cap[1].to_string());
        }

        // Insert the function calls into the hashmap.
        function_calls_map.insert(func_name, calls);
    }

    function_calls_map
}


pub fn parse_source_tree(source_path: &str, json_output: &mut std::fs::File) {
    /// Parses the source tree and creates a JSON file that maps functions to their respective function calls within the function bodies.
    /// 
    /// # Arguments
    /// * `source_path` - The path to the source tree
    /// * `json_output` - A mutable reference to the JSON output file
    let c_files: Vec<_> = glob(&format!("{}/**/*.c", source_path)).expect("Failed to read glob pattern").collect();
    
    let mut all_functions: Vec<String> = Vec::new();
    for c_file in &c_files {
        let path_str = c_file.as_ref().expect("Failed to read glob pattern").to_str().expect("Failed to convert path to str");
        let mut functions = parse_functions(&path_str);
        all_functions.append(&mut functions);
    }
    let mut json_data = serde_json::Map::new();
    for c_file in &c_files {
        // First, parse all files to obtain a list of all function definitions.
        // Then, parse again and list only the calls made to the functions defined by us.
        // This will help to skip library functions.
        let path_str = c_file.as_ref().expect("Failed to read glob pattern").to_str().expect("Failed to convert path to str");
        let functions_calls = parse_functions_and_calls(&path_str);
        for (&ref func, calls) in &functions_calls {
            let mut func_calls = Vec::new();
            for call in calls {
                if all_functions.contains(&call) {
                    func_calls.push(serde_json::Value::String(call.to_string()));
                }
            }
            json_data.insert((&func).to_string(), serde_json::Value::Array(func_calls));
        }
    }
    let json_string = serde_json::to_string_pretty(&json_data).expect("Failed to serialize to JSON");
    std::io::Write::write_all(json_output, json_string.as_bytes()).expect("Failed to write to file");
    writeln!(json_output).expect("Failed to write newline to file"); // Add a new line
}

