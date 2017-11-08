
use cw_fiw::FIW;
use block::Block;
use codeword::Codeword;

const PATTERN_BS1   : u32 = 0x55555555;
const PATTERN_A1    : u32 = 0x9C9ACF1E; // A1: 1600 / 2 FM
const PATTERN_B     : u16 = 0xAAAA;
const PATTERN_BS2   : u8  = 0x05;
const PATTERN_C     : u16 = 0x21B7;

struct Frame {
    fiw: FIW
}

impl Frame {
    fn new (cycle_number: u32,
            frame_number: u32) -> Result<Frame,&'static str>
    {
        let fiw = FIW::new(cycle_number,
                           frame_number,
                           0,
                           0x0F).unwrap();
        return Ok(Frame{fiw: fiw});
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

    fn get_bytes(&self) -> Vec<u8> {
        let block = Block::new().unwrap();

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.get_header());
        bytes.extend_from_slice(&block.get_bytes());
        for i in 0..10 {
            bytes.extend_from_slice(&Block::get_empty_block());
        }
        return bytes;
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
    fn test_frame_get_header() {
        let frame = Frame::new(0, 0).unwrap();
        assert_eq!(frame.get_header(), 
                   [0x55, 0x55, 0x55, 0x55, 0x1E, 0xCF, 0x9A, 0x9C, // sync1
                   0xAA, 0xAA, 0xE1, 0x30, 0x65, 0x63,
                   0x00, 0x00, 0x1E, 0x42,                          // FIW
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
