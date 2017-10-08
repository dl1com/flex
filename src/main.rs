extern crate serde_json;
extern crate bitstream_io;

mod interleaver;

use std::io;
// JSON Support
//  use serde_json::{Value, Error};
// Bitstream-IO
use bitstream_io::{BE, BitWriter};

// Sync1
const FLEX_BS1: u32 = 0xAAAAAAAA;
const FLEX_A1: u32 = 0x9C9ACF1E; // 1600 / 2 FM
//const FLEX_A2: u32 = 0x9C9AE721; // 3200 / 2 FM
//const FLEX_A3: u32 = 0x9C9AE9F2; // 3200 / 4 FM
//const FLEX_A4: u32 = 0x9C9AFA84; // 6400 / 4 FM
const FLEX_B: u16 = 0x5555;
// Sync2
const FLEX_BS2: u8 = 0x05;
const FLEX_C: u16 = 0x7B12;

// TODO Write Builders for Header Structs
// https://crates.io/crates/derive_builder
// https://docs.rs/derive_builder/0.5.0/derive_builder/
// -> Do sanity checks for fields in setters

struct FlexHeaderSync1 {
    bs1: u32,   // 32 bit bitsync
    a: u32,     // 32 bit speed indication
    b: u16,     // 16 bit bitsync
}

struct FlexHeaderSync2 {
    bs2: u8,    // 4 bit bitsync
    c: u16,     // 16 bit speed indication
}

struct FlexHeaderFrameInfo {
    x: u8,              // 4 bit checksum
    cycle_number: u8,   // 4 bit cycle number (0 to 14)
    frame_number: u8,   // 7 bit frame number (0 to 127)
    // n reserved bit
    r: bool,            // repeat paging indicator
    t: u8,              // 4 bit indicator;
                        //      r=1 Repeat format on t0-3,
                        //      r=0 Low traffic per phase (D/C/B/A)
    crc: u16,           // 10 bit CRC
    p: bool             // 1 bit parity 
}
 
fn get_header_sync1_vector(header: FlexHeaderSync1) -> Vec<u8>{
    let mut vector: Vec<u8> = Vec::new();
    {
        let mut writer = BitWriter::<BE>::new(&mut vector);
        writer.write(32, header.bs1).unwrap();
        writer.write(32, header.a).unwrap();
        writer.write(16, header.b).unwrap();
        writer.write(32, !header.a).unwrap();
        writer.into_unwritten();
    }
    return vector;
}

fn get_header_sync2_vector(header: FlexHeaderSync2) -> Vec<u8>{
    let mut vector: Vec<u8> = Vec::new();
    {
        let mut writer = BitWriter::<BE>::new(&mut vector);
        writer.write(4, header.bs2).unwrap();
        writer.write(16, header.c).unwrap();
        writer.write(4, 0xf & !header.bs2).unwrap(); // Strange things happen when writing an inverted nibble without masking
        writer.write(16, !header.c).unwrap();
        writer.into_unwritten();
    }
    return vector;
}

fn get_header_frameinfo_vector(header: FlexHeaderFrameInfo) -> Vec<u8>{
    let mut vector: Vec<u8> = Vec::new();
    {
        let mut writer = BitWriter::<BE>::new(&mut vector);
        writer.write(4, 0xf & header.x).unwrap();
        writer.write(4, 0xf & header.cycle_number).unwrap();
        writer.write(7, 0x7f & header.frame_number).unwrap();
        writer.write_bit(false).unwrap();
        writer.write_bit(header.r).unwrap();
        writer.write(4, 0xf & header.t).unwrap();
        writer.write(10, 0x3ff & header.crc).unwrap();
        writer.write_bit(header.p).unwrap();
        writer.into_unwritten();

    }
    return vector;
}

fn main() {
    let test_data: [u32; 8] = [0; 8];
    let result = interleaver::interleave_codewords_1600(test_data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_header_sync1_vector() {
        let flex_header_sync1 = FlexHeaderSync1 {
            bs1: FLEX_BS1,
            a: FLEX_A1,
            b: FLEX_B
        };

        let result = get_header_sync1_vector(flex_header_sync1);

        assert_eq!(result, vec![0xAA,0xAA,0xAA,0xAA,
                                        0x9C,0x9A,0xCF,0x1E,
                                        0x55,0x55,
                                        0x63,0x65,0x30,0xE1]);
    }

    #[test]
    fn test_get_header_frameinfo_vector() {
        let flex_header_frameinfo = FlexHeaderFrameInfo {
            x: 0x1,
            cycle_number: 0x2,
            frame_number: 0x34,
            r: true,
            t: 0xF,
            crc: 0x3FF,
            p: true
        };

        let result = get_header_frameinfo_vector(flex_header_frameinfo);

        assert_eq!(result, vec![0x12, 0x68, 0xFF, 0xFF]);
    }

    #[test]
    fn test_get_header_sync2_vector() {
        let flex_header_sync2 = FlexHeaderSync2 {
            bs2: FLEX_BS2, c: FLEX_C
        };
        let result = get_header_sync2_vector(flex_header_sync2);
        println!("{:?}", result);
        assert_eq!(result, vec![0x57,0xB1,0x2A,
                                0x84,0xED]);
    }
}
