pub fn interleave_codewords_1600(input: [u32; 8]) -> [u8; 32]
{
    let mut output_data: [u8; 32] = [0; 32];
    for bit_index in 0..32 {
        for codeword in 0..8 {
            let input_mask = 1 << bit_index;
            let masked_input = input[codeword]  & input_mask;
            let backshifted_input = (masked_input >> bit_index) & 0x00000001;
            let read_bit:u8 = backshifted_input as u8;
            let bit_to_write:u8 = read_bit << codeword;
            output_data[bit_index] = output_data[bit_index] ^ bit_to_write;
        }
    }
    
    return output_data;
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_interleave_codewords_1600_all_zeroes() {

        let test_data: [u32; 8] = [0; 8];
        let result = interleave_codewords_1600(test_data);

        assert_eq!(result, [0; 32]);
    }

    #[test]
    fn test_interleave_codewords_1600_all_ones() {

        let test_data: [u32; 8] = [<u32>::max_value(); 8];
        let result = interleave_codewords_1600(test_data);

        assert_eq!(result, [<u8>::max_value(); 32]);
    }

    #[test]
    fn test_interleave_codewords_1600_single_one() {

        let test_data: [u32; 8] = [0x00000200, 0,0,0,0,0,0,0];
        let result = interleave_codewords_1600(test_data);
        let mut test_helper: [u8; 32] = [0; 32];
        test_helper[9] = 0x01;
        assert_eq!(result, test_helper);
    }

    #[test]
    fn test_interleave_codewords_1600_all_as() {

        let test_data: [u32; 8] = [0xaaaaaaaa; 8];
        let result = interleave_codewords_1600(test_data);

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