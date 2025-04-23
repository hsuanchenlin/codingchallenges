mod command_args;
#[cfg(test)]
mod test;
mod std_in;
// This will load tests.rs

use crate::command_args::Args;
use clap::{ArgAction, Parser};
use std::fs;
use std::path::Path;

// Include this at the top of your file
mod io_operations;
use io_operations::{StdinOperations, StdinReader};


// Modify your functions to return a string instead of printing directly
fn process_file(args: &Args, filename: &str, default_mode: bool) -> String {
    let path = Path::new(filename);
    
    match fs::read(path) {
        Ok(content) => {
            let content_str = String::from_utf8_lossy(&content);
            
            let mut output = String::new();
            
            if args.lines || default_mode {
                let lines = content_str.lines().count();
                output.push_str(&format!("{:8}", lines));
            }
            
            if args.words || default_mode {
                let words = content_str.split_whitespace().count();
                output.push_str(&format!("{:8}", words));
            }
            
            if args.bytes || default_mode {
                let bytes = content.len();
                output.push_str(&format!("{:8}", bytes));
            }
            
            if args.chars {
                let chars = content_str.chars().count();
                output.push_str(&format!("{:8}", chars));
            }
            
            output.push_str(&format!(" {}", filename));
            output
        },
        Err(_) => {
            format!("Could not read file: {}", filename)
        }
    }
}

// Replace the original process_stdin function with this version
fn process_stdin<T: StdinOperations>(args: &Args, default_mode: bool, mut stdin_reader: T) -> String {
    let mut bytes = Vec::new();
    
    // Read all stdin as bytes
    stdin_reader.read_to_end(&mut bytes).unwrap();
    
    let content = String::from_utf8_lossy(&bytes);
    
    let mut output = String::new();
    
    if args.lines || default_mode {
        let lines = content.lines().count();
        output.push_str(&format!("{:8}", lines));
    }
    
    if args.words || default_mode {
        let words = content.split_whitespace().count();
        output.push_str(&format!("{:8}", words));
    }
    
    if args.bytes || default_mode {
        let byte_count = bytes.len();
        output.push_str(&format!("{:8}", byte_count));
    }
    
    if args.chars {
        let chars = content.chars().count();
        output.push_str(&format!("{:8}", chars));
    }
    
    output
}

// In your main function:
fn main() {
    let args = Args::parse();
    
    // If no counting options are specified, default to lines, words, and bytes
    let default_mode = !args.bytes && !args.lines && !args.words && !args.chars;
    
    // Process file or stdin based on arguments
    match &args.file {
        Some(file) => {
            // ...
            let output = process_file(&args, file, default_mode);
            println!("{}", output);
            // ...
        },
        None => {
            // Then in your main function or wherever you call process_stdin, use:
            let output = process_stdin(&args, default_mode, StdinReader);
            println!("{}", output);
        },
    }
}