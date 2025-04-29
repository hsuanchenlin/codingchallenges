# Rust Implementation of `cut`

This is a Rust implementation of the Unix command line tool `cut`, which cuts out selected portions from each line in a file.

## Features

- Field extraction with the `-f` option
- Custom delimiter support with the `-d` option
- Support for reading from files or standard input
- Handling of multiple input files

## Installation

```bash
# Clone this repository
git clone <repository-url>

# Navigate to the cut directory
cd cut

# Build with Cargo
cargo build --release
```

The compiled binary will be available at `target/release/cut`.

## Usage

```bash
# Extract the second field from a tab-separated file
./cut -f2 sample.tsv

# Extract the first field from a comma-separated file
./cut -f1 -d, fourchords.csv

# Extract multiple fields
./cut -f1,2 sample.tsv
./cut -f"1 2" -d, fourchords.csv

# Read from standard input
cat fourchords.csv | ./cut -d, -f"1 2"
tail -n5 fourchords.csv | ./cut -d, -f"1 2"

# Read from standard input explicitly
tail -n5 fourchords.csv | ./cut -d, -f"1 2" -
```

## Options

- `-f FIELDS`: Comma or whitespace separated list of fields to select (required)
- `-d DELIMITER`: Character that separates fields (default is tab)
- `FILES`: Input files (reads from stdin if no files are provided or `-` is specified)

## Examples

```bash
# Extract the second field from sample.tsv
./cut -f2 sample.tsv

# Extract the first and second fields from fourchords.csv
./cut -d, -f"1 2" fourchords.csv

# Count unique artists in the fourchords.csv file
./cut -f2 -d, fourchords.csv | uniq | wc -l
```
