# JSON Parser in Rust

A JSON parser implementation built in Rust, following the JSON specification. This project was developed as part of the coding challenge from [Coding Challenges](https://codingchallenges.fyi/challenges/challenge-json-parser).

## Features

- Fully compliant with the JSON specification
- Parses JSON objects, arrays, strings, numbers, booleans, and null values
- Provides detailed error messages for invalid JSON
- Handles nested structures
- Unicode support in strings

## Project Structure

```
json_parser/
├── Cargo.toml
├── src/
│   ├── main.rs          # Entry point and integration tests
│   ├── json_value.rs    # JSON value representation
│   ├── lexer/
│   │   ├── mod.rs       # Lexer implementation
│   │   └── token.rs     # Token definitions
│   └── parser/
│       └── mod.rs       # Parser implementation
└── README.md
```

## Building and Running

### Requirements

- Rust and Cargo (latest stable version recommended)

### Building

```bash
cargo build --release
```

### Running

```bash
./target/release/json_parser path/to/your/json_file.json
```

The program will output:
- "Valid JSON" and exit with code 0 if the file contains valid JSON
- "Invalid JSON" with an error message and exit with code 1 if the file contains invalid JSON

### Running Tests

```bash
cargo test
```

## Implementation Details

1. **Lexer**: Converts the input JSON string into a sequence of tokens
2. **Parser**: Processes the tokens to build a structured representation of the JSON data
3. **Error Handling**: Provides specific error messages for different types of JSON syntax errors

## License

MIT