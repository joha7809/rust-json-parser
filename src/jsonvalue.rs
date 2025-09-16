use std::collections::HashMap;

#[derive(Debug)]
pub enum JSONValue {
    Array(Vec<JSONValue>),
    /// Classic boolean - true, false
    Bool(bool),
    /// Used to represent null value in JSON
    Null,
    /// floating point number for decimal numbers in JSON
    Number(f64),
    Object(HashMap<String, JSONValue>),
    String(String),
}

impl JSONValue {
    /// Returns `true` if the jsonvalue is [`Array`].
    ///
    /// [`Array`]: JSONValue::Array
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(..))
    }

    /// Returns `true` if the jsonvalue is [`Bool`].
    ///
    /// [`Bool`]: JSONValue::Bool
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(..))
    }

    /// Returns `true` if the jsonvalue is [`Null`].
    ///
    /// [`Null`]: JSONValue::Null
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Returns `true` if the jsonvalue is [`Number`].
    ///
    /// [`Number`]: JSONValue::Number
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(..))
    }

    /// Returns `true` if the jsonvalue is [`Object`].
    ///
    /// [`Object`]: JSONValue::Object
    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object(..))
    }

    /// Returns `true` if the jsonvalue is [`String`].
    ///
    /// [`String`]: JSONValue::String
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(..))
    }
}
