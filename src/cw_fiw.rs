use codeword::Codeword;
use apply_bch_and_parity::apply_bch_and_parity;
use fourbit_checksum::apply_4bit_checksum;

pub struct FIW {
    cycle_number: u32,
    frame_number: u32,
    repeat_paging: u32,
    low_traffic_flags: u32
}

impl FIW {
    pub fn new (cycle_number: u32,
            frame_number: u32,
            repeat_paging: u32,
            low_traffic_flags: u32) -> Result<FIW,&'static str>
    {
        // Currently handling only low_traffic_flag for 1600 bps
        if cycle_number <= 14
            && frame_number <= 127
            && repeat_paging <= 1
            {
                Ok(FIW{
                    cycle_number: cycle_number,
                    frame_number: frame_number,
                    repeat_paging: repeat_paging,
                    low_traffic_flags: low_traffic_flags
            })
        }
        else {
            Err("Frame Information Word: Parameter out of range.")
        }
    }
}

impl Codeword for FIW {
    fn get_codeword(&self) -> u32 {
        let mut cw: u32 = 0x0;
        cw |= self.cycle_number << 4;
        cw |= self.frame_number << 8;
        cw |= self.repeat_paging << 16;
        cw |= self.low_traffic_flags << 17;
        cw = apply_4bit_checksum(cw);
        cw = apply_bch_and_parity(cw);
        return cw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codeword_fiw() {
        let fiw = FIW::new(3, 60, 0, 8).unwrap();
        assert_eq!(fiw.get_codeword() & 0x1FFFF0, 0x103C30);
    }

    #[test]
    fn test_codeword_fiw_out_of_range_cycle() {
        assert_eq!(FIW::new(15, 60, 0, 8).is_err(), true);
    }

    #[test]
    fn test_codeword_fiw_out_of_range_frame() {
        assert_eq!(FIW::new(14, 128, 0, 8).is_err(), true);
    }

    #[test]
    fn test_codeword_fiw_out_of_range_repeat() {
        assert_eq!(FIW::new(14, 60, 2, 8).is_err(), true);
    }

    #[test]
    fn test_codeword_fiw_test_crc() {
        let fiw = FIW::new(3, 107, 0, 0).unwrap();
        assert_eq!(fiw.get_codeword(), 0xE4A06B3B);
    }
}