
use codeword::Codeword;
use cw_biw1::BIW1;
use cw_biw2::BIW2;
use cw_biw3::BIW3;
use cw_biw4::BIW4;
use cw_address_short::CWAddressShort;
use cw_vector_alpha::CWVectorAlpha;
use cw_message_alpha::CWMessageAlpha;

use message::Message;

pub struct Blocks {
}

impl Blocks {
    pub fn get_bytes(msgs: &Vec<Message>, send_time: bool) -> Vec<u8> {

        let mut biw234_offset = 0;
        if send_time {
            biw234_offset = 3;
        }

        let mut addr_cws = Vec::new();
        for msg in msgs {
            let address = CWAddressShort::new(msg.address).unwrap();
            addr_cws.push(address.get_codeword());
        }

        let mut vector_cws: Vec<u32> = Vec::new();
        let mut content_cws: Vec<u32> = Vec::new();
        for msg in msgs {
            let content_start = 1               // BIW 1
                                + biw234_offset // BIW234
                                + msgs.len()    // Addresses
                                + msgs.len()    // Vectors
                                + content_cws.len();
            let content_words = msg.get_content_cw_size() as u32;

            let vector = CWVectorAlpha::new(content_start as u32,
                                            content_words).unwrap();
            vector_cws.push(vector.get_codeword());

            let content = CWMessageAlpha::new(0,
                                              msg.data.as_bytes()).unwrap();
            content_cws.extend_from_slice(&content.get_codewords());
        }

        let startword_addresses = biw234_offset;
        let startword_vectors = 1                 // BIW 1
                                + biw234_offset   // BIW 2, 3, 4
                                + Blocks::count_address_cws(msgs);

        let biw1 = BIW1::new(0,
                             startword_addresses as u32,
                             startword_vectors as u32,
                             0, // No carry-on for now
                             0).unwrap(); // 0=pager decodes all frames
        
        let mut cws = Vec::new();
        cws.push(biw1.get_codeword());

        if send_time {
            let biw2 = BIW2::new(0, 0x1).unwrap();
            let biw3 = BIW3::new(23, 05, 1999).unwrap();
            let biw4 = BIW4::new(13, 37, 00).unwrap();
            cws.push(biw2.get_codeword());
            cws.push(biw3.get_codeword());
            cws.push(biw4.get_codeword());
        }

        cws.append(&mut addr_cws);
        cws.append(&mut vector_cws);
        cws.append(&mut content_cws);

        while cws.len() < 88 {
            if cws.len() % 2 == 0 {
                cws.push(0xFFFFFFFF);
            }
            else {
                cws.push(0x0);
            }
        }

        let mut bytes = Vec::new();
        for i in 0..11 {
            bytes.extend_from_slice(&Blocks::interleave_codewords_1600(&cws[i*8..(i+1)*8]));
        }

        return bytes.to_vec();
    }

    fn count_address_cws(msgs: &Vec<Message>) -> usize {
        // Currently, only Short Addresses supported, so 
        // amount of address codwords is equal to amount of messages
        return msgs.len();
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
    fn test_interleave_codewords_1600_all_zeroes() {

        let test_data: [u32; 8] = [0; 8];
        let result = Blocks::interleave_codewords_1600(&test_data);

        assert_eq!(result, [0; 32]);
    }

    #[test]
    fn test_interleave_codewords_1600_all_ones() {

        let test_data: [u32; 8] = [<u32>::max_value(); 8];
        let result = Blocks::interleave_codewords_1600(&test_data);

        assert_eq!(result, [<u8>::max_value(); 32]);
    }

    #[test]
    fn test_interleave_codewords_1600_single_one() {

        let test_data: [u32; 8] = [0x00000200, 0,0,0,0,0,0,0];
        let result = Blocks::interleave_codewords_1600(&test_data);
        let mut test_helper: [u8; 32] = [0; 32];
        test_helper[9] = 0x01;
        assert_eq!(result, test_helper);
    }

    #[test]
    fn test_interleave_codewords_1600_all_as() {

        let test_data: [u32; 8] = [0xaaaaaaaa; 8];
        let result = Blocks::interleave_codewords_1600(&test_data);

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