use codeword::Codeword;
use apply_bch_and_parity::apply_bch_and_parity;

pub struct CWMessageAlphaHeader {
    fragment_check: u32,
    message_continued_flag: u32,
    fragment_number: u32,
    message_number: u32,
    retrieval_flag: u32,
    mail_drop_flag: u32
}

impl CWMessageAlphaHeader {
    pub fn new (fragment_check: u32,
            message_continued_flag: u32,
            fragment_number: u32,
            message_number: u32,
            retrieval_flag: u32,
            mail_drop_flag: u32) -> Result<CWMessageAlphaHeader, &'static str>
    {
        if fragment_check <= 0x3FF
            && message_continued_flag <= 1
            && fragment_number <= 3
            && message_number <= 63
            && retrieval_flag <= 1
            && mail_drop_flag <= 1 {
                Ok(CWMessageAlphaHeader{
                    fragment_check: fragment_check,
                    message_continued_flag: message_continued_flag,
                    fragment_number: fragment_number,
                    message_number: message_number,
                    retrieval_flag: retrieval_flag,
                    mail_drop_flag: mail_drop_flag 
                })
        }
        else {
            Err("Alphanumeric Message Header: Invalid Parameter.")
        }
    }

    pub fn set_fragment_check(&mut self, fragment_check: u32) -> Result<(),&'static str> {
        if fragment_check <= 0x3FF {
            self.fragment_check = fragment_check;
            return Ok(())
        }
        return Err("Fragment check out of range");
    }
}

impl Codeword for CWMessageAlphaHeader {
    fn get_codeword(&self) -> u32 {
        let mut cw: u32 = 0x0;
        cw |= self.fragment_check;
        cw |= self.message_continued_flag << 10;
        cw |= self.fragment_number << 11;
        cw |= self.message_number << 13;
        cw |= self.retrieval_flag << 19;
        cw |= self.mail_drop_flag << 20;
        cw = apply_bch_and_parity(cw);
        return cw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_alpha_header_1() {
        let msg_header = CWMessageAlphaHeader::new(0x2AB,
                                                   0,
                                                   3,
                                                   63,
                                                   1,
                                                   0).unwrap();
        assert_eq!(msg_header.get_codeword() & 0x1FFFFF, 0x0FFAAB);
    }

    #[test]
    fn test_message_alpha_header_2() {
        let msg_header = CWMessageAlphaHeader::new(0x00,
                                                   0,
                                                   3,
                                                   23,
                                                   0,
                                                   0).unwrap();
        assert_eq!(msg_header.get_codeword() & 0x1FFFFF, 0x2F800);
    }

    #[test]
    fn test_message_alpha_header_invalid_fragment_check() {
        assert_eq!(CWMessageAlphaHeader::new(0x400,
                                             0,
                                             3,
                                             63,
                                             1,
                                             0).is_err(), true);
    }

    #[test]
    fn test_message_alpha_header_set_fragment_check() {
        let mut msg_header = CWMessageAlphaHeader::new(0x00,
                                                   0,
                                                   3,
                                                   23,
                                                   0,
                                                   0).unwrap();
        
        assert_eq!(msg_header.set_fragment_check(0x3FF).is_err(), false);
        assert_eq!(msg_header.get_codeword() & 0x3FF, 0x3FF);
    }

    #[test]
    fn test_message_alpha_header_set_fragment_check_out_of_range() {
        let mut msg_header = CWMessageAlphaHeader::new(0x00,
                                                   0,
                                                   3,
                                                   23,
                                                   0,
                                                   0).unwrap();
        
        assert_eq!(msg_header.set_fragment_check(0x400).is_err(), true);
    }
}