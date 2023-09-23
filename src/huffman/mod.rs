use std::fs::File;
use std::io::{Result as ioResult, Write};

use self::queue::frequency_counter;
use self::heap::HuffmanEncoder;

mod queue;
mod heap;

pub fn huffman_encoder(input_data: Vec<u8>) -> ioResult<()>{
    let freq = frequency_counter(&input_data).unwrap();

    let mut encoder = HuffmanEncoder::new(freq.data, &input_data);
    let encoded = encoder.encode()?;
    let output = "output";
    let mut file = File::create(output)?;
    file.write_all(&encoded)?;

    Ok(())
}