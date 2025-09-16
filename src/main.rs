use rust_json_parser::{lexer::Lexer, parser::parse_json};

// fn main() {
//     // let input = r#"{"a": "hello", "b": 123"#;
//     // match parse_json(input) {
//     //     Ok(json) => println!("Parsed JSON: {:?}", json),
//     //     Err(e) => eprintln!("Parse error: {}", e),
//     // }
//     let input = r#"
//     {
//         "user": {
//             "id": 123,
//             "name": "Alice",
//             "profile": {
//                 "email": "alice@example.com",
//                 "age": 30
//             }
//         },
//         "settings": {
//             "theme": "dark",
//             "fontSize": 14
//         }
//     }
//     "#;
//     let invalid_json: &str = r#"{
//     "user": {
//         "id": 123
//         "jame": "Alice"
//     }
// }"#;
//     // 2. Extra comma
//     let invalid_extra_comma: &str = r#"
//     {
//         "user": {
//             "id": 123,
//             "name": "Alice",
//         }
//     }
//     "#;
//
//     // 3. Double comma
//     let invalid_double_comma: &str = r#"
//     {
//         "user": {
//             "id": 123,,
//             "name": "Alice"
//         }
//     }
//     "#;
//
//     // 4. Unquoted key
//     let invalid_unquoted_key: &str = r#"
//     {
//         settings: {
//             "theme": "dark",
//             "fontSize": 14
//         }
//     }
//     "#;
//     let lexer = Lexer::new(invalid_json);
//     for token in lexer {
//         println!("{:?}", token);
//     }
//     match parse_json(invalid_json) {
//         Ok(json) => println!("Parsed JSON: {:?}", json),
//         Err(e) => e.pretty_print(invalid_json),
//     }
// }
//

fn main() {
    let test_cases = vec![
        // 1. Missing comma between object entries → parser error
        r#"{
            "user": {
                "id": 123
                "name": "Alice"
            }
        }"#,
        // 2. Extra comma at the end of an object → parser error
        r#"{
            "user": {
                "id": 123,
                "name": "Alice",
            }
        }"#,
        // 3. Double comma → parser/lexer error
        r#"{
            "user": {
                "id": 123,, 
                "name": "Alice"
            }
        }"#,
        // 4. Unquoted key → lexer error
        r#"{
            settings: {
                "theme": "dark",
                "fontSize": 14
            }
        }"#,
        // 5. Unclosed string → lexer error
        r#"{
            "user": {
                "id": 123,
                "name": "Alice
            }
        }"#,
        // 6. Unclosed object → parser error
        r#"{
            "user": {
                "id": 123,
                "name": "Alice""#,
        // 7. Invalid number → lexer error
        r#"{
            "value": -1.2e
        }"#,
        // 8. Completely empty JSON → parser should fail gracefully
        r#""#,
        // 9. Nested missing commas → parser error
        r#"{
            "user": {
                "id": 123
                "profile": {
                    "email": "alice@example.com"
                    "age": 30
                }
            }
        }"#,
    ];

    for (i, input) in test_cases.iter().enumerate() {
        println!("\n=== Test case {} ===", i + 1);

        // Test lexer separately
        let lexer = Lexer::new(input);
        // for token in lexer {
        //     match token {
        //         Ok(tok) => println!("Lexer token: {:?}", tok),
        //         Err(e) => {
        //             println!(
        //                 "Lexer error at line {}, col {}: {:?}",
        //                 e.line, e.column, e.kind
        //             );
        //             break; // Stop lexer on first error to avoid infinite loop
        //         }
        //     }
        // }

        // Test parser
        match parse_json(input) {
            Ok(json) => println!("Parser succeeded: {:?}", json),
            Err(e) => {
                e.pretty_print(input);
            }
        }
    }
}
