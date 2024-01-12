mod c_parse;
mod json_parse;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut function_to_find = None;
    let mut source_tree = None;
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "-f" => {
                if index + 1 < args.len() {
                    function_to_find = Some(&args[index + 1]);
                    index += 2;
                } else {
                    eprintln!("Please provide a function name after -f flag");
                    return;
                }
            }
            "-i" => {
                if index + 1 < args.len() {
                    source_tree = Some(&args[index + 1]);
                    index += 2;
                } else {
                    eprintln!("Please provide a source tree after -i flag");
                    return;
                }
            }
            _ => {
                eprintln!("Unknown option: {}", args[index]);
                return;
            }
        }
    }
    if source_tree.is_none() {
        println!("Usage: cparse -i <source_tree> -f <function_name>");
        println!("Options:");
        println!("  -i <source_tree>   Specify the source tree to parse");
        println!("  -f <function_name> Specify the function name to find callers of");
        return;
    }
    let path = source_tree.unwrap();

    // Call the function from c_parse.rs to generate the JSON file
    let mut json_output = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("function_calls.json")
        .expect("Failed to open or create file");
    c_parse::parse_source_tree(path, &mut json_output);

    // Call the function from json_parse.rs to parse the generated JSON file
    if let Some(function_name) = function_to_find {
        let function_calls_result = json_parse::parse_function_calls_json("function_calls.json");
        match function_calls_result {
            Ok(function_calls_json) => {
                let callers = json_parse::find_callers_of_function(function_name, &function_calls_json);
                println!("Callers of {}: {:?}", function_name, callers);
                println!("Callers count: {}", callers.len());
            }
            Err(err) => {
                eprintln!("Error parsing JSON: {}", err);
            }
        }
    } else {
        if let Err(err) = json_parse::parse_function_calls_json("function_calls.json") {
            eprintln!("Error parsing JSON: {}", err);
        }
    } 
}