use codeword::Codeword;
use apply_bch_and_parity::apply_bch_and_parity;

struct CWAddressShort {
    address: u32
}

impl CWAddressShort {
    fn new(address: u32) -> Result<CWAddressShort, &'static str> {
        if address >= 0x1 && address <= 0x1F27FF 
            {
                Ok(CWAddressShort{address: address})
            }
        else {
                Err("Short address out of range")
            }
    }
}

impl Codeword for CWAddressShort {
    fn get_codeword(&self) -> u32 {
        let mut cw: u32 = 0x0;
        cw |= self.address;
        cw = apply_bch_and_parity(cw);
        return cw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_address_cw_short_lowest() {
        let cw_address_short = CWAddressShort::new(0x1).unwrap();
        assert_eq!(cw_address_short.get_codeword(), 0x96E00001)
    }

    #[test]
    fn test_get_address_cw_short_highest() {
        let cw_address_short = CWAddressShort::new(0x1F27FF).unwrap();
        assert_eq!(cw_address_short.get_codeword(), 0x13DF27FF)
    }

    #[test]
    fn test_get_address_cw_short_zero() {
        assert_eq!(CWAddressShort::new(0x0).is_err(), true);
    }

    #[test]
    fn test_get_address_cw_short_toohigh() {
        assert_eq!(CWAddressShort::new(0x1F2800).is_err(), true);
    }
}