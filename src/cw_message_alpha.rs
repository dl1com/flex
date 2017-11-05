use codeword::Codeword;
use cw_message_alpha_header::CWMessageAlphaHeader;
use cw_message_alpha_signature::CWMessageAlphaSignature;
use cw_message_alpha_content::CWMessageAlphaContent;

struct CWMessageAlpha {
    header: CWMessageAlphaHeader,
    signature: CWMessageAlphaSignature,
    content: Vec<CWMessageAlphaContent>
}

impl CWMessageAlpha {
    fn new (message_number: u32,
            chars: &[u8]) -> Result<CWMessageAlpha, &'static str>
    {
        // Not supported for now:
        // - Fragmented messages
        // - Message retrieval flag
        // - Mail Drop flag

        if chars.len() > 251 {
           return Err("Fragmented messages not implemented.");
        }

        // TODO Insert chars to fill up to 2 or multiples of 3(+2)
        let signature = CWMessageAlphaSignature::new(CWMessageAlpha::calculate_signature(chars),
                                                        &chars[0..2]).unwrap();
        
        let mut content = Vec::new();
        for i in 0..((chars.len()-2) / 3) {        
           content.push(CWMessageAlphaContent::new(&chars[2+i*3..5+i*3]).unwrap());
        }

        let header = CWMessageAlphaHeader::new(CWMessageAlpha::calculate_fragment_check(),
                                            0,
                                            3,
                                            message_number,
                                            0,
                                            0).unwrap(); 
        
        return Ok(CWMessageAlpha{
            header: header,
            signature: signature,
            content: content
            });
    }

    fn calculate_signature(chars: &[u8]) -> u32 {
        let mut sum: u32 = 0;
        for chr in chars.to_vec() {
            sum += chr as u32 & 0x7F;
        }
        sum ^= 0xFFFFFFFF;
        return sum & 0x7F;
    }

    fn calculate_fragment_check() -> u32 {
        return 0x0;
    }

    fn get_codewords(&self) -> Vec<u32> {
        let mut cws = Vec::new();
        cws.push(self.header.get_codeword());
        cws.push(self.signature.get_codeword());
        for content in &self.content {
            cws.push(content.get_codeword());
        }
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
    fn test_message_alpha_get_codewords() {
        let text_vec = Vec::from("Gurkensalat");
        let msg_alpha = CWMessageAlpha::new(23, &text_vec).unwrap();
        println!("{:?}",msg_alpha.get_codewords());
        println!("0x{:X}",msg_alpha.get_codewords()[0] & 0x1FFFFF);
        assert_eq!(msg_alpha.get_codewords()[0] & 0x1FFA00,0x02F800);
        assert_eq!(msg_alpha.get_codewords()[1] & 0x1FFF80,0x1D6380); // Gu
        assert_eq!(msg_alpha.get_codewords()[2] & 0x1FFFFF,0x1975F2); // rke
        assert_eq!(msg_alpha.get_codewords()[3] & 0x1FFFFF,0x1879EE); // nsa
        assert_eq!(msg_alpha.get_codewords()[4] & 0x1FFFFF,0x1D30EC); // lat
    }
}