# rust-json-parser

A JSON parser implementation written in Rust, created as part of the [Coding Challenges](https://codingchallenges.fyi) series. This project was built for fun and to learn Rust while implementing a complete JSON parser from scratch.

## Features

- **Complete JSON Support**: Parses all valid JSON types including objects, arrays, strings, numbers, booleans, and null
- **Lexer & Parser Architecture**: Clean separation between lexical analysis and parsing
- **Comprehensive Error Handling**: Provides detailed error messages with line and column information
- **Pretty Error Printing**: Visualizes parsing errors with context from the input
- **Extensive Testing**: Includes 44 valid and 74 invalid JSON test cases

## Installation

### Prerequisites

- Rust 1.70 or higher (with the 2024 edition)
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/joha7809/rust-json-parser.git
cd rust-json-parser

# Build the project
cargo build --release

# Run tests
cargo test

# Run the example
cargo run
```

## Usage

### As a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-json-parser = { git = "https://github.com/joha7809/rust-json-parser" }
```

Then use it in your code:

```rust
use rust_json_parser::parse_json;

fn main() {
    let json_input = r#"{
        "name": "John Doe",
        "age": 30,
        "is_student": false,
        "courses": ["Math", "Science"],
        "address": null
    }"#;

    match parse_json(json_input) {
        Ok(value) => println!("Successfully parsed: {:?}", value),
        Err(e) => {
            eprintln!("Parse error:");
            e.pretty_print(json_input);
        }
    }
}
```

### As a Command-Line Tool

The main binary parses a test file and displays any errors:

```bash
cargo run
```

By default, it parses `tests/big.json`. You can modify the `main.rs` file to parse different files.

## Project Structure

```
rust-json-parser/
├── src/
│   ├── main.rs        # Entry point with example usage
│   ├── lib.rs         # Library root
│   ├── lexer.rs       # Lexical analyzer (tokenizer)
│   ├── parser.rs      # JSON parser
│   ├── jsonvalue.rs   # JSON value types and token definitions
│   └── errors.rs      # Error types and pretty printing
├── tests/             # JSON test files (valid and invalid)
├── Cargo.toml         # Project configuration
├── LICENSE            # MIT License
└── README.md          # This file
```

### Architecture

The parser is built with a classic two-stage architecture:

1. **Lexer** (`lexer.rs`): Converts raw JSON text into a stream of tokens
   - Handles strings with escape sequences
   - Parses numbers including scientific notation
   - Tracks line and column positions for error reporting

2. **Parser** (`parser.rs`): Consumes tokens to build a JSON value tree
   - Recursive descent parsing
   - Validates JSON structure
   - Produces detailed error messages

3. **JSON Values** (`jsonvalue.rs`): Represents the parsed JSON structure
   - Objects (HashMap)
   - Arrays (Vec)
   - Strings, Numbers, Booleans, and Null

## Testing

The project includes extensive tests:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_valid_json_files
```

### Test Coverage

- **44 valid JSON test files** (`tests/valid*.json`)
- **33 invalid JSON test files** (`tests/fail*.json`)
- **41 additional invalid JSON test files** (`tests/invalid*.json`)
- Unit tests for lexer and parser components

## Error Handling

The parser provides detailed error messages with context:

```
Parse error at line 3, column 15: Expected ',' or '}' in object but found ]
   3 |     "name": "John"]
     |               ^
```

Supported error types:
- Unexpected characters
- Unclosed strings
- Invalid numbers (leading zeros, malformed decimals/exponents)
- Invalid escape sequences
- Unexpected end of file
- Structural errors (missing commas, brackets, etc.)

## Performance

This parser prioritizes correctness and learning over performance. It:
- Uses `HashMap` for objects (ordering not preserved)
- Stores numbers as `f64`
- Allocates strings rather than using string slices

Potential optimizations noted in code comments for future improvements.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built as part of [Coding Challenges](https://codingchallenges.fyi)
- Created for learning Rust and understanding parser implementation
- Test cases derived from various JSON test suites

## Author

Johannes Sigvardsen

## Contributing

This is primarily a learning project, but feel free to open issues or submit pull requests if you find bugs or have suggestions for improvements!