use std::fs::read;
use std::io::Result as ioResult;
use crate::huffman::huffman_encoder;

mod huffman;

fn main() -> ioResult<()> {
    let input = read("sample.txt")?;
    huffman_encoder(input)?;

    Ok(())
}
