use fourbit_checksum::apply_4bit_checksum;
use bch_calculator::apply_bch_checksum;
use parity::check_and_set_parity;


pub fn apply_checksums(codeword: u32) -> u32
{
    let mut cw = codeword;
    cw = apply_4bit_checksum(cw);
    cw = apply_bch_checksum(cw);
    check_and_set_parity(&mut cw);
    return cw;
}