use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct  Frequency(pub char, pub usize);
pub type Frequencies = Vec<Frequency>;
pub struct FrequencyStruct {
    pub data: Frequencies
}

impl FrequencyStruct {
    fn new(len: usize) -> FrequencyStruct {
        FrequencyStruct { data: Vec::with_capacity(len) }
    }

    /// Retrieves a node's index. 
    /// 
    /// This checks if a `DataFrequency` is already added for a data point and if so, 
    /// returns the frequency's index. Returns `None` if the frequency doesn't exist.
    fn get_data_index(&self, b: &char) -> Option<usize>{
        let mut index = None;
        let nodes = &self.data;

        for (i, n) in nodes.iter().enumerate() {
            if n.0 == *b {
                index = Some(i);
                break;
            }
        }

        index
    }

    /// Increments weight of node
    fn increase_data_weight(&mut self, i: usize) {
        let node = self.data.get_mut(i).unwrap();
        node.1 += 1;
    }

    /// Creates a new node
    fn insert_new_tree(&mut self, data: char){
        self.data.insert(0, Frequency(data, 1));
    }
}


pub fn frequency_counter(data: &Vec<u8>) -> Option<FrequencyStruct> {
    if data.is_empty() {
        return None
    }

    let mut queue = FrequencyStruct::new(data.len());

    for b in data {
        match queue.get_data_index(&(*b as char)) {
            Some(i) => queue.increase_data_weight(i),
            None => queue.insert_new_tree(*b as char)
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
        let q = frequency_counter(&data);
        assert!(q.is_some());

        Ok(())
    }
}