use codeword::Codeword;
use apply_bch_and_parity::apply_bch_and_parity;

pub struct CWMessageAlphaSignature {
    signature: u32,
    chars: Vec<u8>
}

impl CWMessageAlphaSignature {
    pub fn new (signature: u32,
                chars: &[u8]) -> Result<CWMessageAlphaSignature, &'static str>
    {
        if chars.len() != 2 {
            return Err("Alphanumeric Message Signature: 2 chars required.");
        }

        if signature <= 0x7F {
            return Ok(CWMessageAlphaSignature{
                signature: signature,
                chars: chars.to_vec()
            });
        }
        else {
            return Err("Alphanumeric Message Signature: Invalid Parameter.");
        }
    }
}

impl Codeword for CWMessageAlphaSignature {
    fn get_codeword(&self) -> u32 {
        let mut cw: u32 = 0x0;
        cw |= self.signature;
        cw |= ((self.chars[0] & 0x7F) as u32) << 7;
        cw |= ((self.chars[1] & 0x7F) as u32) << 14;
        cw = apply_bch_and_parity(cw);
        return cw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_alpha_signature()   {
        let chars = vec![0x42, 0x23];
        let msg_signature = CWMessageAlphaSignature::new(0x7F,
                                                         &chars).unwrap();
        assert_eq!(msg_signature.get_codeword() & 0x1FFFFF, 0x8E17F);
    }

    #[test]
    fn test_message_alpha_signature_invalid() {
        let chars = vec![0x42, 0x23];
        assert_eq!(CWMessageAlphaSignature::new(0x800,
                                                &chars).is_err(), true);
    }

    #[test]
    fn test_message_alpha_signature_too_few_chars() {
        let chars = vec![0x42];
        assert_eq!(CWMessageAlphaSignature::new(0x7FF,
                                                &chars).is_err(), true);
    }

    #[test]
    fn test_message_alpha_signature_too_many_chars() {
        let chars = vec![0x42, 0x23, 0x05];
        assert_eq!(CWMessageAlphaSignature::new(0x7FF,
                                                &chars).is_err(), true);
    }
}