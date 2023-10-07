use std::{io::{Error, Result as ioResult}, fs::{remove_file, self}};

use clap::{Parser, command};
use serde::{Deserialize, Serialize};

use super::frequency::Frequencies;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct AppOptions {
    #[arg(short, long)]
    pub input: String,

    #[arg(short, long)]
    pub output: String,
    
    #[arg(short, long, default_value_t = false)]
    pub decode: bool,
}

pub trait BitString {
    fn len(&self) -> usize;

    /// Converts the input string to to `Vec`.
    fn to_vec(&self) -> Result<Vec<u8>, Error>;

    /// Checks if the string can converted to `Vec`. 
    fn is_complete(&self) -> bool {
        self.len() % 8 == 0
    }

    /// Length of bytes to be derived from the string. 
    fn byte_len(&self) -> usize {
        self.len()/8
    }
}

impl BitString for String {
    fn len(&self) -> usize {
        self.len()
    }

    fn to_vec(&self) -> Result<Vec<u8>, Error> {
        if !self.is_complete() {
            let err = Error::new(std::io::ErrorKind::InvalidInput, "String is not a complete bit string.");
            return Err(err);
        }

        let mut current_byte = 0u8; // 8-bits zeros
        let mut remaining_bytes: u8 = 8;
        let mut output = Vec::with_capacity(self.byte_len());

        for bit in self.chars() {
            if bit != '1' && bit != '0' {
                let err = Error::new(std::io::ErrorKind::InvalidData, "Not a valid bit string");
                return Err(err);
            }

            let bit_value: u8 = if bit == '1' { 1 } else { 0 };
            current_byte = (current_byte << 1) | bit_value;

            remaining_bytes -= 1;

            // If current_byte is full, push it to `output` and reinitialize our monitors
            if remaining_bytes == 0 {
                output.push(current_byte);
                current_byte = 0;
                remaining_bytes = 8;
            }
        }

        Ok(output)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub frequencies: Frequencies
}

/// Generates a repeated character string
pub fn str_generator(c: char, len: usize) -> String {
    std::iter::repeat(c).take(len).collect()
}

pub fn rm_file(path: &str) -> ioResult<()> {
    if let Ok(_) = fs::metadata(path) {
        remove_file(path)?;
    }

    Ok(())
}