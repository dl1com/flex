use codeword::Codeword;
use apply_bch_and_parity::apply_bch_and_parity;

struct CWMessageAlphaChars {
    chars: [u8; 3]
}

impl CWMessageAlphaChars {
    fn new (chars: [u8; 3]) -> Result<CWMessageAlphaChars, &'static str>
    {
        Ok(CWMessageAlphaChars{
            chars: chars
        })
    }
}

impl Codeword for CWMessageAlphaChars {
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
    fn test_message_alpha_chars() {
        let msg_chars = CWMessageAlphaChars::new([0x42, 0x23, 0x05]).unwrap();
        assert_eq!(msg_chars.get_codeword() & 0x1FFFFF, 0x151C2);
    }
}