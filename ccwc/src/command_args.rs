use crate::ArgAction;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Count bytes
    #[arg(short = 'c', long = "bytes", action = ArgAction::SetTrue)]
    pub bytes: bool,

    /// Count lines
    #[arg(short = 'l', long = "lines", action = ArgAction::SetTrue)]
    pub lines: bool,

    /// Count words
    #[arg(short = 'w', long = "words", action = ArgAction::SetTrue)]
    pub words: bool,

    /// Count characters
    #[arg(short = 'm', long = "chars", action = ArgAction::SetTrue)]
    pub chars: bool,

    /// File to process, if omitted reads from stdin
    pub file: Option<String>,
}

