use std::fs;
use std::str::FromStr;

use crate::Result;
use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::{chunk_type::ChunkType, chunk::Chunk, png::Png};

pub fn encode(args: EncodeArgs) -> Result<()> {
    let path = args.path;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let message_chunk = Chunk::new(chunk_type, args.message.as_bytes().to_vec());
    let mut png = Png::from_file(path.clone())?;
    png.append_chunk(message_chunk);

    fs::write(path, png.as_bytes())?;
    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<()> {
    let path = args.path;
    let chunk_type = args.chunk_type.as_str();
    let png = Png::from_file(path)?;
    
    let message = png.chunk_by_type(chunk_type)
        .ok_or("Cannot find message with that chunk type")?;

    let message_string = message.data_as_string()?;
    println!("{}", message_string);
    
    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    let path = args.path;
    let chunk_type = args.chunk_type.as_str();
    let mut png = Png::from_file(path.clone())?;
    png.remove_first_chunk(chunk_type)?;

    fs::write(path, png.as_bytes())?;
    Ok(())
}

pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let path = args.path;
    let png = Png::from_file(path)?;
    println!("{png}");

    Ok(())
}