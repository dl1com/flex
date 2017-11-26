
#[derive(Debug, Clone, Deserialize)]
pub enum MessageType {
    AlphaNum,
}

#[derive(Debug, PartialEq)]
pub enum CapcodeType {
    ShortAddress,
    Invalid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Message {
    pub frame: u32,
    msgtype: MessageType,
    pub capcode: u32,
    pub data: String,
}

impl Message {
    pub fn new(
        frame: u32,
        msgtype: MessageType,
        capcode: u32,
        data: String,
    ) -> Result<Message, &'static str> {

        if Message::get_capcode_type(capcode) == CapcodeType::Invalid {
            return Err("Invalid CAPCODE");
        }

        if frame > 128 {
            return Err("Invalid Frame number");
        }

        return Ok(Message {
            frame: frame,
            msgtype: msgtype,
            capcode: capcode,
            data: data,
        });
    }

    pub fn get_num_of_message_codewords(&self) -> Result<usize, &'static str> {
        let mut size = 0;
        match Message::get_capcode_type(self.capcode) {
            CapcodeType::ShortAddress => size += 2,  // Address Word + Vector Word
            CapcodeType::Invalid => return Err("Invalid CAPCODE given"),
        }

        match self.msgtype {
            MessageType::AlphaNum => size += self.get_num_of_content_codewords(),
        }

        return Ok(size);
    }

    pub fn get_num_of_content_codewords(&self) -> usize {
        let mut size: usize = 0;
        size += 2; // Message Header and Signature
        size += (self.data.len() - 2) / 3; // 3 chars per Content codeword
        if (self.data.len() - 2) % 3 > 0 {
            size += 1;
        }
        return size;
    }

    fn get_capcode_type(capcode: u32) -> CapcodeType {
        if capcode >= 0x0001 && capcode <= 0x1EA7FF {
            return CapcodeType::ShortAddress;
        }
        return CapcodeType::Invalid;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_when_typical() {
        let msg = Message::new(0, MessageType::AlphaNum, 0x0001, String::from("test"));
        assert_eq!(msg.is_err(), false);
    }

    #[test]
    fn test_message_when_invalid_capcode() {
        let msg = Message::new(0, MessageType::AlphaNum, 0x0000, String::from("test"));
        assert_eq!(msg.is_err(), true);
    }

    #[test]
    fn test_message_get_capcode_type_when_short_address() {
        assert_eq!(Message::get_capcode_type(0x8001), CapcodeType::ShortAddress);
    }

    #[test]
    fn test_message_get_capcode_type_when_zero_address() {
        assert_eq!(Message::get_capcode_type(0x0), CapcodeType::Invalid);
    }

    #[test]
    fn test_message_get_capcode_type_when_invalid_address() {
        assert_eq!(Message::get_capcode_type(0x1EB000), CapcodeType::Invalid);
    }

    #[test]
    fn test_message_get_num_of_content_codewords_when_five_character() {
        let msg = Message::new(0, MessageType::AlphaNum, 0x8001, String::from("abcde")).unwrap();
        assert_eq!(msg.get_num_of_content_codewords(), 3);
    }

    #[test]
    fn test_message_get_num_of_message_codewords_when_two_character() {
        let msg = Message::new(0, MessageType::AlphaNum, 0x8001, String::from("ab")).unwrap();
        assert_eq!(msg.get_num_of_message_codewords().unwrap(), 4);
    }

    #[test]
    fn test_message_get_num_of_message_codewords_when_four_character() {
        let msg = Message::new(0, MessageType::AlphaNum, 0x8001, String::from("abcd")).unwrap();
        assert_eq!(msg.get_num_of_message_codewords().unwrap(), 5);
    }
}
