use std::fs;
use std::io::{self, Read};
use std::path::Path;
use clap::{Parser, ArgAction};

/// A simple implementation of the wc command
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Count bytes
    #[arg(short = 'c', long = "bytes", action = ArgAction::SetTrue)]
    bytes: bool,

    /// Count lines
    #[arg(short = 'l', long = "lines", action = ArgAction::SetTrue)]
    lines: bool,

    /// Count words
    #[arg(short = 'w', long = "words", action = ArgAction::SetTrue)]
    words: bool,

    /// Count characters
    #[arg(short = 'm', long = "chars", action = ArgAction::SetTrue)]
    chars: bool,

    /// File to process, if omitted reads from stdin
    file: Option<String>,
}

fn main() {
    let args = Args::parse();
    
    // If no counting options are specified, default to lines, words, and bytes
    let default_mode = !args.bytes && !args.lines && !args.words && !args.chars;
    
    // Process file or stdin based on arguments
    match &args.file {
        Some(file) => process_file(&args, file, default_mode),
        None => process_stdin(&args, default_mode),
    }
}

fn process_file(args: &Args, filename: &str, default_mode: bool) {
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
            println!("{}", output);
        },
        Err(_) => {
            eprintln!("Could not read file: {}", filename);
        }
    }
}

fn process_stdin(args: &Args, default_mode: bool) {
    let stdin = io::stdin();
    let mut bytes = Vec::new();
    
    // Read all stdin as bytes
    {
        let mut handle = stdin.lock();
        handle.read_to_end(&mut bytes).unwrap();
    }
    
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
    
    println!("{}", output);
}
