use rust_json_parser::{parse, Value};
use std::collections::HashMap;

#[test]
fn test_parse_null() {
    let result = parse("null").unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_parse_bool() {
    let result = parse("true").unwrap();
    assert_eq!(result, Value::Bool(true));
    
    let result = parse("false").unwrap();
    assert_eq!(result, Value::Bool(false));
}

#[test]
fn test_parse_number() {
    let result = parse("42").unwrap();
    assert_eq!(result, Value::Number(42.0));
    
    let result = parse("3.14").unwrap();
    assert_eq!(result, Value::Number(3.14));
    
    let result = parse("-10").unwrap();
    assert_eq!(result, Value::Number(-10.0));
}

#[test]
fn test_parse_string() {
    let result = parse(r#""hello""#).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
    
    let result = parse(r#""hello world""#).unwrap();
    assert_eq!(result, Value::String("hello world".to_string()));
}

#[test]
fn test_parse_empty_array() {
    let result = parse("[]").unwrap();
    assert_eq!(result, Value::Array(vec![]));
}

#[test]
fn test_parse_array() {
    let result = parse(r#"[1, 2, 3]"#).unwrap();
    assert_eq!(result, Value::Array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]));
    
    let result = parse(r#"["a", "b", "c"]"#).unwrap();
    assert_eq!(result, Value::Array(vec![
        Value::String("a".to_string()),
        Value::String("b".to_string()),
        Value::String("c".to_string()),
    ]));
}

#[test]
fn test_parse_empty_object() {
    let result = parse("{}").unwrap();
    assert_eq!(result, Value::Object(HashMap::new()));
}

#[test]
fn test_parse_object() {
    let result = parse(r#"{"name": "Alice", "age": 30}"#).unwrap();
    
    let mut expected = HashMap::new();
    expected.insert("name".to_string(), Value::String("Alice".to_string()));
    expected.insert("age".to_string(), Value::Number(30.0));
    
    assert_eq!(result, Value::Object(expected));
}

#[test]
fn test_parse_nested_structure() {
    let json = r#"{
        "name": "Alice",
        "age": 30,
        "hobbies": ["reading", "coding"],
        "address": {
            "street": "123 Main St",
            "city": "Anytown"
        }
    }"#;
    
    let result = parse(json).unwrap();
    
    if let Value::Object(obj) = result {
        assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
        assert_eq!(obj.get("age"), Some(&Value::Number(30.0)));
        
        if let Some(Value::Array(hobbies)) = obj.get("hobbies") {
            assert_eq!(hobbies.len(), 2);
            assert_eq!(hobbies[0], Value::String("reading".to_string()));
            assert_eq!(hobbies[1], Value::String("coding".to_string()));
        } else {
            panic!("Expected hobbies to be an array");
        }
        
        if let Some(Value::Object(address)) = obj.get("address") {
            assert_eq!(address.get("street"), Some(&Value::String("123 Main St".to_string())));
            assert_eq!(address.get("city"), Some(&Value::String("Anytown".to_string())));
        } else {
            panic!("Expected address to be an object");
        }
    } else {
        panic!("Expected result to be an object");
    }
}

#[test]
fn test_value_methods() {
    let null_val = Value::Null;
    assert!(null_val.is_null());
    assert!(!null_val.is_bool());
    
    let bool_val = Value::Bool(true);
    assert!(bool_val.is_bool());
    assert_eq!(bool_val.as_bool(), Some(true));
    
    let num_val = Value::Number(42.0);
    assert!(num_val.is_number());
    assert_eq!(num_val.as_f64(), Some(42.0));
    
    let str_val = Value::String("hello".to_string());
    assert!(str_val.is_string());
    assert_eq!(str_val.as_str(), Some("hello"));
    
    let arr_val = Value::Array(vec![Value::Number(1.0)]);
    assert!(arr_val.is_array());
    assert!(arr_val.as_array().is_some());
    
    let obj_val = Value::Object(HashMap::new());
    assert!(obj_val.is_object());
    assert!(obj_val.as_object().is_some());
}