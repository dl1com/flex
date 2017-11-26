use super::codeword::Codeword;
use apply_bch_and_parity::apply_bch_and_parity;

const SHORT_ADDRESS_CAPCODE_OFFSET: u32 = 0x8000;

pub struct CWAddressShort {
    capcode: u32,
}

impl CWAddressShort {
    pub fn new(capcode: u32) -> Result<CWAddressShort, &'static str> {
        if capcode >= 0x0001 && capcode <= 0x1EA7FF {
            Ok(CWAddressShort { capcode: capcode })
        } else {
            Err("CAPCODE for short address out of range")
        }
    }
}

impl Codeword for CWAddressShort {
    fn get_codeword(&self) -> u32 {
        let mut cw: u32 = 0x0;
        cw |= self.capcode + SHORT_ADDRESS_CAPCODE_OFFSET;
        cw = apply_bch_and_parity(cw);
        return cw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cw_address_short_get_codeword_when_first_capcode() {
        let cw_address_short = CWAddressShort::new(0x0001).unwrap();
        assert_eq!(cw_address_short.get_codeword() & 0x1FFFFF, 0x008001)
    }

    #[test]
    fn test_cw_address_short_get_codeword_when_typical_with_crc() {
        let cw_address_short = CWAddressShort::new(0x0204).unwrap();
        assert_eq!(cw_address_short.get_codeword(), 0xBF008204)
    }

    #[test]
    fn test_cw_address_short_get_codeword_when_highest_capcode_with_crc() {
        let cw_address_short = CWAddressShort::new(0x1EA7FF).unwrap();
        assert_eq!(cw_address_short.get_codeword(), 0x13DF27FF)
    }

    #[test]
    fn test_cw_address_short_get_codeword_when_zero_capcode() {
        assert_eq!(CWAddressShort::new(0x0).is_err(), true);
    }

    #[test]
    fn test_cw_address_short_get_codeword_when_invalid_capcode() {
        assert_eq!(CWAddressShort::new(0x1EA800).is_err(), true);
    }
}
