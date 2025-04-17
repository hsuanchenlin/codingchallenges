# CCWC - A Simple Word, Character and Line Counter

CCWC (Claude's Count Word Characters) is a simple command-line utility inspired by the Unix `wc` command. It allows you to count bytes, characters, words, and lines in text files.

## Features

- Count bytes in a file (`-c` option)
- Count characters in a file (`-m` option)
- Count words in a file (`-w` option)
- Count lines in a file (`-l` option)
- Default behavior (no option provided) shows all counts
- Read from files or from standard input (pipe)

## Installation

Clone the repository and build the project:

```bash
git clone https://github.com/yourusername/ccwc.git
cd ccwc
make
```

## Usage

```
ccwc [OPTION]... [FILE]...
```

### Options

- `-c` : Print the byte count
- `-m` : Print the character count
- `-w` : Print the word count
- `-l` : Print the line count
- When no option is provided, ccwc displays all counts

### Examples

Count lines in a file:

```bash
./ccwc -l test.txt
```

Count words in a file:

```bash
./ccwc -w test.txt
```

Count characters in a file:

```bash
./ccwc -m test.txt
```

Count bytes in a file:

```bash
./ccwc -c test.txt
```

Display all counts for a file:

```bash
./ccwc test.txt
```

Use with pipes:

```bash
cat test.txt | ./ccwc -l
```

## Implementation Notes

CCWC closely follows the behavior of the original Unix `wc` command with a few key differences:

- Character count (`-m`) correctly handles UTF-8 multibyte characters
- Clear, readable C code with minimal dependencies
- Focused on core counting functionality without unnecessary complexity

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
