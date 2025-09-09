use rust_json_parser::{parse, Value};
use std::env;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let input = if args.len() > 1 {
        // Use command line argument as JSON input
        args[1].clone()
    } else {
        // Read from stdin
        let mut buffer = String::new();
        match io::stdin().read_to_string(&mut buffer) {
            Ok(_) => buffer,
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                std::process::exit(1);
            }
        }
    };

    match parse(&input) {
        Ok(value) => {
            println!("Successfully parsed JSON:");
            print_value(&value, 0);
        }
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            std::process::exit(1);
        }
    }
}

fn print_value(value: &Value, indent: usize) {
    let spaces = "  ".repeat(indent);
    
    match value {
        Value::Null => println!("{}null", spaces),
        Value::Bool(b) => println!("{}{}", spaces, b),
        Value::Number(n) => println!("{}{}", spaces, n),
        Value::String(s) => println!("{}\"{}\"", spaces, s),
        Value::Array(arr) => {
            println!("{}[", spaces);
            for (i, item) in arr.iter().enumerate() {
                print_value(item, indent + 1);
                if i < arr.len() - 1 {
                    print!(",");
                }
                println!();
            }
            println!("{}]", spaces);
        }
        Value::Object(obj) => {
            println!("{}{}", spaces, "{");
            let items: Vec<_> = obj.iter().collect();
            for (i, (key, val)) in items.iter().enumerate() {
                print!("{}\"{}\": ", "  ".repeat(indent + 1), key);
                print_value(val, 0);
                if i < items.len() - 1 {
                    print!(",");
                }
                println!();
            }
            println!("{}{}", spaces, "}");
        }
    }
}