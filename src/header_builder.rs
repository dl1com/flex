use bitstream_io::{BE, BitWriter};

// Sync1
const FLEX_BS1: u32 = 0xAAAAAAAA;
const FLEX_A1: u32 = 0x9C9ACF1E; // 1600 / 2 FM
//const FLEX_A2: u32 = 0x9C9AE721; // 3200 / 2 FM
//const FLEX_A3: u32 = 0x9C9AE9F2; // 3200 / 4 FM
//const FLEX_A4: u32 = 0x9C9AFA84; // 6400 / 4 FM
const FLEX_B: u16 = 0x5555;
// Sync2
const FLEX_BS2: u8 = 0x05;
const FLEX_C: u16 = 0x7B12;

// TODO Write Builders for Header Structs
// https://crates.io/crates/derive_builder
// https://docs.rs/derive_builder/0.5.0/derive_builder/
// -> Do sanity checks for fields in setters

struct FlexHeaderSync1 {
    bs1: u32,   // 32 bit bitsync
    a: u32,     // 32 bit speed indication
    b: u16,     // 16 bit bitsync
}

struct FlexHeaderSync2 {
    bs2: u8,    // 4 bit bitsync
    c: u16,     // 16 bit speed indication
}

 
fn get_header_sync1_vector(header: FlexHeaderSync1) -> Vec<u8>{
    let mut vector: Vec<u8> = Vec::new();
    {
        let mut writer = BitWriter::<BE>::new(&mut vector);
        writer.write(32, header.bs1).unwrap();
        writer.write(32, header.a).unwrap();
        writer.write(16, header.b).unwrap();
        writer.write(32, !header.a).unwrap();
        writer.into_unwritten();
    }
    return vector;
}

fn get_header_sync2_vector(header: FlexHeaderSync2) -> Vec<u8>{
    let mut vector: Vec<u8> = Vec::new();
    {
        let mut writer = BitWriter::<BE>::new(&mut vector);
        writer.write(4, header.bs2).unwrap();
        writer.write(16, header.c).unwrap();
        writer.write(4, 0xf & !header.bs2).unwrap(); // Strange things happen when writing an inverted nibble without masking
        writer.write(16, !header.c).unwrap();
        writer.into_unwritten();
    }
    return vector;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_header_sync1_vector() {
        let flex_header_sync1 = FlexHeaderSync1 {
            bs1: FLEX_BS1,
            a: FLEX_A1,
            b: FLEX_B
        };

        let result = get_header_sync1_vector(flex_header_sync1);

        assert_eq!(result, vec![0xAA,0xAA,0xAA,0xAA,
                                        0x9C,0x9A,0xCF,0x1E,
                                        0x55,0x55,
                                        0x63,0x65,0x30,0xE1]);
    }

    #[test]
    fn test_get_header_sync2_vector() {
        let flex_header_sync2 = FlexHeaderSync2 {
            bs2: FLEX_BS2, c: FLEX_C
        };
        let result = get_header_sync2_vector(flex_header_sync2);
        println!("{:?}", result);
        assert_eq!(result, vec![0x57,0xB1,0x2A,
                                0x84,0xED]);
    }
}
