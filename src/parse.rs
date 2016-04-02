use std::fs::File;
use node::*;
use bitstream::*;

pub fn parse() -> Vec<Vec<Node>> {
    let infile = File::open("map.dat").unwrap();
    let mut data = Vec::with_capacity(4800 + 2);
    let mut breader: BitReader<File, BigEndian> = BitReader::new(infile);
    for _ in 0..4800 {
        let mut buf = Vec::with_capacity(4800);
        for _ in 0..4800 {
            buf.push(Node::new(breader.read_bit().unwrap()));
        }
        data.push(buf);
    }
    return data;
}
