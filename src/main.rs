extern crate bitstream_io;

mod interleaver;
mod parity;
mod header_builder;
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
mod block;
mod apply_bch_and_parity;

use frame::Frame;

use std::fs::File;
use std::io::prelude::*;


fn main() {

    let frame = Frame::new(0, 0).unwrap();
    let bytes = frame.get_bytes();
    println!("{:?}", bytes);

    let mut file = File::create("/tmp/frame.dat").unwrap();
    file.write_all(&bytes).unwrap();
}
