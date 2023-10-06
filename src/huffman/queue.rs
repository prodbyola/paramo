use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct  Frequency(pub u8, pub usize);
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
    fn get_data_index(&self, b: &u8) -> Option<usize>{
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
    fn insert_new_tree(&mut self, data: u8){
        self.data.insert(0, Frequency(data, 1));
    }

    // pub fn pull_data(&self) -> Vec<(u8, usize)> {
    //     let mut data = Vec::with_capacity(self.data.len());
    //     for f in &self.data {
    //         data.push((f.0, f.1));
    //     }

    //     data
    // }
}


pub fn frequency_counter(data: &Vec<u8>) -> Option<FrequencyStruct> {
    if data.is_empty() {
        return None
    }

    let mut queue = FrequencyStruct::new(data.len());

    for b in data {
        match queue.get_data_index(b) {
            Some(i) => queue.increase_data_weight(i),
            None => queue.insert_new_tree(*b)
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