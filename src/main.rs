extern crate bitstream;
mod parse;

fn main() {
    let data = parse::parse();
    println!("len: {}, width: {}", data.len(), data[0].len());
}
