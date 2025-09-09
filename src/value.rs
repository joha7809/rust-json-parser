//! JSON value types.
//!
//! This module defines the core data structures for representing JSON values.

use std::collections::HashMap;

/// Represents a JSON value.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// A JSON null value.
    Null,
    /// A JSON boolean value.
    Bool(bool),
    /// A JSON number value (stored as f64 for simplicity).
    Number(f64),
    /// A JSON string value.
    String(String),
    /// A JSON array value.
    Array(Vec<Value>),
    /// A JSON object value.
    Object(HashMap<String, Value>),
}

impl Value {
    /// Returns true if the value is null.
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Returns true if the value is a boolean.
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    /// Returns true if the value is a number.
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    /// Returns true if the value is a string.
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Returns true if the value is an array.
    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    /// Returns true if the value is an object.
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    /// Attempts to convert the value to a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Attempts to convert the value to a number.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Attempts to convert the value to a string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Attempts to convert the value to an array.
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Attempts to convert the value to an object.
    pub fn as_object(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Object(obj) => Some(obj),
            _ => None,
        }
    }
}