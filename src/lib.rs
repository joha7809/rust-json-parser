//! A JSON parser implementation in Rust for educational purposes.
//!
//! This library provides a simple JSON parser that demonstrates the core concepts
//! of lexical analysis and parsing. It's designed to be educational and easy to understand.
//!
//! # Example
//!
//! ```rust
//! use rust_json_parser::{parse, Value};
//!
//! let json = r#"{"name": "Alice", "age": 30}"#;
//! let value = parse(json).expect("Failed to parse JSON");
//! 
//! if let Value::Object(obj) = value {
//!     println!("Name: {:?}", obj.get("name"));
//! }
//! ```

pub mod error;
pub mod lexer;
pub mod parser;
pub mod value;

pub use error::{Error, Result};
pub use parser::parse;
pub use value::Value;