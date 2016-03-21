use bit_vec::*;
use std::fs::File;
use std::io::prelude::*;
use node::*;

pub fn parse() -> Vec<Vec<Node>> {
    let mut infile = File::open("map.dat").unwrap();
    let mut indata = Vec::with_capacity(4800*(4800/8));
    infile.read_to_end(&mut indata).unwrap();
    let mut data = Vec::with_capacity(4800 + 2);
    for a in 0..4800 {
        let slice = &indata[a..a+4800/8];
        let bv = BitVec::from_bytes(slice);
        let newvec = bv.iter().map(|e| Node::new(e)).collect::<Vec<Node>>();
        data.push(newvec);
    }
    println!("indata.len(): {}", indata.len());
    println!("required len: {}", 4800*(4800/8));

    return data;
}
