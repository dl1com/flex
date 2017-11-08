use codeword::Codeword;
use apply_bch_and_parity::apply_bch_and_parity;
use fourbit_checksum::apply_4bit_checksum;

pub struct BIW1 {
    priority_addr: u32,
    end_of_block: u32,
    vector_field_start: u32,
    carry_on: u32,
    frame_id_collapse_mark: u32
}

impl BIW1 {
    pub fn new (priority_addr: u32,
            end_of_block: u32,
            vector_field_start: u32,
            carry_on: u32,
            frame_id_collapse_mark: u32) -> Result<BIW1,&'static str>
    {
        let biw1 = BIW1 {
            priority_addr: priority_addr & 0xF,
            end_of_block: end_of_block & 0x3,
            vector_field_start: vector_field_start & 0x3F,
            carry_on: carry_on & 0x3,
            frame_id_collapse_mark: frame_id_collapse_mark & 0x7};
        Ok(biw1)
    }
}

impl Codeword for BIW1 {
    fn get_codeword(&self) -> u32 {
        let mut cw: u32 = 0x0;
        cw |= self.priority_addr << 4;
        cw |= self.end_of_block << 8;
        cw |= self.vector_field_start << 10;
        cw |= self.carry_on << 16;
        cw |= self.frame_id_collapse_mark << 18;
        cw = apply_4bit_checksum(cw);
        cw = apply_bch_and_parity(cw);
        return cw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codeword_biw1() {
        let biw1 = BIW1::new(10,2,60,1,6).unwrap();
        assert_eq!(biw1.get_codeword() & 0x1FFFF0, 0x19F2A0);
    }
}