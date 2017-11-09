
use codeword::Codeword;
use cw_biw1::BIW1;
use cw_biw3::BIW3;
use cw_biw4::BIW4;
use cw_address_short::CWAddressShort;
use cw_vector_alpha::CWVectorAlpha;
use cw_message_alpha::CWMessageAlpha;

pub struct Block {
}

impl Block {
    pub fn new() -> Result<Block, &'static str> {
        return Ok(Block{});
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        // Test block layout
        // 0    BIW 1
        // 1    BIW3
        // 2    BIW4
        // 3    Address Short
        // 4    ALN Vector
        // 5    ALN Msg Header
        // 6    ALN Msg Sig
        // 7    ALN Msg

        // Priority addresses not supported by now
        // Carry-on not supported by now
        let biw1 = BIW1::new(0,
                             2, // Addresses start from word 3
                             4, // Vectors start from field 4
                             0,
                             0).unwrap(); // 0=pager decodes all frames
        let address = CWAddressShort::new(0x8204).unwrap();
        let biw3 = BIW3::new(23, 05, 1999).unwrap();
        let biw4 = BIW4::new(13, 37, 0).unwrap();
        let vector_alpha = CWVectorAlpha::new(5, 3).unwrap();
        let msg_alpha = CWMessageAlpha::new(1, String::from("gurke").as_bytes()).unwrap();

        let mut block_cws = Vec::new();
        block_cws.push(biw1.get_codeword());
        block_cws.push(biw3.get_codeword());
        block_cws.push(biw4.get_codeword());
        block_cws.push(address.get_codeword());
        block_cws.push(vector_alpha.get_codeword());
        for codeword in msg_alpha.get_codewords() {
            block_cws.push(codeword);
        }

        return Block::interleave_codewords_1600(&block_cws).to_vec();
    }

    pub fn get_empty_block() -> Vec<u8> {
        return [0x55; 32].to_vec();
    }

    fn interleave_codewords_1600(input: &[u32]) -> [u8; 32]
    {
        if input.len() != 8 {panic!("Exactly 8 input codewords required");}

        let mut output_data: [u8; 32] = [0; 32];
        for bit_index in 0..32 {
            for codeword in 0..8 {
                let input_mask = 1 << bit_index;
                let masked_input = &input[codeword]  & input_mask;
                let backshifted_input = (masked_input >> bit_index) & 0x00000001;
                let read_bit:u8 = backshifted_input as u8;
                let bit_to_write:u8 = read_bit << codeword;
                output_data[bit_index] = output_data[bit_index] ^ bit_to_write;
            }
        }        
        return output_data;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block() {
        assert_eq!(Block::new().is_err(), false);
    }

    #[test]
    fn test_get_empty_block() {
        assert_eq!(Block::get_empty_block(), [0x55; 32]);
    }

    #[test]
    fn test_interleave_codewords_1600_all_zeroes() {

        let test_data: [u32; 8] = [0; 8];
        let result = Block::interleave_codewords_1600(&test_data);

        assert_eq!(result, [0; 32]);
    }

    #[test]
    fn test_interleave_codewords_1600_all_ones() {

        let test_data: [u32; 8] = [<u32>::max_value(); 8];
        let result = Block::interleave_codewords_1600(&test_data);

        assert_eq!(result, [<u8>::max_value(); 32]);
    }

    #[test]
    fn test_interleave_codewords_1600_single_one() {

        let test_data: [u32; 8] = [0x00000200, 0,0,0,0,0,0,0];
        let result = Block::interleave_codewords_1600(&test_data);
        let mut test_helper: [u8; 32] = [0; 32];
        test_helper[9] = 0x01;
        assert_eq!(result, test_helper);
    }

    #[test]
    fn test_interleave_codewords_1600_all_as() {

        let test_data: [u32; 8] = [0xaaaaaaaa; 8];
        let result = Block::interleave_codewords_1600(&test_data);

        assert_eq!(result, [0x00, 0xff, 0x00, 0xff,
                            0x00, 0xff, 0x00, 0xff,
                            0x00, 0xff, 0x00, 0xff,
                            0x00, 0xff, 0x00, 0xff,
                            0x00, 0xff, 0x00, 0xff,
                            0x00, 0xff, 0x00, 0xff,
                            0x00, 0xff, 0x00, 0xff,
                            0x00, 0xff, 0x00, 0xff]);
    }
}