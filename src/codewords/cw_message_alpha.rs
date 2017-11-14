use super::codeword::Codeword;
use super::cw_message_alpha_header::CWMessageAlphaHeader;
use super::cw_message_alpha_signature::CWMessageAlphaSignature;
use super::cw_message_alpha_content::CWMessageAlphaContent;

use apply_bch_and_parity::apply_bch_and_parity;

pub struct CWMessageAlpha {
    header: CWMessageAlphaHeader,
    signature: CWMessageAlphaSignature,
    content: Vec<CWMessageAlphaContent>
}

impl CWMessageAlpha {
    pub fn new (message_number: u32,
            chars: &[u8]) -> Result<CWMessageAlpha, &'static str>
    {
        // Not supported for now:
        // - Fragmented messages
        // - Message retrieval flag
        // - Mail Drop flag

        let header = CWMessageAlphaHeader::new(0,
                                               3,
                                               message_number,
                                               0,
                                               0).unwrap();
        let mut chars_filled = chars.to_vec();
        CWMessageAlpha::fill_up_chars(&mut chars_filled);
        let signature = CWMessageAlphaSignature::new(
                            CWMessageAlpha::calculate_signature(chars),
                            &chars_filled[0..2]).unwrap();
        
        let mut content = Vec::new();
        for i in 0..((chars_filled.len()-2) / 3) {        
           content.push(CWMessageAlphaContent::new(
                        &chars_filled[2+i*3..5+i*3]).unwrap());
        }
        
        return Ok(CWMessageAlpha{header: header,
                                 signature: signature,
                                 content: content});
    }

    fn fill_up_chars(chars: &mut Vec<u8>) {
        while chars.len() % 3 != 2 {
            chars.push(0x3); // ETX
        }
    }

    fn calculate_signature(chars: &[u8]) -> u32 {
        let mut sum: u32 = 0;
        for chr in chars.to_vec() {
            sum += chr as u32 & 0x7F;
        }
        sum ^= 0xFFFFFFFF;
        return sum & 0x7F;
    }

    fn calculate_fragment_check(codewords: &Vec<u32>) -> u32 {
        let mut fragment_check: u32 = 0x00;

        for codeword in codewords {
            fragment_check += CWMessageAlpha::get_bitgroup_sum(*codeword);
        }

        fragment_check ^= 0xFFFFFFFF;
        return fragment_check & 0x3FF;
    }

    fn get_bitgroup_sum(codeword: u32) -> u32 {
        let mut sum: u32 = 0x0;
        sum += codeword & 0xFF;
        sum += (codeword >> 8) & 0xFF;
        sum += (codeword >> 16) & 0x1F;
        return sum;
    }

    pub fn get_codewords(&self) -> Vec<u32> {
        let mut cws = Vec::new();
        cws.push(self.header.get_codeword());
        cws.push(self.signature.get_codeword());
        for content in &self.content {
            cws.push(content.get_codeword());
        }

        let fragment_check = CWMessageAlpha::calculate_fragment_check(&cws);
        let header = cws[0] | fragment_check;
        cws[0] = apply_bch_and_parity(header & 0x1FFFFF);
        return cws;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_alpha_calculate_signature_1() {
        let chars: [u8; 1] = [0x7F];
        let msg_signature = CWMessageAlpha::calculate_signature(&chars);
        assert_eq!(msg_signature, 0x0);
    }

    #[test]
    fn test_message_alpha_calculate_signature_3() {
        let chars: [u8; 3] = [0x7F; 3];
        let msg_signature = CWMessageAlpha::calculate_signature(&chars);
        assert_eq!(msg_signature, 0x2);
    }

    #[test]
    fn test_fill_up_chars_1() {
        let mut chars = vec![0x00];
        CWMessageAlpha::fill_up_chars(&mut chars);
        assert_eq!(chars.len(), 2);
    }

    #[test]
    fn test_fill_up_chars_3() {
        let mut chars = vec![0x00, 0x01, 0x03];
        CWMessageAlpha::fill_up_chars(&mut chars);
        assert_eq!(chars.len(), 5);
    }

    #[test]
    fn test_message_alpha_get_codewords() {
        let text_vec = Vec::from("Gurkensalat");
        let msg_alpha = CWMessageAlpha::new(23, &text_vec).unwrap();
        assert_eq!(msg_alpha.get_codewords()[0] & 0x1FFA00,0x02F800);
        assert_eq!(msg_alpha.get_codewords()[0] & 0x3FF, 0x14F );
        assert_eq!(msg_alpha.get_codewords()[1] & 0x1FFF80,0x1D6380); // Gu
        assert_eq!(msg_alpha.get_codewords()[2] & 0x1FFFFF,0x1975F2); // rke
        assert_eq!(msg_alpha.get_codewords()[3] & 0x1FFFFF,0x1879EE); // nsa
        assert_eq!(msg_alpha.get_codewords()[4] & 0x1FFFFF,0x1D30EC); // lat
    }

    #[test]
    fn test_message_alpha_get_codewords_fillup() {
        let text_vec = Vec::from("Gurken");
        let msg_alpha = CWMessageAlpha::new(23, &text_vec).unwrap();
        assert_eq!(msg_alpha.get_codewords()[1] & 0x1FFF80,0x1D6380); // Gu
        assert_eq!(msg_alpha.get_codewords()[2] & 0x1FFFFF,0x1975F2); // rke
        assert_eq!(msg_alpha.get_codewords()[3] & 0x1FFFFF,0x00C1EE); // nETXETX
    }

    #[test]
    fn test_get_bitgroup_sum() {
        assert_eq!(CWMessageAlpha::get_bitgroup_sum(0x1FFFFF), 0x21D);
    }

    #[test]
    fn test_calculate_fragment_check_1() {
        let codewords = vec![0x1FFFFF];
        assert_eq!(CWMessageAlpha::calculate_fragment_check(&codewords), 0x1E2);
    }

    #[test]
    fn test_calculate_fragment_check_2() {
        let codewords = vec![0x1FFFFF, 0x1FFFFF];
        assert_eq!(CWMessageAlpha::calculate_fragment_check(&codewords), 0x3C5);
    }
}