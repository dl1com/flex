use codeword::Codeword;
use apply_bch_and_parity::apply_bch_and_parity;

struct CWMessageAlphaSignature {
    signature: u32,
    chars: [u8; 2]
}

impl CWMessageAlphaSignature {
    fn new (signature: u32,
            chars: [u8; 2]) -> Result<CWMessageAlphaSignature, &'static str>
    {
        if signature <= 0x7F {
            Ok(CWMessageAlphaSignature{
                signature: signature,
                chars: chars 
            })
        }
        else {
            Err("Alphanumeric Message Signature: Invalid Parameter.")
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
    fn test_message_alpha_signature() {
        let msg_signature = CWMessageAlphaSignature::new(0x7F,
                                                         [0x42, 0x23]).unwrap();
        assert_eq!(msg_signature.get_codeword() & 0x1FFFFF, 0x8E17F);
    }

    #[test]
    fn test_message_alpha_signature_invalid() {
        assert_eq!(CWMessageAlphaSignature::new(0x800,
                                                [0x42, 0x23]).is_err(), true);
    }
}