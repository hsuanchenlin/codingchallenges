# Huffman Compression Tool

A Rust implementation of the Huffman coding algorithm for lossless data compression.

## Overview

This tool implements the Huffman coding algorithm to provide lossless data compression and decompression. Huffman coding works by assigning variable-length codes to characters based on their frequency in the input - more frequent characters get shorter codes.

## Features

- Compress text files using Huffman coding
- Decompress previously compressed files
- Display character frequency tables
- Output compression statistics

## Requirements

- Rust 1.56.0 or later
- Cargo package manager

## Dependencies

- `clap`: Command-line argument parsing
- `bitvec`: Efficient bit manipulation

## Installation

Clone the repository and build the project:

```bash
cargo build --release
```

The compiled binary will be located at `target/release/huffman`.

## Usage

### Compression (Encoding)

```bash
huffman encode -i <input_file> -o <output_file> [-f]
```

- `-i, --input`: Input file to compress
- `-o, --output`: Output file for compressed data
- `-f, --frequency-table`: (Optional) Display the character frequency table

### Decompression (Decoding)

```bash
huffman decode -i <input_file> -o <output_file>
```

- `-i, --input`: Compressed input file
- `-o, --output`: Output file for decompressed data

## How It Works

The Huffman compression algorithm follows these steps:

1. **Frequency Analysis**: Count the frequency of each character in the input file
2. **Tree Construction**: Build a binary tree where characters are leaf nodes and their frequency determines their position
3. **Code Generation**: Traverse the tree to generate prefix codes for each character (left=0, right=1)
4. **Encoding**: Replace each character with its code to generate compressed data
5. **Header Writing**: Store the tree structure in the file header to enable later decompression
6. **Decoding**: Use the stored tree to map bit sequences back to original characters

## File Format

The compressed file format consists of:
- 4 bytes: Size of the serialized Huffman tree (big-endian u32)
- N bytes: Serialized Huffman tree
- 1 byte: Number of valid bits in the last byte (0 if all bits are used)
- Remaining bytes: Compressed data

## Performance

The compression ratio varies based on the input data's characteristics. Text files with repetitive content typically achieve higher compression ratios.
