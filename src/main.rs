use std::fs::read;
use std::io::Result as ioResult;
use crate::huffman::huffman_decoder;
use clap::{Parser, command};
use huffman::huffman_encoder;

mod huffman;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Options {
    #[arg(short, long, default_value_t = false)]
    decode: bool
}

fn main() -> ioResult<()> {
    let opt = Options::parse();

    if opt.decode {
        huffman_decoder()?;
    } else {
        let input = read("sample2.txt")?;
        huffman_encoder(input)?;
    }

    Ok(())
}
