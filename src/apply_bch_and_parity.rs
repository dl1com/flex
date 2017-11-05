use bch_calculator::apply_bch_checksum;
use parity::check_and_set_parity;

pub fn apply_bch_and_parity(codeword: u32) -> u32
{
    let mut cw = codeword;
    cw = apply_bch_checksum(cw);
    check_and_set_parity(&mut cw);
    return cw;
}