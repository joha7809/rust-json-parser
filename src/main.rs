use rust_json_parser::parser::parse_json;

fn main() {
    // test a specific file and print result
    let path = "tests/big.json";
    let input = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to read test num {}", path));
    match parse_json(&input) {
        Ok(_) => {}
        Err(e) => {
            println!("Parser failed for {}:", path);
            e.pretty_print(&input); // Print error with context
        }
    }
}

#[test]
fn test_valid_json_files() {
    use std::fs;

    for i in 1..=44 {
        let path = format!("tests/valid{}.json", i);
        let input = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read test num {}: {}", i, path));
        let result = parse_json(&input); // Adjust function path as needed
        assert!(
            result.is_ok(),
            "Should parse {} successfully, got {:?}",
            path,
            result
        );
    }
}

#[test]
fn test_invalid_json_files_by_chat() {
    use std::fs;

    for i in 1..=41 {
        let path = format!("tests/invalid{}.json", i);
        let input = fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read {}", path));

        let result = parse_json(&input); // Adjust function path as needed
        assert!(result.is_err(), "Should fail to parse {}", path);
    }
}

#[test]
fn test_invalid_json_files() {
    use std::fs;

    for i in 1..=33 {
        let path = format!("tests/fail{}.json", i);
        let input = fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read {}", path));

        let result = parse_json(&input); // Adjust function path as needed
        assert!(result.is_err(), "Should fail to parse {}", path);
    }
}
