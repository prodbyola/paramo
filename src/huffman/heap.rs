use std::{collections::{HashMap, BinaryHeap}, rc::Rc};

use super::queue::{Frequencies, Frequency};

/// Huffman internal (non-leaf) node implementation 
#[derive(Eq)]
pub struct HuffmanNode {
    weight: usize,
    value: Option<u8>,
    left: Rc<Option<HuffmanNode>>,
    right: Rc<Option<HuffmanNode>>,
}

impl HuffmanNode {
    fn init(freq: Frequency) -> HuffmanNode {
        HuffmanNode { weight: freq.1, value: Some(freq.0), left: Rc::new(None), right: Rc::new(None) }
    }

    fn weight(&self) -> usize {
        self.weight
    }

    fn left(&self) -> &Option<HuffmanNode> {
        &*self.left
    }

    fn right(&self) -> &Option<HuffmanNode> {
        &*self.right
    }

    fn is_leaf(&self) -> bool {
        let l = &*self.left;
        let r = &*self.right;

       l.is_none() && r.is_none()
    }

    
}

impl PartialEq for HuffmanNode {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, other: &Self) -> bool {
        self.weight() == other.weight()
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.weight().partial_cmp(&self.weight())
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight().cmp(&self.weight())
    }
}

/// Huffman tree implementation 
pub struct HuffmanTree {
    pub root: HuffmanNode,
}

impl HuffmanTree {
    pub fn new(frequencies: Frequencies) -> HuffmanTree {
        let mut pq = BinaryHeap::with_capacity(frequencies.len());
        for freq in frequencies {
            pq.push(HuffmanNode::init(freq));
        }

        while pq.len() > 1 {
            let left = pq.pop().unwrap();
            let right = pq.pop().unwrap();

            let merged = HuffmanNode {
                value: None,
                weight: left.weight + right.weight,
                left: Rc::new(Some(left)),
                right: Rc::new(Some(right))
            };

            pq.push(merged);
        }

        let root = pq.pop().unwrap();

        HuffmanTree { root }
    }
}

pub fn assign_codes(root: Option<&HuffmanNode>, current_code: &mut String, codes: &mut HashMap<u8, String>) {
    if let Some(t) = root {
        if t.is_leaf() {
            if let Some(v) = t.value {
                codes.insert(v, current_code.to_owned());
                return;
            }
        }

        if let Some(_) = t.left() {
            current_code.push('0');
            assign_codes(t.left().as_ref(), current_code, codes);
            current_code.pop();
        }

        if let Some(_) = t.right() {
            current_code.push('1');
            assign_codes(t.right().as_ref(), current_code, codes);
            current_code.pop();
        }
    }
}
