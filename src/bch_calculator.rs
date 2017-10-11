pub fn calculate_crc(codeword: u32) -> u32 
{
    let mut crc = codeword;
    for i in 0..21 {
        if (crc & (0x00000001 << i)) != 0 {
            crc ^= 0x4B7 << i;
        }
    }
    return codeword | crc;    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_crc_all_zeroes() {

        let test_codeword: u32 = 0;
        let expected_crc = 0;
        let result = calculate_crc(test_codeword);

        assert_eq!(result, expected_crc);
    }

    #[test]
    fn test_calculate_crc_all_ones() {

        let test_codeword: u32 = 0x001FFFFF;
        let expected_crc = 0x7FFFFFFF;
        let result = calculate_crc(test_codeword);

        assert_eq!(result, expected_crc);
    }

    #[test]
    fn test_calculate_crc_1() {

        let test_codeword: u32 = 0x1D40CD;
        let expected_crc: u32 = 0x1EDD40CD;
        let result = calculate_crc(test_codeword);

        assert_eq!(result, expected_crc);
    }

    #[test]
    fn test_calculate_crc_2() {

        let test_codeword: u32 = 0x87523;
        let expected_crc: u32 = 0x38C87523;
        let result = calculate_crc(test_codeword);

        assert_eq!(result, expected_crc);
    }
}