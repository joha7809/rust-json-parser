# Rust JSON Parser

A JSON parser implementation in Rust for educational purposes. This project demonstrates the core concepts of lexical analysis and parsing through a clean, well-structured implementation.

## Features

- **Lexer**: Tokenizes JSON input into meaningful tokens
- **Parser**: Recursive descent parser that converts tokens into JSON values  
- **Value Types**: Complete JSON value representation (null, bool, number, string, array, object)
- **Error Handling**: Detailed error messages with position information
- **CLI**: Command-line interface for parsing JSON

## Project Structure

```
src/
├── lib.rs      # Library entry point and public API
├── main.rs     # CLI application
├── value.rs    # JSON value types and utility methods
├── error.rs    # Error types and handling
├── lexer.rs    # JSON tokenizer/lexer
└── parser.rs   # Recursive descent parser
```

## Usage

### As a Library

```rust
use rust_json_parser::{parse, Value};

let json = r#"{"name": "Alice", "age": 30}"#;
let value = parse(json).expect("Failed to parse JSON");

if let Value::Object(obj) = value {
    println!("Name: {:?}", obj.get("name"));
}
```

### As a CLI Tool

```bash
# Parse JSON from command line argument
cargo run -- '{"name": "Alice", "age": 30}'

# Parse JSON from stdin
echo '{"test": true}' | cargo run

# Build and run
cargo build
echo '[1, 2, 3]' | ./target/debug/rust-json-parser
```

## Testing

Run the test suite:

```bash
cargo test
```

## Educational Aspects

This implementation focuses on:

1. **Clear separation of concerns** between lexing and parsing
2. **Error handling** with meaningful error messages
3. **Recursive descent parsing** for nested structures
4. **Rust idioms** like pattern matching and Result types
5. **Comprehensive testing** of all components

## JSON Support

Currently supports all standard JSON features:
- ✅ Null values
- ✅ Boolean values (true/false)
- ✅ Numbers (integers and floats)
- ✅ Strings with basic escape sequences
- ✅ Arrays
- ✅ Objects
- ⚠️  Unicode escape sequences (planned)
- ⚠️  Scientific notation (basic support)

## License

MIT License - see LICENSE file for details.