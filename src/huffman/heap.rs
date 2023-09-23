use std::{collections::{HashMap, BinaryHeap}, rc::Rc};

// /// Huffman base trait 
// trait HuffmanNode {
//     fn is_leaf(&self) -> bool;
//     fn weight(&self) -> usize;
// }

// /// Huffman leaf node implementation 
// struct HuffmanLeafNode {
//     value: u8,
//     weight: usize
// }

// impl HuffmanNode for HuffmanLeafNode {
    
//     fn is_leaf(&self) -> bool {
//         true
//     }

//     fn weight(&self) -> usize {
//         self.weight
//     }
// }

/// Huffman internal (non-leaf) node implementation 
#[derive(Clone)]
pub struct HuffmanNode {
    weight: usize,
    value: Option<u8>,
    left: Rc<Option<HuffmanNode>>,
    right: Rc<Option<HuffmanNode>>,
}

impl HuffmanNode {
    pub fn init(value: u8) -> HuffmanNode {
        HuffmanNode { weight: 1, value: Some(value), left: Rc::new(None), right: Rc::new(None) }
    }

    pub fn weight(&self) -> usize {
        self.weight
    }

    pub fn value(&self) -> Option<u8> {
        self.value
    }

    pub fn left(&self) -> &Option<HuffmanNode> {
        &*self.left
    }

    pub fn right(&self) -> &Option<HuffmanNode> {
        &*self.right
    }

    pub fn is_leaf(&self) -> bool {
        let l = &*self.left;
        let r = &*self.right;

       l.is_none() && r.is_none()
    }

    pub fn add_weight(&mut self) {
        self.weight += 1;
    }
}

/// Huffman tree implementation 
#[derive(Clone)]
pub struct HuffmanTree {
    pub root: HuffmanNode 
}

impl HuffmanTree {
    pub fn add_weight(&mut self) {
        self.root.weight += 1;
    }

    pub fn weight(&self) -> usize {
        self.root.weight()
    }

    pub fn value(&self) -> Option<u8> {
        self.root.value
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
        self.weight().partial_cmp(&other.weight())
    }
}

pub struct HHeap<'a> {
    pub tree: &'a HuffmanTree,
    pub codes: HashMap<u8, String>,
}

impl<'a> HHeap<'a> {
    pub fn from_queue(ts: &'a mut Vec<HuffmanNode>) -> HHeap<'a> {
        while ts.len() > 1 {
            let tmp1 = ts.remove(1);
            let tmp2 = ts.remove(0);

            let tmp3 = HuffmanNode {
                weight: tmp1.weight() + tmp2.weight(),
                left: Rc::new(Some(tmp1)),
                right: Rc::new(Some(tmp2)),
                value: None,
            };

            ts.insert(0, tmp3);
        }

        HHeap { 
            tree: ts.get(0).unwrap(),
            codes: HashMap::new() 
        }
    }
}

// fn build_heap(){
//     let hp = BinaryHeap::new();
//     hp
// }

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
