use std::str::FromStr;

use hex::{FromHex, FromHexError};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Checksum([u8; 20]);

impl Checksum {
    pub fn new(hash: [u8; 20]) -> Checksum {
        Checksum(hash)
    }
}

impl FromStr for Checksum {
    type Err = FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buffer = <[u8; 20]>::from_hex(s)?;

        Ok(Checksum(buffer))
    }
}
