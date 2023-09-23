use std::collections::HashMap;
use std::io::Result as ioResult;
use crate::huffman::heap::assign_codes;

use self::queue::create_queue;
use self::heap::HHeap;

mod queue;
mod heap;

pub fn run_huffman(input: Vec<u8>) -> ioResult<()>{
    let mut queue = create_queue(input).unwrap();
    for cf in &queue.nodes {
        println!("{}: {}", cf.value().unwrap() as char, cf.weight());
    }
    let heap = HHeap::from_queue(&mut queue.nodes);

    let mut codes = HashMap::new();
    let root = Some(&heap.tree.root);

    assign_codes(root, &mut String::new(), &mut codes);
    // println!("{:?}", codes);
    Ok(())
}