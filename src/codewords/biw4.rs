use super::codeword::Codeword;
use helper::apply_bch_and_parity::apply_bch_and_parity;
use helper::fourbit_checksum::apply_4bit_checksum;

pub struct BIW4 {
    hour: u32,
    minute: u32,
    second: u32,
}

impl BIW4 {
    pub fn new(hour: u32, minute: u32, second: u32) -> Result<BIW4, &'static str> {
        if !(BIW4::check_minute(minute) && BIW4::check_hour(hour)) {
            Err("BIW3: Invalid time given")
        } else {
            let biw4 = BIW4 {
                hour: hour,
                minute: minute,
                second: (second as f32 / 7.5) as u32 & 0x7,
            };
            Ok(biw4)
        }
    }

    fn check_minute(minute: u32) -> bool {
        return minute <= 59;
    }

    fn check_hour(hour: u32) -> bool {
        return hour <= 23;
    }
}

impl Codeword for BIW4 {
    fn get_codeword(&self) -> u32 {
        let mut cw: u32 = 0x0;
        cw |= 0x2 << 4;
        cw |= self.hour << 7;
        cw |= self.minute << 12;
        cw |= self.second << 18;
        cw = apply_4bit_checksum(cw);
        cw = apply_bch_and_parity(cw);
        return cw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codeword_biw4_1() {
        let biw4 = BIW4::new(23, 59, 59).unwrap();
        assert_eq!(biw4.get_codeword() & 0x1FFFF0, 0x1FBBA0);
    }

    #[test]
    fn test_codeword_biw4_2() {
        let biw4 = BIW4::new(23, 59, 31).unwrap();
        assert_eq!(biw4.get_codeword() & 0x1FFFF0, 0x13BBA0);
    }

    #[test]
    fn test_codeword_biw4_3() {
        let biw4 = BIW4::new(0, 0, 0).unwrap();
        assert_eq!(biw4.get_codeword() & 0x1FFFF0, 0x000020);
    }

    #[test]
    fn test_codeword_biw4_invalid_hour() {
        assert_eq!(BIW4::new(24, 59, 59).is_err(), true);
    }

    #[test]
    fn test_codeword_biw4_invalid_minute() {
        assert_eq!(BIW4::new(23, 60, 59).is_err(), true);
    }
}
