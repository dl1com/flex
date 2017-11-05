use codeword::Codeword;
use apply_checksums::apply_checksums;

struct BIW2 {
    local_id: u32,
    timezone: u32
}

impl BIW2 {
    fn new (local_id: u32,
            timezone: u32) -> Result<BIW2, &'static str>
    {
        let biw2 = BIW2 {
            local_id: local_id & 0x1FF,
            timezone: timezone & 0x1F
        };
        Ok(biw2)
    }
}

impl Codeword for BIW2 {
    fn get_codeword(&self) -> u32 {
        let mut cw: u32 = 0x0;
        cw |= self.timezone << 7;
        cw |= self.local_id << 12;
        cw = apply_checksums(cw);
        return cw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codeword_biw2() {
        let biw2 = BIW2::new(0x1FF,1).unwrap();
        assert_eq!(biw2.get_codeword() & 0x1FFFFF, 0x1FF088);
    }
}