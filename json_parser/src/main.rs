use std::env;
use std::fs;
use std::process;

mod lexer;
mod parser;
mod json_value;

use json_value::JsonValue;
use parser::ParseError;

fn parse_json(input: &str) -> Result<JsonValue, ParseError> {
    let mut lexer = lexer::Lexer::new(input);
    let tokens = lexer.tokenize();

    if tokens.is_empty() {
        return Err(ParseError::InvalidJson);
    }

    // Check if the first token is a valid starting token for JSON (object or array)
    if tokens[0] != lexer::Token::OpenBrace && tokens[0] != lexer::Token::OpenBracket {
        return Err(ParseError::InvalidJson);
    }

    let mut parser = parser::Parser::new(tokens);
    let result = parser.parse()?;

    // Double-check the result just to be safe
    match result {
        JsonValue::Object(_) | JsonValue::Array(_) => Ok(result),
        _ => Err(ParseError::InvalidJson),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./json_parser <file>");
        process::exit(1);
    }

    let file_path = &args[1];
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading file: {}", e);
            process::exit(1);
        }
    };

    match parse_json(&content) {
        Ok(_) => {
            println!("Valid JSON");
            process::exit(0);
        },
        Err(e) => {
            println!("Invalid JSON: {:?}", e);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_object() {
        let result = parse_json("{}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_string_key_value() {
        let result = parse_json("{\"key\": \"value\"}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_types() {
        let result = parse_json("{\"key1\": true, \"key2\": false, \"key3\": null, \"key4\": \"value\", \"key5\": 101}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_nested_structures() {
        let result = parse_json("{\"key\": \"value\", \"key-n\": 101, \"key-o\": {}, \"key-l\": []}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_complex_json() {
        let json = r#"
        {
            "string": "Hello World",
            "number": 42,
            "boolean": true,
            "null": null,
            "array": [1, 2, 3, "four", null, true, {"nested": "object"}],
            "object": {
                "nested": "value",
                "another": 123
            }
        }
        "#;
        let result = parse_json(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_json() {
        let result = parse_json("{");
        assert!(result.is_err());

        let result = parse_json("}{");
        assert!(result.is_err());

        let result = parse_json("{\"key\": \"unclosed string}");
        assert!(result.is_err());

        let result = parse_json("true");
        assert!(result.is_err());

        let result = parse_json("123");
        assert!(result.is_err());

        let result = parse_json("\"string\"");
        assert!(result.is_err());

        let result = parse_json("null");
        assert!(result.is_err());
    }

    #[test]
    fn test_all_json_files() {
        let test_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test");
        for entry in fs::read_dir(&test_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let content = fs::read_to_string(&path).unwrap();
                let res = parse_json(&content);
                println!("{}: {:?}", file_name, res);
                if file_name.starts_with("pass") {
                    assert!(res.is_ok(), "{} should be valid JSON", file_name);
                } else if file_name.starts_with("fail") {
                    assert!(res.is_err(), "{} should be invalid JSON", file_name);
                }
            }
        }
    }
}