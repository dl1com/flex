
pub enum MessageType {
    AlphaNum
}

#[derive(Debug, PartialEq)]
pub enum AddressType {
    Short,
    Invalid
}

pub struct Message {
    msgtype: MessageType,
    pub address: u32,
    pub data: String
}

impl Message {
    pub fn new(msgtype: MessageType,
               address: u32,
               data: String) -> Result<Message, &'static str> {

        if Message::get_address_type(address) == AddressType::Invalid {
            return Err("Invalid address");
        }
        return Ok(Message{msgtype: msgtype,
                          address: address,
                          data: data
        });
    }

    pub fn get_num_codewords(&self) -> Result<usize, &'static str> {
        let mut size = 0;
        match Message::get_address_type(self.address) {
            AddressType::Short => size += 2,  // Address Word + Vector Word
            AddressType::Invalid => return Err("Invalid address given")
        }

        match self.msgtype {
            MessageType::AlphaNum => size += self.get_content_cw_size()
        }

        return Ok(size);
    }

    pub fn get_content_cw_size(&self) -> usize {
        let mut size: usize = 0;
        size += 2;                      // Message Header and Signature
        size += (self.data.len()-2) / 3;   // 3 chars per Content codeword
        if (self.data.len()-2) % 3 > 0 {
            size += 1;
        }
        return size;
    }

    fn get_address_type(address: u32) -> AddressType {
        if address >= 0x8001 && address <= 0x1F27FF {
            return AddressType::Short;
        }
        return AddressType::Invalid;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message() {
        let msg = Message::new(MessageType::AlphaNum,
                               0x8001,
                               String::from("test"));
        assert_eq!(msg.is_err(), false);
    }

    #[test]
    fn test_message_invalid_address() {
        let msg = Message::new(MessageType::AlphaNum,
                               0x1,
                               String::from("test"));
        assert_eq!(msg.is_err(), true);
    }

    #[test]
    fn test_message_address_type() {        
        assert_eq!(Message::get_address_type(0x8001), AddressType::Short);
    }

    #[test]
    fn test_get_content_cw_size() {
        let msg = Message::new(MessageType::AlphaNum,
                        0x8001,
                        String::from("abcde")).unwrap();
        assert_eq!(msg.get_content_cw_size(), 3);
    }

    #[test]
    fn test_message_get_size_2() {
        let msg = Message::new(MessageType::AlphaNum,
                               0x8001,
                               String::from("ab")).unwrap();
        assert_eq!(msg.get_num_codewords().unwrap(), 4);
    }

    #[test]
    fn test_message_get_size_4() {
        let msg = Message::new(MessageType::AlphaNum,
                               0x8001,
                               String::from("abcd")).unwrap();
        assert_eq!(msg.get_num_codewords().unwrap(), 5);
    }
}