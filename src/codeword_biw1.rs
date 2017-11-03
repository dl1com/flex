use apply_checksums::apply_checksums;

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
        cw = apply_checksums(cw);
        
        let biw1 = BIW1 {codeword: cw};
        Ok(biw1)
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