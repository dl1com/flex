use codeword::Codeword;
use apply_bch_and_parity::apply_bch_and_parity;

pub struct CWMessageAlphaHeader {
    message_continued_flag: u32,
    fragment_number: u32,
    message_number: u32,
    retrieval_flag: u32,
    mail_drop_flag: u32
}

impl CWMessageAlphaHeader {
    pub fn new (message_continued_flag: u32,
            fragment_number: u32,
            message_number: u32,
            retrieval_flag: u32,
            mail_drop_flag: u32) -> Result<CWMessageAlphaHeader, &'static str>
    {
        if message_continued_flag <= 1
            && fragment_number <= 3
            && message_number <= 63
            && retrieval_flag <= 1
            && mail_drop_flag <= 1 {
                Ok(CWMessageAlphaHeader{message_continued_flag: message_continued_flag,
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
}

impl Codeword for CWMessageAlphaHeader {
    fn get_codeword(&self) -> u32 {
        let mut cw: u32 = 0x0;
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
        let msg_header = CWMessageAlphaHeader::new(0,
                                                   3,
                                                   63,
                                                   1,
                                                   0).unwrap();
        assert_eq!(msg_header.get_codeword() & 0x1FFA00, 0x0FF800);
    }

    #[test]
    fn test_message_alpha_header_2() {
        let msg_header = CWMessageAlphaHeader::new(0,
                                                   3,
                                                   23,
                                                   0,
                                                   0).unwrap();
        assert_eq!(msg_header.get_codeword() & 0x1FFA00, 0x2F800);
    }
}