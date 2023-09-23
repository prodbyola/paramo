use std::{collections::{HashMap, BinaryHeap}, rc::Rc};
use std::io::Error;
use super::queue::{Frequencies, Frequency};

/// Huffman internal (non-leaf) node implementation 
#[derive(Eq)]
pub struct HuffmanNode<'a > {
    weight: usize,
    value: Option<&'a u8>,
    left: Rc<Option<HuffmanNode<'a>>>,
    right: Rc<Option<HuffmanNode<'a>>>,
}

impl HuffmanNode<'_> {
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

impl PartialEq for HuffmanNode<'_> {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, other: &Self) -> bool {
        self.weight() == other.weight()
    }
}

impl PartialOrd for HuffmanNode<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.weight().partial_cmp(&self.weight())
    }
}

impl Ord for HuffmanNode<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight().cmp(&self.weight())
    }
}

/// Huffman tree implementation 
pub struct HuffmanEncoder<'a> {
    root: HuffmanNode<'a>,
    codes: HashMap<&'a u8, String>,
    input: &'a Vec<u8>,
}

impl<'a> HuffmanEncoder<'a> {
    pub fn new(frequencies: Frequencies<'a>, input: &'a Vec<u8>) -> HuffmanEncoder<'a> {
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

        HuffmanEncoder { root, input, codes: HashMap::new() }
    }

    fn assign_codes(&'a mut self) -> &HuffmanEncoder<'a> {
        assign_codes(Some(&self.root), &mut String::new(), &mut self.codes);
        self
    }

    pub fn encode(&'a mut self) -> Result<Vec<u8>, Error> {
        let encoder = self.assign_codes();

        // Let's encode our data as binary using the generated Huffman code.
        let mut current_byte = 0u8; // 8-bits zeros
        let mut remaining_bytes: u8 = 8;
        let mut encoded_data = Vec::new();
        // let mut err = None;

        for c in encoder.input {
            if let Some(code) = encoder.codes.get(c) {
                for bit in code.chars() {
                    let bit_value: u8 = if bit == '1' { 1 } else { 0 };
                    current_byte = (current_byte << 1) | bit_value;

                    remaining_bytes -= 1;

                    // If current_byte is full, push it to encoded data and reinitialize our monitors
                    if remaining_bytes == 0 {
                        encoded_data.push(current_byte);
                        current_byte = 0;
                        remaining_bytes = 8;
                    }
                }
            } else {
                let msg = format!("Error finding the char {} in encoded map.", *c as char);
                let err = Error::new(std::io::ErrorKind::InvalidData, msg);
                return Err(err);
            }
        }

        Ok(encoded_data)
    }
}

fn assign_codes<'a>(root: Option<&'a HuffmanNode>, current_code: &mut String, codes: &mut HashMap<&'a u8, String>) {
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
