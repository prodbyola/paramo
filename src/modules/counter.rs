use std::{fs::read, io::Result as ioResult, borrow::BorrowMut};
use core::cmp::Ordering;
use std::fmt::Debug;

use super::sorter::sort;

#[derive(Clone, Debug)]
pub struct FrequencyNode {
    pub datum: u8,
    pub count: u32,
}

impl FrequencyNode {
    fn increment(&mut self){
        self.count += 1
    }
}

impl PartialOrd for FrequencyNode {
    fn lt(&self, other: &Self) -> bool {
        self.count < other.count
    }

    fn le(&self, other: &Self) -> bool {
        self.count <= other.count
    }

    fn gt(&self, other: &Self) -> bool {
        self.count > other.count
    }

    fn ge(&self, other: &Self) -> bool {
       self.count >= other.count
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.datum.partial_cmp(&other.datum) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.count.partial_cmp(&other.count)
    }
}

impl PartialEq for FrequencyNode {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

pub struct Frequencies {
    pub nodes: Vec<FrequencyNode>
}

impl Frequencies {
    fn new() -> Frequencies {
        Frequencies { nodes: Vec::new() }
    }

    /// Retrieves a node's index. 
    /// 
    /// This checks of the a node exists and if it does, 
    /// returns the node's index. Returns `None` if it doesn't exist.
    fn get_node_index(&self, b: &u8) -> Option<usize>{
        let mut index = None;
        let nodes = &self.nodes;

        for (i, n) in nodes.iter().enumerate() {
            if n.datum == *b {
                index = Some(i);
                break;
            }
        }

        index
    }

    /// Increments a node's count 
    fn increment_node(&mut self, index: usize) {
        let mut n = self.nodes.get(index).unwrap().to_owned();
        n.increment();

        self.nodes[index] = n;
    }

    /// Creates a new node
    fn insert_node(&mut self, b: u8){
        let n = FrequencyNode { datum: b, count: 1 };

        // let nx = self.get_insertion_index(&n);
        self.nodes.push(n);
    }

    fn sort(&mut self){
        let nodes = self.nodes.borrow_mut();
        sort::<FrequencyNode>(nodes);
    }
}


pub fn data_frequencies(filename: &str) -> ioResult<Frequencies> {
    let input_file = read(filename)?;
    let mut freq = Frequencies::new();

    for b in input_file {
        match freq.get_node_index(&b) {
            Some(i) => freq.increment_node(i),
            None => freq.insert_node(b)
        }
    }

    freq.sort();

    Ok(freq)
}
