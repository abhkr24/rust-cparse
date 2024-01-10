mod c_parse;
mod json_parse;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a repository to parse");
        return;
    }
    let path = &args[1];

    // Call the function from c_parse.rs to generate the JSON file
    let mut json_output = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("function_calls.json")
        .expect("Failed to open or create file");
    c_parse::parse_source_tree(path, &mut json_output);

    // Call the function from json_parse.rs to parse the generated JSON file
    if let Err(err) = json_parse::parse_function_calls_json("function_calls.json") {
        eprintln!("Error parsing JSON: {}", err);
    }
}