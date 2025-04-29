use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;
use clap::{Parser, ArgAction};

/// A Rust implementation of the Unix cut command
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Field list, comma or whitespace separated (e.g., "1,2" or "1 2")
    #[arg(short = 'f', required = true)]
    fields: String,

    /// Delimiter character (default is tab)
    #[arg(short = 'd', default_value = "\t")]
    delimiter: String,

    /// Input file (default is stdin if no file or "-" is provided)
    #[arg(action = ArgAction::Append)]
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    // Parse fields
    let fields: Vec<usize> = parse_fields(&args.fields);
    
    // Get delimiter (first character of the delimiter string)
    let delimiter = args.delimiter.chars().next().unwrap_or('\t');

    // Handle input: if no files provided or "-" specified, use stdin
    if args.files.is_empty() {
        process_input(&mut io::stdin().lock(), &fields, delimiter);
    } else {
        for file_path in args.files {
            if file_path == "-" {
                process_input(&mut io::stdin().lock(), &fields, delimiter);
            } else {
                let file = match File::open(&file_path) {
                    Ok(file) => file,
                    Err(err) => {
                        eprintln!("cut: {}: {}", file_path, err);
                        process::exit(1);
                    }
                };
                let reader = BufReader::new(file);
                process_input(reader, &fields, delimiter);
            }
        }
    }
}

fn parse_fields(fields_str: &str) -> Vec<usize> {
    let mut result = Vec::new();
    
    // Split by comma or whitespace
    for field in fields_str.split(|c: char| c == ',' || c.is_whitespace()) {
        if !field.is_empty() {
            match field.parse::<usize>() {
                Ok(num) if num > 0 => result.push(num - 1), // Convert to 0-indexed
                _ => {
                    eprintln!("cut: invalid field value: {}", field);
                    process::exit(1);
                }
            }
        }
    }
    
    if result.is_empty() {
        eprintln!("cut: fields must be non-empty");
        process::exit(1);
    }
    
    result
}

fn process_input<R: BufRead>(reader: R, fields: &[usize], delimiter: char) {
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let parts: Vec<&str> = line.split(delimiter).collect();
                
                // Extract and join the requested fields
                let mut first = true;
                for &field_idx in fields {
                    if !first {
                        print!("{}", delimiter);
                    }
                    first = false;
                    
                    if field_idx < parts.len() {
                        print!("{}", parts[field_idx]);
                    }
                }
                println!();
            },
            Err(err) => {
                eprintln!("cut: {}", err);
                process::exit(1);
            }
        }
    }
}
