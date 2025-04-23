use std::io::{Read, Result};

pub trait StdinOperations {
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>;
}

pub struct StdinReader;

impl StdinOperations for StdinReader {
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_end(buf)
    }
}