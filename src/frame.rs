
use cw_fiw::FIW;
use blocks::Blocks;
use codeword::Codeword;
use message::Message;
use message::MessageType;

const PATTERN_BS1   : u32 = 0x55555555;
const PATTERN_A1    : u32 = 0x9C9ACF1E; // A1: 1600 / 2 FM
const PATTERN_B     : u16 = 0xAAAA;
const PATTERN_BS2   : u8  = 0x05;
const PATTERN_C     : u16 = 0x21B7;

const MAX_CODEWORDS_PER_BLOCK_1600 : usize = 88;

pub struct Frame {
    fiw: FIW,
    num_cws: usize,
    send_time: bool,
    msgs: Vec<Message>
}

impl Frame {
    pub fn new (cycle_number: u32,
            frame_number: u32) -> Result<Frame,&'static str>
    {
        let fiw = FIW::new(cycle_number,
                           frame_number,
                           0,
                           0x00).unwrap();

        let mut num_cws = 1; // BIW1
        let mut send_time = false;
        if frame_number == 0 {
            send_time = true;  // send BIW2, 3 and 4
            num_cws += 3;
        }

        return Ok(Frame{fiw: fiw,
                        num_cws: num_cws,
                        send_time,
                        msgs: Vec::new()});
    }

    fn get_sync1() -> Vec<u8> {
        let mut sync1 = Vec::new();
        sync1.extend_from_slice(&Frame::u32_to_4_u8(PATTERN_BS1));
        sync1.extend_from_slice(&Frame::u32_to_4_u8(PATTERN_A1));
        sync1.extend_from_slice(&Frame::u16_to_2_u8(PATTERN_B));
        sync1.extend_from_slice(&Frame::u32_to_4_u8(PATTERN_A1 ^ 0xFFFFFFFF));
        return sync1;
    }

    fn get_sync2() -> Vec<u8> {
        let mut sync2 = Vec::new();
        let mut tmp: u32 = 0x0;
        tmp |= (PATTERN_BS2 & 0xF) as u32;
        tmp |= (PATTERN_C as u32) << 4;
        tmp |= (((PATTERN_BS2 ^ 0xF) & 0xF) as u32) << 20;
        tmp |= ((PATTERN_C ^ 0xFFFF) as u32) << 24;
        sync2.extend_from_slice(&Frame::u32_to_4_u8(tmp));
        sync2.push(((PATTERN_C ^ 0xFFFF) >> 8) as u8);
        return sync2;
    }

    fn get_header(&self) -> Vec<u8> {
        let mut header = Vec::new();
        header.extend_from_slice(&Frame::get_sync1());
        header.extend_from_slice(&Frame::u32_to_4_u8(self.fiw.get_codeword()));
        header.extend_from_slice(&Frame::get_sync2());
        return header;
    }

    pub fn get_bytes(&self) -> Vec<u8> {        
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.get_header());
        bytes.extend_from_slice(&Blocks::get_bytes(&self.msgs, self.send_time));
        return bytes;
    }

    pub fn add_message(&mut self, msg: Message) -> Result<usize,&'static str> {
        let size_new_msg = msg.get_num_codewords().unwrap();
        let sum = size_new_msg + self.num_cws;

        if sum < MAX_CODEWORDS_PER_BLOCK_1600 {
            self.msgs.push(msg);
            self.num_cws = sum;
            return Ok(sum);
        }
        return Err("could not add message to frame");        
    }

    fn u32_to_4_u8(var: u32) -> [u8; 4] {
        let mut array: [u8; 4] = [0; 4];
        array[0] = (var & 0xFF) as u8;
        array[1] = (var >> 8 & 0xFF) as u8;
        array[2] = (var >> 16 & 0xFF) as u8;
        array[3] = (var >> 24 & 0xFF) as u8;
        return array;
    }

    fn u16_to_2_u8(var: u16) -> [u8; 2] {
        let mut array: [u8; 2] = [0; 2];
        array[0] = (var & 0xFF) as u8;
        array[1] = (var >> 8 & 0xFF) as u8;
        return array;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_add_message() {
        let mut frame = Frame::new(0, 1).unwrap();
        let msg = Message::new(MessageType::AlphaNum,
                               0x8001,
                               String::from("test")).unwrap();
        assert_eq!(frame.add_message(msg).unwrap(), 6);
    }

    #[test]
    fn test_frame_add_message_86() {
        let mut frame = Frame::new(0, 1).unwrap();
        for _ in 0..16 { 
            frame.add_message(Message::new(MessageType::AlphaNum,
                              0x8001,
                              String::from("test")).unwrap()).unwrap();
        }
        let msg = Message::new(MessageType::AlphaNum,
                               0x8001,
                               String::from("test")).unwrap();
        assert_eq!(frame.add_message(msg).unwrap(), 86);
    }

    #[test]
    fn test_frame_add_message_91() {
        let mut frame = Frame::new(0, 1).unwrap();
        for _ in 0..17 { 
            frame.add_message(Message::new(MessageType::AlphaNum,
                              0x8001,
                              String::from("test")).unwrap()).unwrap();
        }
        let msg = Message::new(MessageType::AlphaNum,
                               0x8001,
                               String::from("test")).unwrap();
        assert_eq!(frame.add_message(msg).is_err(), true);
    }

    #[test]
    fn test_frame_get_header() {
        let frame = Frame::new(3, 107).unwrap();
        assert_eq!(frame.get_header(), 
                   [0x55, 0x55, 0x55, 0x55, 0x1E, 0xCF, 0x9A, 0x9C, // sync1
                   0xAA, 0xAA, 0xE1, 0x30, 0x65, 0x63,
                   0x3B, 0x6B, 0xA0, 0xE4,                          // FIW
                   0x75, 0x1B, 0xA2, 0x48, 0xDE]);                  // Sync2
    }

    #[test]
    fn test_get_sync1() {
        assert_eq!(Frame::get_sync1(),
                   [0x55, 0x55, 0x55, 0x55, 0x1E, 0xCF, 0x9A, 0x9C, 
                   0xAA, 0xAA, 0xE1, 0x30, 0x65, 0x63]);
    }

    #[test]
    fn test_get_sync2() {
        assert_eq!(Frame::get_sync2(),
                   [0x75, 0x1B, 0xA2, 0x48, 0xDE]);
    }

    #[test]
    fn test_u32_to_4_u8() {
        assert_eq!(Frame::u32_to_4_u8(0x12345678),
                   [0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn test_u16_to_2_u8() {
        assert_eq!(Frame::u16_to_2_u8(0x1234),
                   [0x34, 0x12]);
    }
}
