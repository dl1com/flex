pub fn apply_4bit_checksum(codeword: u32) -> u32
{
    let mut sum: u32 = 0x0;
    sum += (codeword>>4) & 0xF;
    sum += (codeword>>8) & 0xF;
    sum += (codeword>>12) & 0xF;
    sum += (codeword>>16) & 0xF;
    sum += (codeword>>20) & 0x1;
    sum ^= 0xF;
    sum &= 0xF;
    return codeword | sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_4bit_checksum_all_zeroes() {

        let test_codeword: u32 = 0x00000000;
        let expected_crc: u32 = 0x0000000F;
        let result = apply_4bit_checksum(test_codeword);

        assert_eq!(result, expected_crc);
    }

    #[test]
    fn test_calculate_4bit_checksum_all_ones() {

        let test_codeword: u32 = 0x001FFFF0;
        let expected_crc: u32 = 0x001FFFF2;
        let result = apply_4bit_checksum(test_codeword);

        assert_eq!(result, expected_crc);
    }

    #[test]
    fn test_calculate_4bit_checksum() {

        let test_codeword: u32 = 0x00139C50;
        let expected_crc: u32 = 0x00139C51;
        let result = apply_4bit_checksum(test_codeword);

        assert_eq!(result, expected_crc);
    }
}