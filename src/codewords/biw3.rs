use super::codeword::Codeword;
use apply_bch_and_parity::apply_bch_and_parity;
use fourbit_checksum::apply_4bit_checksum;

pub struct BIW3 {
    year: u32,
    month: u32,
    day: u32,
}

impl BIW3 {
    pub fn new(day: u32, month: u32, year: u32) -> Result<BIW3, &'static str> {
        if !(BIW3::check_day(day) && BIW3::check_month(month) && BIW3::check_year(year)) {
            Err("BIW3: Invalid date given")
        } else {
            let biw3 = BIW3 {
                year: year - 1994,
                month: month,
                day: day,
            };
            Ok(biw3)
        }
    }

    fn check_day(day: u32) -> bool {
        return day >= 1 && day <= 31;
    }

    fn check_month(month: u32) -> bool {
        return month >= 1 && month <= 12;
    }

    fn check_year(year: u32) -> bool {
        return year >= 1994 && year <= 2025;
    }
}

impl Codeword for BIW3 {
    fn get_codeword(&self) -> u32 {
        let mut cw: u32 = 0x0;
        cw |= 0x1 << 4;
        cw |= self.year << 7;
        cw |= self.day << 12;
        cw |= self.month << 17;
        cw = apply_4bit_checksum(cw);
        cw = apply_bch_and_parity(cw);
        return cw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codeword_biw3() {
        let biw3 = BIW3::new(31, 12, 1999).unwrap();
        assert_eq!(biw3.get_codeword() & 0x1FFFF0, 0x19F290);
    }

    #[test]
    fn test_codeword_biw3_invalid_day() {
        assert_eq!(BIW3::new(32, 12, 1999).is_err(), true);
    }

    #[test]
    fn test_codeword_biw3_invalid_month() {
        assert_eq!(BIW3::new(31, 13, 1999).is_err(), true);
    }

    #[test]
    fn test_codeword_biw3_invalid_day_0() {
        assert_eq!(BIW3::new(0, 12, 1999).is_err(), true);
    }

    #[test]
    fn test_codeword_biw3_invalid_month_0() {
        assert_eq!(BIW3::new(31, 0, 1999).is_err(), true);
    }

    #[test]
    fn test_codeword_biw3_invalid_year_low() {
        assert_eq!(BIW3::new(1, 1, 1993).is_err(), true);
    }

    #[test]
    fn test_codeword_biw3_invalid_year_high() {
        assert_eq!(BIW3::new(1, 1, 2026).is_err(), true);
    }
}
