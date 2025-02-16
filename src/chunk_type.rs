#![allow(unused_variables)]

use std::convert::TryFrom;
use std::ops::Index;
use std::fmt;
use std::str::{self, from_utf8, FromStr};

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PngByte {
    Critical,
    Public,
    Reserved,
    SafeToCopy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    r#type: [u8; 4],
}

impl ChunkType {
    /// Returns the raw bytes contained in this chunk
    pub fn bytes(&self) -> [u8; 4] {
        self.r#type
    }

    /// Returns the property state of the first byte as described in the PNG spec
    pub fn is_critical(&self) -> bool {
        (self.r#type[0] >> 5) & 1 == 0
    }

    /// Returns the property state of the second byte as described in the PNG spec
    pub fn is_public(&self) -> bool {
        (self.r#type[1] >> 5) & 1 == 0
    }

    /// Returns the property state of the third byte as described in the PNG spec
    pub fn is_reserved_bit_valid(&self) -> bool {
        (self.r#type[2] >> 5) & 1 == 0
    }

    /// Returns the property state of the fourth byte as described in the PNG spec
    pub fn is_safe_to_copy(&self) -> bool {
        (self.r#type[3] >> 5) & 1 == 1
    }

    /// Returns the property state of byte defined in enum as described in the PNG spec
    pub fn is_byte_valid(&self, byte: PngByte) -> bool {
        if byte != PngByte::SafeToCopy {
            return (self[byte] >> 5) & 1 == 0
        }
        return (self[byte] >> 5) & 1 == 1
    }

    /// Valid bytes are represented by the characters A-Z or a-z
    pub fn is_valid_byte(byte: u8) -> bool {
        (byte >= 65 && byte <= 90) || (byte >= 97 && byte <= 122)
    }

    /// Returns true if the reserved byte is valid and all four bytes are represented by the characters A-Z or a-z.
    /// Note that this chunk type should always be valid as it is validated during construction.
    pub fn is_valid(&self) -> bool {
        for byte in self.r#type.iter() {
            if !Self::is_valid_byte(*byte) {
                return false;
            }
        }
        self.is_reserved_bit_valid()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        for byte in bytes.iter() {
            if !Self::is_valid_byte(*byte) {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Bytes must be between ASCII A-Z, a-z. Or decimal 65-90, 97-122"
                )));
            }
        }
        Ok(ChunkType { r#type: bytes })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.r#type))
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 4 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Chunk type must be 4 bytes long"
            )));
        }
        
        let bytes: [u8; 4] = s.as_bytes().try_into().map_err(|_| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Failed to convert string to bytes"
            ))
        })?;

        ChunkType::try_from(bytes)
    }
}

impl Index<PngByte> for ChunkType {
    type Output = u8;

    fn index(&self, pngbyte: PngByte) -> &Self::Output {
        match pngbyte {
            PngByte::Critical => &self.r#type[0],
            PngByte::Public => &self.r#type[1],
            PngByte::Reserved => &self.r#type[2],
            PngByte::SafeToCopy => &self.r#type[3],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}