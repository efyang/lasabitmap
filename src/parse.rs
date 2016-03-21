use bitstream::*;
use std::fs::File;

pub fn parse() -> Vec<Vec<bool>> {
    let mut infile = File::open("map.dat").unwrap();
    let mut data = Vec::new();
    let mut breader: BitReader<File, BigEndian> = BitReader::new(infile);
    for _ in 0..4800 {
        let mut row = Vec::with_capacity(4800);
        for _ in 0..4800 {
            row.push(breader.read_bit().unwrap());
        }
        data.push(row);
    }
    return data;
}
