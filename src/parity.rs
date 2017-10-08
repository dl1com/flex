
// Think of using crate hamming to use hamming weight for this
pub fn check_and_set_parity(codeword: &mut u32) -> bool {

    let ones = count_ones(*codeword);

    let odd = (ones % 2) != 0;
    if odd {
        set_parity(codeword);
    }
    else {
        clear_parity(codeword);
    }

    return odd;
}

fn count_ones(codeword: u32) -> u32{
    let mut ones = 0;
    for i in 0..31 {
        let mask = 1 << i;
        ones += (codeword & mask) >> i;
    }
    return ones;
}

fn set_parity(codeword: &mut u32) {
    let parity_bit = 0x80000000;
    *codeword = *codeword ^ parity_bit;
}

fn clear_parity(codeword: &mut u32) {
    *codeword = *codeword & 0x7FFFFFFF;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parity_all_zeroes() {

        let mut test_data: u32 = 0x0;
        let result: bool = check_and_set_parity(&mut test_data);

        assert_eq!(result, false);
        assert_eq!(test_data, test_data);
    }

    #[test]
    fn test_parity_all_ones() {

        let mut test_data: u32 = 0x7FFFFFFF;
        let result: bool = check_and_set_parity(&mut test_data);

        assert_eq!(result, true);
        assert_eq!(test_data, 0xFFFFFFFF);
    }

    #[test]
    fn test_parity_even() {

        let mut test_data: u32 = 0x00000003;
        let result: bool = check_and_set_parity(&mut test_data);

        assert_eq!(result, false);
        assert_eq!(test_data, test_data);
    }

    #[test]
    fn test_parity_illegal_paritybit() {

        let mut test_data: u32 = 0x80000003;
        let result: bool = check_and_set_parity(&mut test_data);

        assert_eq!(result, false);
        assert_eq!(test_data, 0x00000003);
    }
}