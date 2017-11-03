use apply_checksums::apply_checksums;

struct BIW3 {
    codeword: u32
}

impl BIW3
{
    fn new (day: u32,
            month: u32,
            year: u32) -> Result<BIW3, &'static str>
    {
        if !(BIW3::check_day(day)
             && BIW3::check_month(month)
             && BIW3::check_year(year))
        {
            Err("BIW3: Invalid date given")
        }
        else
        {
            let mut cw: u32 = 0x0;
            cw += 0x1 << 4;
            cw += (year & 0x1F) << 7;
            cw += (day & 0x1F) << 12;
            cw += (month & 0xF) << 17;
            cw = apply_checksums(cw);

            let biw3 = BIW3 {codeword: cw};
            Ok(biw3)
        }
    }

    fn check_day(day: u32) -> bool {
        return day >= 1 && day <= 31
    }

    fn check_month(month: u32) -> bool {
        return month >= 1 && month <= 12
    }

    fn check_year(year: u32) -> bool {
        return year <= 0x1F
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codeword_biw3() {
        let biw3 = BIW3::new(31, 12, 5).unwrap();
        assert_eq!(biw3.codeword & 0x1FFFFF, 0x19F29B);
    }

    #[test]
    fn test_codeword_biw3_invalid_day() {
        assert_eq!(BIW3::new(32, 12, 5).is_err(), true);
    }

    #[test]
    fn test_codeword_biw3_invalid_month() {
        assert_eq!(BIW3::new(31, 13, 5).is_err(), true);
    }

    #[test]
    fn test_codeword_biw3_invalid_day_0() {
        assert_eq!(BIW3::new(0, 12, 5).is_err(), true);
    }

    #[test]
    fn test_codeword_biw3_invalid_month_0() {
        assert_eq!(BIW3::new(31, 0, 5).is_err(), true);
    }
}