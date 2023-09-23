use std::io::Result as ioResult;
use crate::huffman::heap::assign_codes;

use self::queue::frequency_counter;
use self::heap::HuffmanTree;

mod queue;
mod heap;

pub fn run_huffman(input: Vec<u8>) -> ioResult<()>{
    let freq = frequency_counter(input).unwrap();
    for cf in &freq.data {
        println!("{}: {}", cf.0 as char, cf.1);
    }
    
    HuffmanTree::new(freq.data);
    // println!("root is leaf {}", tree.root.is_leaf());

    // let mut codes = HashMap::new();
    // let root = Some(&heap.tree.root);

    // assign_codes(root, &mut String::new(), &mut codes);
    // println!("{:?}", codes);
    Ok(())
}