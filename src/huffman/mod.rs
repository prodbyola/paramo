use std::collections::HashMap;
use std::fs::{OpenOptions, File, remove_file, self};
use std::io::{Result as ioResult, Write, Read, Seek, SeekFrom};

use crate::huffman::utils::{str_generator, Header};

use self::queue::frequency_counter;
use self::encoder::HuffmanEncoder;
use self::utils::BitString;

mod queue;
mod encoder;
mod utils;



pub fn huffman_encoder(input_data: Vec<u8>) -> ioResult<()>{
    println!("input len {}", input_data.len());

    let freq = frequency_counter(&input_data).unwrap();
    let mut codes = HashMap::new();

    let mut encoder = HuffmanEncoder::new(freq.data, &input_data);
    
    encoder.assign_codes(&mut codes);
    let data = encoder.encode(&codes)?;
    println!("encoded len {}", data.len());

    let header_bytes = serde_json::to_vec(&Header { codes })?;

    // We generate a 4-byte container to store our header length
    // This will be used to retrieve header content when decoding
    let hlen_bits = format!("{:b}", header_bytes.len());
    let mut hlen_container = str_generator('0', 32 - hlen_bits.len());
    hlen_container.push_str(&hlen_bits);
    let hlen = hlen_container.to_vec()?;

    let output = "output";
    if let Ok(_) = fs::metadata(output) {
        remove_file(output)?;
    }

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

pub fn huffman_decoder() -> ioResult<()> {
    let mut file = File::open("output")?;
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

    println!("{:?}", header.codes.len());

    // read the data
    let hlen = u64::try_from(hlen).unwrap();
    file.seek(SeekFrom::Start(4+hlen))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    println!("data len {}", data.len());

    Ok(())
}