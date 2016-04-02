extern crate bit_vec;
extern crate bitstream;
#[macro_use]
extern crate bmp;
use bmp::Image;
mod parse;
mod node;
mod coord;
mod graph;
use graph::Graph;
use coord::Coord;

fn main() {
    let mut data = parse::parse();
    let mut graph = Graph::new(&mut data);
    let mut cont = true;
    while cont {
        match graph.iterate() {
            Some(_) => {},
            None => {cont = false},
        }
    }
    println!("Reachable mowable spots: {}", graph.get_count());
    println!("Generating image...");
    let mut img = Image::new(4800, 4800);
    for (x, y) in img.coordinates() {
        img.set_pixel(x, y, graph.index_data(&Coord::new(x as usize, y as usize)).make_pixel());
    }
    let _ = img.save("map.bmp");
}
