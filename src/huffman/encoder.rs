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

impl<'a> HuffmanNode<'a> {
    fn init(freq: &'a Frequency) -> HuffmanNode<'a> {
        HuffmanNode { weight: freq.1, value: Some(&freq.0), left: Rc::new(None), right: Rc::new(None) }
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
    codes: HashMap<u8, String>,
}

impl<'a> HuffmanEncoder<'a> {
    pub fn new(frequencies: &'a Frequencies) -> HuffmanEncoder {
        // The priority queue for building our initial partial tree
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

        let root: HuffmanNode<'a> = pq.pop().unwrap();
        let mut codes: HashMap<u8, String> = HashMap::new();

        assign_codes(
            Some(&root), 
            &mut String::new(), 
            &mut codes
        );

        HuffmanEncoder { 
            root, 
            codes
        }
    }

    pub fn encode(&'a mut self, input_data: &Vec<u8>) -> Result<Vec<u8>, Error> {
        // let encoder = self.assign_codes();

        // Let's encode our data as binary using the generated Huffman code.
        let mut current_byte = 0u8; // 8-bits zeros
        let mut remaining_bytes: u8 = 8;
        let mut encoded_data = Vec::new();

        for c in input_data {
            if let Some(code) = self.codes.get(c) {
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

    pub fn decode(&self, encoded_data: Vec<u8>) -> Result<Vec<u8>, Error> {
        decode(&self.root, encoded_data)
    }
}

fn assign_codes<'a>(
    root: Option<&'a HuffmanNode>, 
    current_code: &mut String, 
    codes: &mut HashMap<u8, String>
) {
    if let Some(t) = root {
        if t.is_leaf() {
            if let Some(v) = t.value {
                codes.insert(*v, current_code.to_owned());
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

fn decode<'a>(root: &'a HuffmanNode<'a>, encoded_data: Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut decoded_data = Vec::new();
    let mut current_byte = 0u8;
    let mut remaining_bits = 0;

    let mut current_node = root;

    for byte in encoded_data {
        for bit_position in (0..8).rev() {
            let bit = (byte >> bit_position) & 1;
            if bit == 0 {
                if let Some(n) = current_node.left().as_ref() {
                    current_node = n;
                }
            } else {
                if let Some(n) = current_node.right().as_ref() {
                    current_node = n;
                }
            }

            if current_node.is_leaf() {
                let value = current_node.value.unwrap();
                println!("leaf val {}", *value as char);
                current_byte = (current_byte << 1) | value;
                remaining_bits += 1;

                if remaining_bits == 8 {
                    // println!("{}", current_byte);
                    decoded_data.push(current_byte);
                    remaining_bits = 0;
                    current_byte = 0;
                }
            }
        }
    }

    // Check for any remaining bits in the last byte and handle padding
    // if remaining_bits > 0 {
    //     // Determine the number of padding bits
    //     let padding_bits = 8 - remaining_bits;

    //     // Ensure that the padding bits are all zeros
    //     if (current_byte >> padding_bits) != 0 {
    //         return Err(Error::new(
    //             std::io::ErrorKind::InvalidData,
    //             "Invalid encoded data: padding bits are not zero.",
    //         ));
    //     }

    //     // Remove padding bits from the last byte
    //     current_byte &= (1 << padding_bits) - 1;

    //     // Push the last byte without padding to the decoded data
    //     decoded_data.push(current_byte);
    // }

    Ok(decoded_data)
}
