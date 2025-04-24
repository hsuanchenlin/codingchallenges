use clap::{Parser, Subcommand, ArgAction};
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::process;

#[derive(Parser)]
#[command(
    name = "JSON Parser",
    author = "Your Name",
    version = "1.0.0",
    about = "A JSON parser that validates JSON files",
    long_about = "A JSON parser implementation built in Rust, following the JSON specification."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to the JSON file to validate (if no subcommand is used)
    #[arg(required = false)]
    file: Option<String>,

    /// Print detailed error information when validation fails
    #[arg(short, long, action = ArgAction::SetTrue)]
    verbose: bool,

    /// Read JSON from a standard input instead of a file
    #[arg(short, long, action = ArgAction::SetTrue)]
    stdin: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a JSON file
    Validate {
        /// Path to the JSON file to validate
        file: String,
        
        /// Print detailed error information
        #[arg(short, long, action = ArgAction::SetTrue)]
        verbose: bool,
    },
    
    /// Format (pretty-print) a JSON file
    Format {
        /// Path to the JSON file to format
        file: String,
        
        /// Output file (defaults to stdout)
        #[arg(short, long)]
        output: Option<String>,
        
        /// Indentation spaces (default: 2)
        #[arg(short, long, default_value_t = 2)]
        indent: usize,
    },
}

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
    // Parse command-line arguments
    let cli = Cli::parse();

    // Process subcommands if present
    if let Some(command) = cli.command {
        match command {
            Commands::Validate { file, verbose } => {
                let content = read_from_file(&file);
                handle_validation(content, verbose);
            }
            
            Commands::Format { file, output, indent } => {
                let content = read_from_file(&file);
                match validate_json(&content) {
                    Ok(_) => {
                        // In a real implementation, you would format the JSON here
                        let formatted = format!("Formatted JSON with {} spaces indentation", indent);
                        
                        if let Some(output_file) = output {
                            match fs::write(&output_file, formatted) {
                                Ok(_) => println!("Formatted JSON written to '{}'", output_file),
                                Err(err) => {
                                    eprintln!("Error writing to file: {}", err);
                                    process::exit(1);
                                }
                            }
                        } else {
                            println!("{}", formatted);
                        }
                    }
                    Err(err) => {
                        eprintln!("Cannot format invalid JSON: {}", err);
                        process::exit(1);
                    }
                }
            }
        }
        return;
    }

    // If no subcommand, process the default validation
    let json_content = if cli.stdin {
        read_from_stdin()
    } else if let Some(file_path) = cli.file.as_deref() {
        read_from_file(file_path)
    } else {
        eprintln!("Error: Please provide a file path or use --stdin to read from standard input");
        eprintln!("Run with --help for usage information");
        process::exit(1);
    };

    handle_validation(json_content, cli.verbose);
}

fn handle_validation(content: String, verbose: bool) {
    match validate_json(&content) {
        Ok(_) => {
            println!("Valid JSON");
            process::exit(0);
        }
        Err(err) => {
            if verbose {
                eprintln!("Invalid JSON: {}", err);
            } else {
                eprintln!("Invalid JSON");
            }
            process::exit(1);
        }
    }
}

// Function to read content from a file
fn read_from_file(file_path: &str) -> String {
    let path = Path::new(file_path);
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", file_path, err);
            process::exit(1);
        }
    }
}

// Function to read content from stdin
fn read_from_stdin() -> String {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => buffer,
        Err(err) => {
            eprintln!("Error reading from stdin: {}", err);
            process::exit(1);
        }
    }
}

// Placeholder function for JSON validation
fn validate_json(content: &str) -> Result<(), String> {
    // Implement your JSON validation logic here
    // For now, just a simple check
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return Err("Empty JSON content".to_string());
    }
    
    // Basic validation for a JSON object or array
    let first_char = trimmed.chars().next();
    let last_char = trimmed.chars().last();
    
    match (first_char, last_char) {
        (Some('{'), Some('}')) => Ok(()),
        (Some('['), Some(']')) => Ok(()),
        _ => Err("JSON must be either an object or an array".to_string()),
    }
    
    // Note: Replace the above with your full JSON parser implementation
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
        let test_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("test");
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