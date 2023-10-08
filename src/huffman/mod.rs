use std::fs::{OpenOptions, File, read};
use std::io::{Result as ioResult, Write, Read, Seek, SeekFrom};

use crate::huffman::utils::{str_generator, Header};

use self::frequency::frequency_counter;
use self::encoder::HuffmanEncoder;
use self::utils::{BitString, AppOptions, rm_file, ParamoIOFiles};

mod frequency;
mod encoder;
pub mod utils;

/// Implements huffman encoder
fn huffman_encoder(opts: AppOptions) -> ioResult<()>{
    let io_files = ParamoIOFiles::parse(&opts)?;

    let input_data = read(&io_files.input)?;
    let freq = frequency_counter(&input_data).unwrap();
    
    let mut encoder = HuffmanEncoder::new(&freq.data);
    let data = encoder.encode(&input_data)?;

    let header_bytes = serde_json::to_vec(&Header { 
        frequencies: freq.data,
        extension: io_files.input_ext
    })?;

    // We generate a 4-byte container to store our header length
    // This will be used to retrieve header content when decoding
    let hlen_bits = format!("{:b}", header_bytes.len());
    let mut hlen_container = str_generator('0', 32 - hlen_bits.len());
    hlen_container.push_str(&hlen_bits);
    let hlen = hlen_container.to_vec()?;

    let output_file = io_files.output;
    rm_file(&output_file)?;
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(output_file)?;

    file.write_all(&hlen)?;
    file.write_all(&header_bytes)?;
    file.write_all(&data)?;

    Ok(())
}

/// Implements huffman decoder
fn huffman_decoder(opts: AppOptions) -> ioResult<()> {
    let io_files = ParamoIOFiles::parse(&opts)?;
    let mut file = File::open(&io_files.input)?;
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
    let mut output = io_files.output;
    let mut ext = io_files.output_ext.unwrap_or_default();
    if let Some(hext) = header.extension {
        ext = hext;
    }
    
    output.set_extension(ext);
    rm_file(&output)?;
    
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output)?;

    file.write_all(&decoded_data)?;

    Ok(())
}

pub fn run_huffman(opts: AppOptions) -> ioResult<()> {
    if opts.decode {
        huffman_decoder(opts)?;
    } else {
        huffman_encoder(opts)?;
    }

    Ok(())
}