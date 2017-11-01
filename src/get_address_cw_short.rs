use bch_calculator::apply_bch_checksum;
use parity::check_and_set_parity;

pub fn get_address_cw_short(address: u32) -> Result<u32, &'static str>
{
    if address >= 0x1 && address <= 0x1F27FF 
        {
            let mut codeword: u32 = 0x0;
            codeword += address;
            codeword = apply_bch_checksum(codeword);
            check_and_set_parity(&mut codeword);
            Ok(codeword)
        }
    else {
            Err("Short address out of range")
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_address_cw_short_lowest() {
        assert_eq!(get_address_cw_short(0x1).unwrap(),
                   0x96E00001)
    }

    #[test]
    fn test_get_address_cw_short_highest() {
        assert_eq!(get_address_cw_short(0x1F27FF).unwrap(),
                   0x13DF27FF)
    }

    #[test]
    fn test_get_address_cw_short_zero() {
        assert_eq!(get_address_cw_short(0x0).unwrap_err(),
                   "Short address out of range")
    }

    #[test]
    fn test_get_address_cw_short_toohigh() {
        assert_eq!(get_address_cw_short(0x1F2800).unwrap_err(),
                   "Short address out of range")
    }
}