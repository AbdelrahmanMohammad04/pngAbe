use std::path::PathBuf;
use clap::{self, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct PngAbeArgs {
    #[command(subcommand)]
    pub command: PngCommands
}

#[derive(Subcommand)]
pub enum PngCommands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Parser, Debug)]
pub struct EncodeArgs {
    #[arg(short, long)]
    pub path: PathBuf,

    #[arg(short)]
    pub chunk_type: String,
    
    #[arg(short, long)]
    pub message: String,
}

#[derive(Parser, Debug)]
pub struct DecodeArgs {
    #[arg(short, long)]
    pub path: PathBuf,
    
    #[arg(short)]
    pub chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    #[arg(short, long)]
    pub path: PathBuf,
    
    #[arg(short)]
    pub chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
    #[arg(short, long)]
    pub path: PathBuf,
}