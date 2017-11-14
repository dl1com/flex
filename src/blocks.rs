
use codewords::codeword::Codeword;
use codewords::biw1::BIW1;
use codewords::biw2::BIW2;
use codewords::biw3::BIW3;
use codewords::biw4::BIW4;
use codewords::address_short::CWAddressShort;
use codewords::vector_alpha::CWVectorAlpha;
use codewords::message_alpha::CWMessageAlpha;

use message::Message;

pub struct Blocks {
}

impl Blocks {
    pub fn get_bytes(msgs: &Vec<Message>, send_time: bool) -> Vec<u8> {

        let amount_address_cws = Blocks::count_address_cws(msgs);

        let biw_cws = Blocks::get_biw_cws(amount_address_cws, send_time);
        let addr_cws = Blocks::get_addr_cws(msgs);
        let (vector_cws, content_cws) = Blocks::get_vector_and_content_cws(msgs,
                                                                           biw_cws.len(),
                                                                           addr_cws.len());

        let mut cws = Vec::new();
        cws.extend_from_slice(&biw_cws);
        cws.extend_from_slice(&addr_cws);
        cws.extend_from_slice(&vector_cws);
        cws.extend_from_slice(&content_cws);

        cws = Blocks::fill_up_block_1600(&cws);

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

    fn get_biw_cws(amount_addresses: usize,
                   send_time: bool) -> Vec<u32> {
        let mut startword_addresses = 0;
        if send_time {
            startword_addresses = 3; // offset because of BIW 2,3 and 4
        }

        let startword_vectors = 1   // BIW 1
                                + startword_addresses
                                + amount_addresses;

        let biw1 = BIW1::new(0,
                             startword_addresses as u32,
                             startword_vectors as u32,
                             0, // No carry-on for now
                             0).unwrap(); // 0=pager decodes all frames
        
        let mut biw_cws = Vec::new();
        biw_cws.push(biw1.get_codeword());

        if send_time {
            let biw2 = BIW2::new(0, 0x1).unwrap();
            let biw3 = BIW3::new(23, 05, 1999).unwrap();
            let biw4 = BIW4::new(13, 37, 00).unwrap();
            biw_cws.push(biw2.get_codeword());
            biw_cws.push(biw3.get_codeword());
            biw_cws.push(biw4.get_codeword());
        }
        return biw_cws;
    }

    fn get_addr_cws(msgs: &Vec<Message>) -> Vec<u32> {
        let mut addr_cws = Vec::new();
        for msg in msgs {
            let address = CWAddressShort::new(msg.address).unwrap();
            addr_cws.push(address.get_codeword());
        }
        return addr_cws;
    }

    fn get_vector_and_content_cws(msgs: &Vec<Message>,
                                  biw_cws_size: usize,
                                  addr_cws_size: usize) -> (Vec<u32>,Vec<u32>) {
        let biw_addr_vector_cws_size = biw_cws_size
                                       + 2*addr_cws_size; // Address and Vector CWS

        let mut vector_cws: Vec<u32> = Vec::new();
        let mut content_cws: Vec<u32> = Vec::new();
        for msg in msgs {
            let content_start = biw_addr_vector_cws_size
                                + content_cws.len();
            let content_words = msg.get_content_cw_size() as u32;

            let vector = CWVectorAlpha::new(content_start as u32,
                                            content_words).unwrap();
            vector_cws.push(vector.get_codeword());

            let content = CWMessageAlpha::new(0,
                                              msg.data.as_bytes()).unwrap();
            content_cws.extend_from_slice(&content.get_codewords());
        }

        return (vector_cws,content_cws);
    }

    fn fill_up_block_1600(cws: &Vec<u32>) -> Vec<u32> {
        let mut filled_cws = Vec::new();
        filled_cws.extend_from_slice(&cws);
        while filled_cws.len() < 88 {
            if filled_cws.len() % 2 == 0 {
                filled_cws.push(0xFFFFFFFF);
            }
            else {
                filled_cws.push(0x0);
            }
        }
        return filled_cws;
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
    use message::MessageType;

    #[test]
    fn test_get_biw_cws_no_time() {
        let cws = Blocks::get_biw_cws(1, false);
        assert_eq!(cws.len(), 1);
        assert_eq!(cws[0] & 0x1FFFF0, 0x000800);
    }

    #[test]
    fn test_get_biw_cws_send_time() {
        let cws = Blocks::get_biw_cws(1, true);
        assert_eq!(cws.len(), 4);
        assert_eq!(cws[0] & 0x1FFFF0, 0x001700);
    }

    #[test]
    fn test_get_addr_cws() {
        let msg = Message::new(MessageType::AlphaNum,
                               0x8001,
                               String::from("test")).unwrap();
        let msgs = vec![msg];
        let cws = Blocks::get_addr_cws(&msgs);
        assert_eq!(cws.len(), 1);
        assert_eq!(cws[0] & 0x1FFFFF, 0x8001);
    }

    #[test]
    fn test_get_vector_and_content_cws() {
        let msg = Message::new(MessageType::AlphaNum,
                               0x8001,
                               String::from("test")).unwrap();
        let msgs = vec![msg];
        let (vector_cws,content_cws) = Blocks::get_vector_and_content_cws(&msgs, 1, msgs.len());
        assert_eq!(vector_cws.len(), 1);
        assert_eq!(vector_cws[0] & 0x1FFFF0, 0x00C1D0);
        assert_eq!(content_cws.len(), 3); 
    }

    #[test]
    fn test_fill_up_block_1600() {
        let cws = vec![0x0];
        let filled_up = Blocks::fill_up_block_1600(&cws);
        assert_eq!(filled_up.len(), 88);
    }

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