use clap::{Parser, ArgAction};
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::process;

mod json_value;
mod lexer;
mod parser;

use json_value::JsonValue;

/// A JSON parser that validates JSON files
#[derive(Parser)]
#[command(
    name = "JSON Parser",
    author = "Your Name",
    version = "1.0.0",
    about = "A JSON parser that validates JSON files",
    long_about = "A JSON parser implementation built in Rust, following the JSON specification."
)]
struct Args {
    /// Path to the JSON file to validate
    #[arg(required = false)]
    file: Option<String>,

    /// Print detailed error information when validation fails
    #[arg(short, long, action = ArgAction::SetTrue)]
    verbose: bool,

    /// Read JSON from standard input instead of a file
    #[arg(short, long, action = ArgAction::SetTrue)]
    stdin: bool,
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();
    
    // Determine whether to read from file or stdin
    let json_content = if args.stdin {
        // Read from stdin
        read_from_stdin()
    } else if let Some(file_path) = args.file.as_deref() {
        // Read from file
        read_from_file(file_path)
    } else {
        eprintln!("Error: Please provide a file path or use --stdin to read from standard input");
        process::exit(1);
    };

    // Validate the JSON content
    match validate_json(&json_content) {
        Ok(_) => {
            println!("Valid JSON");
            process::exit(0);
        }
        Err(err) => {
            if args.verbose {
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

// Function to validate JSON
fn validate_json(content: &str) -> Result<(), String> {
    let mut lexer = lexer::Lexer::new(content);
    let tokens = lexer.tokenize();

    if tokens.is_empty() {
        return Err("Empty JSON or only whitespace".to_string());
    }

    // Check if the first token is a valid starting token for JSON
    if tokens[0] != lexer::Token::OpenBrace && tokens[0] != lexer::Token::OpenBracket {
        return Err("JSON must start with { or [".to_string());
    }

    let mut parser = parser::Parser::new(tokens);
    match parser.parse() {
        Ok(result) => {
            // Ensure the result is an object or array
            match result {
                JsonValue::Object(_) | JsonValue::Array(_) => Ok(()),
                _ => Err("JSON must be either an object or an array".to_string()),
            }
        },
        Err(err) => Err(err.to_string()),
    }
}