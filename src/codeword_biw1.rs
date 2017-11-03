use fourbit_checksum::apply_4bit_checksum;
use bch_calculator::apply_bch_checksum;
use parity::check_and_set_parity;

struct BIW1
{
    codeword: u32
}

impl BIW1 {
    fn new (priority_addr: u32,
            end_of_block: u32,
            vector_field_start: u32,
            carry_on: u32,
            frame_id_collapse_mark: u32) -> Result<BIW1,&'static str>
    {
        let mut cw: u32 = 0x0;
        cw += (priority_addr & 0xF) << 4;
        cw += (end_of_block & 0x3) << 8;
        cw += (vector_field_start & 0x3F) << 10;
        cw += (carry_on & 0x3) << 16;
        cw += (frame_id_collapse_mark & 0x7) << 18;

        let mut biw1 = BIW1 {codeword: cw};
        biw1.apply_checksums();
        Ok(biw1)
    }

    fn apply_checksums(&mut self)
    {
        let mut cw = self.codeword;
        cw = apply_4bit_checksum(cw);
        cw = apply_bch_checksum(cw);
        check_and_set_parity(&mut cw);
        self.codeword = cw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codeword_biw1() {
        let biw1 = BIW1::new(10,2,60,1,6).unwrap();
        assert_eq!(biw1.codeword & 0x1FFFFF, 0x19F2AA);
    }
}