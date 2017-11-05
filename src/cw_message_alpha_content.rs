use codeword::Codeword;
use apply_bch_and_parity::apply_bch_and_parity;

use std::str;

pub struct CWMessageAlphaContent {
    chars: Vec<u8>
}

impl CWMessageAlphaContent {
    pub fn new (chars: &[u8]) -> Result<CWMessageAlphaContent, &'static str>
    {
        if chars.len() != 3 {
            return Err("Alphanumeric Message Content: 3 chars required.");
        }
        println!("{:?}", str::from_utf8(chars).unwrap());
        return Ok(CWMessageAlphaContent{
            chars: chars.to_vec()
        });
    }
}

impl Codeword for CWMessageAlphaContent {
    fn get_codeword(&self) -> u32 {
        let mut cw: u32 = 0x0;
        cw |= (self.chars[0] & 0x7F) as u32;
        cw |= ((self.chars[1] & 0x7F) as u32) << 7;
        cw |= ((self.chars[2] & 0x7F) as u32) << 14;
        cw = apply_bch_and_parity(cw);
        return cw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_alpha_content() {
        let chars = vec![0x42, 0x23, 0x05];
        let msg_chars = CWMessageAlphaContent::new(&chars).unwrap();
        assert_eq!(msg_chars.get_codeword() & 0x1FFFFF, 0x151C2);
    }

    #[test]
    fn test_message_alpha_content_too_few_chars() {
        let chars = vec![0x42, 0x23];
        assert_eq!(CWMessageAlphaContent::new(&chars).is_err(), true);
    }

    #[test]
    fn test_message_alpha_content_too_many_chars() {
        let chars = vec![0x42, 0x23, 0x05, 0x10];
        assert_eq!(CWMessageAlphaContent::new(&chars).is_err(), true);
    }
}