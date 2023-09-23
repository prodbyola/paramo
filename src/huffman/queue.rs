use super::heap::HuffmanNode;

pub struct PriorityQueue {
    pub nodes: Vec<HuffmanNode>
}

impl PriorityQueue {
    fn new(len: usize) -> PriorityQueue {
        PriorityQueue { nodes: Vec::with_capacity(len) }
    }

    /// Retrieves a node's index. 
    /// 
    /// This checks if a `DataFrequency` is already added for a data point and if so, 
    /// returns the frequency's index. Returns `None` if the frequency doesn't exist.
    fn get_node_index(&self, b: &u8) -> Option<usize>{
        let mut index = None;
        let nodes = &self.nodes;

        for (i, n) in nodes.iter().enumerate() {
            if let Some(v) = n.value() {
                if v == *b {
                    index = Some(i);
                    break;
                }
            }
        }

        index
    }

    /// Increments weight of node
    fn increase_node_weight(&mut self, i: usize) {
        let len = self.nodes.len();

        let node = self.nodes.get_mut(i).unwrap();
        node.add_weight();

        let tree = node.clone();

        if len > 1 {
            let last_index = len - 1; 

            // check if there is a tree with lesser weight ahead and if
            // it exists, swap it for inserted tree. This helps keep queue
            // priority order
            let mut swappable = None;
            for j in i+1..=last_index {
                if let Some(next) = self.nodes.get(j) {
                    if &tree > next {
                        swappable = Some(j);
                    }
                }
            }

            if let Some(s) = swappable {
                self.nodes.swap(i, s);
            }
        }
    }

    /// Creates a new node
    fn insert_new_tree(&mut self, data: u8){
        self.nodes.insert(0, HuffmanNode::init(data));
    }   
}


pub fn create_queue(data: Vec<u8>) -> Option<PriorityQueue> {
    if data.is_empty() {
        return None
    }

    let mut queue = PriorityQueue::new(data.len());

    for b in data {
        match queue.get_node_index(&b) {
            Some(i) => queue.increase_node_weight(i),
            None => queue.insert_new_tree(b)
        }
    }

    Some(queue)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read;
    use std::io::Result as ioResult;

    #[test]
    fn test_create_queue() -> ioResult<()>{
        let data = read("test.txt")?;
        let q = create_queue(data);
        assert!(q.is_some());

        Ok(())
    }
}