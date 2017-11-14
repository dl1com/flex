mod parity;
mod bch_calculator;
mod fourbit_checksum;
mod codeword;
mod cw_fiw;
mod cw_biw1;
mod cw_biw2;
mod cw_biw3;
mod cw_biw4;
mod cw_address_short;
mod cw_vector_alpha;
mod cw_message_alpha_header;
mod cw_message_alpha_signature;
mod cw_message_alpha_content;
mod cw_message_alpha;
mod frame;
mod blocks;
mod apply_bch_and_parity;
mod message;

use frame::Frame;
use message::Message;
use message::MessageType;

use std::fs::File;
use std::io::prelude::*;

extern crate bit_reverse;
use bit_reverse::ParallelReverse;

fn main() {

    let mut frames = Vec::new();
    for cycle in 0..1 {
        for frame in 0..1 {
            let msg = Message::new(MessageType::AlphaNum,
                                   0x42083,
                                   String::from("das pferd isst keinen gurkensalat")).unwrap();
            let mut frame = Frame::new(cycle, frame).unwrap();
            frame.add_message(msg).unwrap();
            frames.push(frame);
        }
    }

    let mut file = File::create("/tmp/dump.bin").unwrap();
    for frame in frames {
        let bytes = frame.get_bytes();
        let mut rotated_bytes = Vec::new();
        for byte in bytes {
            rotated_bytes.push(byte.swap_bits());
        }

        println!("{:?}", rotated_bytes);        
        file.write_all(&rotated_bytes).unwrap();
    }
}
