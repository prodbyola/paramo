use std::collections::HashMap;
use std::io::Result as ioResult;
use crate::huffman::heap::assign_codes;

use self::queue::frequency_counter;
use self::heap::HuffmanTree;

mod queue;
mod heap;

pub fn run_huffman(input: Vec<u8>) -> ioResult<()>{
    let freq = frequency_counter(input).unwrap();

    let tree = HuffmanTree::new(freq.data);
    let mut codes = HashMap::new();
    let root = Some(&tree.root);

    assign_codes(root, &mut String::new(), &mut codes);
    
    for (k, v) in codes {
        println!("{}: {}", k as char, v);
    }
    Ok(())
}