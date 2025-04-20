use clap::{Parser, Subcommand};
use bitvec::prelude::*;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::PathBuf;
use std::error::Error;
use std::cmp::Ordering;

#[derive(Parser)]
#[command(author, version, about = "Huffman encoder/decoder tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compress the input file to output file
    Encode {
        /// Input file path
        input: PathBuf,
        /// Output file path
        output: PathBuf,
    },
    /// Decompress the input file to output file
    Decode {
        /// Input file path
        input: PathBuf,
        /// Output file path
        output: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Encode { input, output } => compress_file(&input, &output)?,
        Commands::Decode { input, output } => decompress_file(&input, &output)?,
    }
    Ok(())
}

#[derive(Eq)]
struct Node {
    freq: usize,
    symbol: Option<u8>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_huffman_tree(freq_map: &HashMap<u8, usize>) -> Option<Box<Node>> {
    let mut heap: BinaryHeap<Node> = freq_map.iter()
        .map(|(&symbol, &freq)| Node { freq, symbol: Some(symbol), left: None, right: None })
        .collect();
    if heap.is_empty() {
        return None;
    }
    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        heap.push(Node {
            freq: left.freq + right.freq,
            symbol: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        });
    }
    Some(Box::new(heap.pop().unwrap()))
}

fn generate_codes(node: &Node, prefix: &mut BitVec<u8, Msb0>, codes: &mut HashMap<u8, BitVec<u8, Msb0>>) {
    if let Some(symbol) = node.symbol {
        codes.insert(symbol, prefix.clone());
    } else {
        if let Some(ref left) = node.left {
            prefix.push(false);
            generate_codes(left, prefix, codes);
            prefix.pop();
        }
        if let Some(ref right) = node.right {
            prefix.push(true);
            generate_codes(right, prefix, codes);
            prefix.pop();
        }
    }
}

fn compress_file(input: &PathBuf, output: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(input)?);
    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;
    let mut freq_map = HashMap::new();
    for &byte in &data {
        *freq_map.entry(byte).or_insert(0) += 1;
    }
    let tree = build_huffman_tree(&freq_map).ok_or("Empty input")?;
    let mut codes = HashMap::new();
    let mut prefix = BitVec::<u8, Msb0>::new();
    generate_codes(&tree, &mut prefix, &mut codes);
    let mut bits = BitVec::<u8, Msb0>::new();
    for &byte in &data {
        bits.extend(codes.get(&byte).unwrap());
    }
    let bit_length = bits.len() as u64;
    let mut writer = BufWriter::new(File::create(output)?);
    writer.write_all(b"HUFF")?;
    let symbol_count = freq_map.len() as u16;
    writer.write_all(&symbol_count.to_be_bytes())?;
    let mut symbols: Vec<_> = freq_map.iter().collect();
    symbols.sort_by_key(|(&symbol, _)| symbol);
    for (&symbol, &count) in symbols {
        writer.write_all(&[symbol])?;
        writer.write_all(&(count as u64).to_be_bytes())?;
    }
    writer.write_all(&bit_length.to_be_bytes())?;
    writer.write_all(bits.as_raw_slice())?;
    writer.flush()?;
    Ok(())
}

fn decompress_file(input: &PathBuf, output: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(input)?);
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic)?;
    if &magic != b"HUFF" {
        return Err("Invalid format".into());
    }
    let mut buf2 = [0u8; 2];
    reader.read_exact(&mut buf2)?;
    let symbol_count = u16::from_be_bytes(buf2) as usize;
    let mut freq_map = HashMap::new();
    for _ in 0..symbol_count {
        let mut sym = [0u8; 1];
        reader.read_exact(&mut sym)?;
        let mut cnt = [0u8; 8];
        reader.read_exact(&mut cnt)?;
        freq_map.insert(sym[0], u64::from_be_bytes(cnt) as usize);
    }
    let mut bit_len_buf = [0u8; 8];
    reader.read_exact(&mut bit_len_buf)?;
    let bit_length = u64::from_be_bytes(bit_len_buf) as usize;
    let mut encoded = Vec::new();
    reader.read_to_end(&mut encoded)?;
    let bits = BitVec::<u8, Msb0>::from_vec(encoded);
    let tree = build_huffman_tree(&freq_map).ok_or("Invalid header")?;
    let mut output_data = Vec::new();
    let mut node = &*tree;
    for bit in bits.iter().take(bit_length) {
        node = if !*bit { node.left.as_ref().unwrap() } else { node.right.as_ref().unwrap() };
        if let Some(symbol) = node.symbol {
            output_data.push(symbol);
            node = &*tree;
        }
    }
    let mut writer = BufWriter::new(File::create(output)?);
    writer.write_all(&output_data)?;
    writer.flush()?;
    Ok(())
}