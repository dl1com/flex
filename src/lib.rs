#[macro_use]
extern crate serde_derive;

mod parity;
mod bch_calculator;
mod fourbit_checksum;
mod apply_bch_and_parity;
mod blocks;

mod codewords;

pub mod message;
pub mod frame;