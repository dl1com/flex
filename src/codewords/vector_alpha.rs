use super::codeword::Codeword;
use apply_bch_and_parity::apply_bch_and_parity;
use fourbit_checksum::apply_4bit_checksum;

pub struct CWVectorAlpha {
    message_start: u32,
    message_words: u32,
}

impl CWVectorAlpha {
    pub fn new(message_start: u32, message_words: u32) -> Result<CWVectorAlpha, &'static str> {
        if message_start >= 3 && message_start <= 87 && message_words >= 1 && message_words <= 85 {
            Ok(CWVectorAlpha {
                message_start: message_start,
                message_words: message_words,
            })
        } else {
            Err("CWVectoralpha: Parameters out of range.")
        }
    }
}

impl Codeword for CWVectorAlpha {
    fn get_codeword(&self) -> u32 {
        let mut cw: u32 = 0x0;
        cw |= 0x5 << 4; // Type: Alpha Message Vector
        cw |= self.message_start << 7;
        cw |= self.message_words << 14;
        cw = apply_4bit_checksum(cw);
        cw = apply_bch_and_parity(cw);
        return cw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cw_vector_alphanum_lowest() {
        let cw_vector_alphanum = CWVectorAlpha::new(3, 1).unwrap();
        assert_eq!(cw_vector_alphanum.get_codeword() & 0x1FFFF0, 0x0041D0)
    }

    #[test]
    fn test_cw_vector_alphanum_highest() {
        let cw_vector_alphanum = CWVectorAlpha::new(87, 85).unwrap();
        assert_eq!(cw_vector_alphanum.get_codeword() & 0x1FFFF0, 0x156BD0)
    }

    #[test]
    fn test_cw_vector_alphanum_crc() {
        let cw_vector_alphanum = CWVectorAlpha::new(3, 49).unwrap();
        assert_eq!(cw_vector_alphanum.get_codeword(), 0xD98C41D1)
    }

    #[test]
    fn test_cw_vector_alphanum_out_of_range_1() {
        assert_eq!(CWVectorAlpha::new(0, 1).is_err(), true);
    }

    #[test]
    fn test_cw_vector_alphanum_out_of_range_2() {
        assert_eq!(CWVectorAlpha::new(88, 1).is_err(), true);
    }

    #[test]
    fn test_cw_vector_alphanum_out_of_range_3() {
        assert_eq!(CWVectorAlpha::new(3, 0).is_err(), true);
    }

    #[test]
    fn test_cw_vector_alphanum_out_of_range_4() {
        assert_eq!(CWVectorAlpha::new(3, 86).is_err(), true);
    }
}
