use crate::modules::counter::data_frequencies;

mod modules;

fn main() {
    let freq = data_frequencies("test.txt").unwrap();
    for n in freq.nodes {
        println!("{}: {}", n.datum as char, n.count);
    }
}
