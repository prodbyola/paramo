use std::fs::{OpenOptions, File, read};
use std::io::{Result as ioResult, Write, Read, Seek, SeekFrom};

use crate::huffman::utils::{str_generator, Header};

use self::frequency::frequency_counter;
use self::encoder::HuffmanEncoder;
use self::utils::{BitString, AppOptions, rm_file};

mod frequency;
mod encoder;
pub mod utils;

/// Implements huffman encoder
fn huffman_encoder(input: &str, output: &str) -> ioResult<()>{
    let input_data = read(input)?;
    let freq = frequency_counter(&input_data).unwrap();
    
    let mut encoder = HuffmanEncoder::new(&freq.data);
    let data = encoder.encode(&input_data)?;

    let header_bytes = serde_json::to_vec(&Header { frequencies: freq.data })?;

    // We generate a 4-byte container to store our header length
    // This will be used to retrieve header content when decoding
    let hlen_bits = format!("{:b}", header_bytes.len());
    let mut hlen_container = str_generator('0', 32 - hlen_bits.len());
    hlen_container.push_str(&hlen_bits);
    let hlen = hlen_container.to_vec()?;

    rm_file(output)?;
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(output)?;

    file.write_all(&hlen)?;
    file.write_all(&header_bytes)?;
    file.write_all(&data)?;

    Ok(())
}

/// Implements huffman decoder
fn huffman_decoder(input: &str, output: &str) -> ioResult<()> {
    let mut file = File::open(input)?;
    let mut hlen_bytes = [0u8; 4];

    // Get file header's length
    file.read_exact(&mut hlen_bytes)?;
    let hlen = u32::from_be_bytes(hlen_bytes);
    let hlen: usize = u32::try_into(hlen).unwrap();

    // read the header
    let mut header_bytes = vec![0u8; hlen];
    file.seek(SeekFrom::Start(4))?;
    file.read_exact(&mut header_bytes)?;
    
    let header: Header = serde_json::from_slice(&header_bytes)?;
    let encoder = HuffmanEncoder::new(&header.frequencies);

    // read the data
    let hlen = u64::try_from(hlen).unwrap();
    file.seek(SeekFrom::Start(4+hlen))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let decoded_data = encoder.decode(data)?;
    rm_file(output)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output)?;

    file.write_all(&decoded_data)?;

    Ok(())
}

pub fn run_huffman(opts: AppOptions) -> ioResult<()> {
    if opts.decode {
        huffman_decoder(&opts.input, &opts.output)?;
    } else {
        huffman_encoder(&opts.input, &opts.output)?;
    }

    Ok(())
}