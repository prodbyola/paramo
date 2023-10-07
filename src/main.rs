use std::io::Result as ioResult;
use clap::Parser;

use huffman::utils::AppOptions;
use crate::huffman::run_huffman;

mod huffman;

fn main() -> ioResult<()> {
    let opts = AppOptions::parse();
    run_huffman(opts)
}
