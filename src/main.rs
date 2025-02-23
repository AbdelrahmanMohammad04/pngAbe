use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use args::{PngAbeArgs, PngCommands};
use commands::{decode, encode, print_chunks, remove};

fn main() -> Result<()> {
    let cli = PngAbeArgs::parse();

    match cli.command {
        PngCommands::Encode(args) => encode(args),
        PngCommands::Decode(args) => decode(args),
        PngCommands::Remove(args) => remove(args),
        PngCommands::Print(args) => print_chunks(args),
    }
}