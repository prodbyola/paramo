use std::fs::read;
use std::io::Result as ioResult;
use crate::huffman::run_huffman;

mod huffman;

fn main() -> ioResult<()> {
    let input = read("sample.txt")?;
    run_huffman(input)?;

    Ok(())
}
