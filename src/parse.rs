use std::fs::File;
use node::*;
use bitstream::*;

pub fn parse() -> Vec<Vec<Node>> {
    let infile = File::open("map.dat").unwrap();
    //let mut indata = Vec::with_capacity(4800*(4800/8));
    //infile.read_to_end(&mut indata).unwrap();
    //let mut data = Vec::with_capacity(4800 + 2);
    //for a in 0..4800 {
        //let slice = &indata[a..a+4800/8];
        //let bv = BitVec::from_bytes(slice);
        //let newvec = bv.iter().map(|e| Node::new(e)).collect::<Vec<Node>>();
        //data.push(newvec);
    //}
    let mut data = Vec::with_capacity(4800 + 2);
    let mut breader: BitReader<File, BigEndian> = BitReader::new(infile);
    for _ in 0..4800 {
        let mut buf = Vec::with_capacity(4800);
        for _ in 0..4800 {
            buf.push(Node::new(breader.read_bit().unwrap()));
        }
        data.push(buf);
    }
    //println!("{}", data.len());

    return data;
}
