extern crate bit_vec;
extern crate bitstream;
extern crate image;
mod parse;
mod node;
mod coord;
mod graph;
use graph::Graph;
use std::path::Path;

fn main() {
    println!("Parsing file...");
    let mut data = parse::parse();
    let mut graph = Graph::new(&mut data);
    let mut cont = true;
    println!("Traversing graph...");

    while cont {
        match graph.iterate() {
            Some(_) => {}
            None => cont = false,
        }
    }
    println!("Reachable mowable spots: {}", graph.get_count());
    println!("Generating png...");
    let finalframe = graph.generate_imagebuffer();
    let finalpath = Path::new("map.png");
    finalframe.save(finalpath).unwrap();
    println!("Done.");
}
