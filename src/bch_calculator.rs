pub fn calculate_crc(codeword: u32) -> u32 
{
    let mut cw = codeword;

    for i in 0..21 {
        if ((cw & 0x80000000) > 0) {
            cw ^= 0xED200000;
        }
        cw <<= 1;
    }
    return cw;
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
}