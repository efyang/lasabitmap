extern crate bit_vec;
extern crate bitstream;
mod parse;
mod node;
mod coord;
mod graph;
use graph::Graph;

fn main() {
    let mut data = parse::parse();
    println!("len: {}, width: {}", data.len(), data[0].len());
    let mut graph = Graph::new(&mut data);
    let mut cont = true;
    while cont {
        match graph.iterate() {
            Some(_) => {},
            None => {cont = false},
        }
    }
    println!("Reachable mowable spots: {}", graph.get_count());
}
