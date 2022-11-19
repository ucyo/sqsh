#![allow(dead_code)]

const RLE_U8_B1: [u8; 2] = [0b00000000, 0b00000001];
const RLE_U8_B2: [u8; 4] = [0b00000000, 0b00000011, 0b00000001, 0b00000010];
const RLE_U8_B3: [u8; 8] = [
    0b00000000, 0b00000111, 0b00000001, 0b00000011, 0b00000110, 0b00000100, 0b00000010, 0b00000101,
];
const RLE_U8_B4: [u8; 16] = [
    0b00000000, 0b00001111, 0b00000001, 0b00000011, 0b00000111, 0b00001110, 0b00001100, 0b00001000,
    0b00000010, 0b00000110, 0b00000100, 0b00001101, 0b00001001, 0b00001011, 0b00000101, 0b00001010,
];
const RLE_U8_B5: [u8; 32] = [
    0b00000000, 0b00011111, 0b00000001, 0b00000011, 0b00000111, 0b00001111, 0b00011110, 0b00011100,
    0b00011000, 0b00010000, 0b00000010, 0b00000110, 0b00000100, 0b00001110, 0b00001100, 0b00001000,
    0b00011101, 0b00011001, 0b00011011, 0b00010001, 0b00010011, 0b00010111, 0b00000101, 0b00001101,
    0b00001001, 0b00001011, 0b00011010, 0b00010010, 0b00010110, 0b00010100, 0b00001010, 0b00010101,
];
const RLE_U8_B6: [u8; 64] = [
    0b00000000, 0b00111111, 0b00000001, 0b00000011, 0b00000111, 0b00001111, 0b00011111, 0b00111110,
    0b00111100, 0b00111000, 0b00110000, 0b00100000, 0b00000010, 0b00000110, 0b00000100, 0b00001110,
    0b00001100, 0b00001000, 0b00011110, 0b00011100, 0b00011000, 0b00010000, 0b00111101, 0b00111001,
    0b00111011, 0b00110001, 0b00110011, 0b00110111, 0b00100001, 0b00100011, 0b00100111, 0b00101111,
    0b00000101, 0b00001101, 0b00001001, 0b00001011, 0b00011101, 0b00011001, 0b00011011, 0b00010001,
    0b00010011, 0b00010111, 0b00111010, 0b00110010, 0b00110110, 0b00110100, 0b00100010, 0b00100110,
    0b00100100, 0b00101110, 0b00101100, 0b00101000, 0b00001010, 0b00011010, 0b00010010, 0b00010110,
    0b00010100, 0b00110101, 0b00100101, 0b00101101, 0b00101001, 0b00101011, 0b00010101, 0b00101010,
];
const RLE_U8_B7: [u8; 128] = [
    0b00000000, 0b01111111, 0b00000001, 0b00000011, 0b00000111, 0b00001111, 0b00011111, 0b00111111,
    0b01111110, 0b01111100, 0b01111000, 0b01110000, 0b01100000, 0b01000000, 0b00000010, 0b00000110,
    0b00000100, 0b00001110, 0b00001100, 0b00001000, 0b00011110, 0b00011100, 0b00011000, 0b00010000,
    0b00111110, 0b00111100, 0b00111000, 0b00110000, 0b00100000, 0b01111101, 0b01111001, 0b01111011,
    0b01110001, 0b01110011, 0b01110111, 0b01100001, 0b01100011, 0b01100111, 0b01101111, 0b01000001,
    0b01000011, 0b01000111, 0b01001111, 0b01011111, 0b00000101, 0b00001101, 0b00001001, 0b00001011,
    0b00011101, 0b00011001, 0b00011011, 0b00010001, 0b00010011, 0b00010111, 0b00111101, 0b00111001,
    0b00111011, 0b00110001, 0b00110011, 0b00110111, 0b00100001, 0b00100011, 0b00100111, 0b00101111,
    0b01111010, 0b01110010, 0b01110110, 0b01110100, 0b01100010, 0b01100110, 0b01100100, 0b01101110,
    0b01101100, 0b01101000, 0b01000010, 0b01000110, 0b01000100, 0b01001110, 0b01001100, 0b01001000,
    0b01011110, 0b01011100, 0b01011000, 0b01010000, 0b00001010, 0b00011010, 0b00010010, 0b00010110,
    0b00010100, 0b00111010, 0b00110010, 0b00110110, 0b00110100, 0b00100010, 0b00100110, 0b00100100,
    0b00101110, 0b00101100, 0b00101000, 0b01110101, 0b01100101, 0b01101101, 0b01101001, 0b01101011,
    0b01000101, 0b01001101, 0b01001001, 0b01001011, 0b01011101, 0b01011001, 0b01011011, 0b01010001,
    0b01010011, 0b01010111, 0b00010101, 0b00110101, 0b00100101, 0b00101101, 0b00101001, 0b00101011,
    0b01101010, 0b01001010, 0b01011010, 0b01010010, 0b01010110, 0b01010100, 0b00101010, 0b01010101,
];
const RLE_U8_B8: [u8; 256] = [
    0b00000000, 0b11111111, 0b00000001, 0b00000011, 0b00000111, 0b00001111, 0b00011111, 0b00111111,
    0b01111111, 0b11111110, 0b11111100, 0b11111000, 0b11110000, 0b11100000, 0b11000000, 0b10000000,
    0b00000010, 0b00000110, 0b00000100, 0b00001110, 0b00001100, 0b00001000, 0b00011110, 0b00011100,
    0b00011000, 0b00010000, 0b00111110, 0b00111100, 0b00111000, 0b00110000, 0b00100000, 0b01111110,
    0b01111100, 0b01111000, 0b01110000, 0b01100000, 0b01000000, 0b11111101, 0b11111001, 0b11111011,
    0b11110001, 0b11110011, 0b11110111, 0b11100001, 0b11100011, 0b11100111, 0b11101111, 0b11000001,
    0b11000011, 0b11000111, 0b11001111, 0b11011111, 0b10000001, 0b10000011, 0b10000111, 0b10001111,
    0b10011111, 0b10111111, 0b00000101, 0b00001101, 0b00001001, 0b00001011, 0b00011101, 0b00011001,
    0b00011011, 0b00010001, 0b00010011, 0b00010111, 0b00111101, 0b00111001, 0b00111011, 0b00110001,
    0b00110011, 0b00110111, 0b00100001, 0b00100011, 0b00100111, 0b00101111, 0b01111101, 0b01111001,
    0b01111011, 0b01110001, 0b01110011, 0b01110111, 0b01100001, 0b01100011, 0b01100111, 0b01101111,
    0b01000001, 0b01000011, 0b01000111, 0b01001111, 0b01011111, 0b11111010, 0b11110010, 0b11110110,
    0b11110100, 0b11100010, 0b11100110, 0b11100100, 0b11101110, 0b11101100, 0b11101000, 0b11000010,
    0b11000110, 0b11000100, 0b11001110, 0b11001100, 0b11001000, 0b11011110, 0b11011100, 0b11011000,
    0b11010000, 0b10000010, 0b10000110, 0b10000100, 0b10001110, 0b10001100, 0b10001000, 0b10011110,
    0b10011100, 0b10011000, 0b10010000, 0b10111110, 0b10111100, 0b10111000, 0b10110000, 0b10100000,
    0b00001010, 0b00011010, 0b00010010, 0b00010110, 0b00010100, 0b00111010, 0b00110010, 0b00110110,
    0b00110100, 0b00100010, 0b00100110, 0b00100100, 0b00101110, 0b00101100, 0b00101000, 0b01111010,
    0b01110010, 0b01110110, 0b01110100, 0b01100010, 0b01100110, 0b01100100, 0b01101110, 0b01101100,
    0b01101000, 0b01000010, 0b01000110, 0b01000100, 0b01001110, 0b01001100, 0b01001000, 0b01011110,
    0b01011100, 0b01011000, 0b01010000, 0b11110101, 0b11100101, 0b11101101, 0b11101001, 0b11101011,
    0b11000101, 0b11001101, 0b11001001, 0b11001011, 0b11011101, 0b11011001, 0b11011011, 0b11010001,
    0b11010011, 0b11010111, 0b10000101, 0b10001101, 0b10001001, 0b10001011, 0b10011101, 0b10011001,
    0b10011011, 0b10010001, 0b10010011, 0b10010111, 0b10111101, 0b10111001, 0b10111011, 0b10110001,
    0b10110011, 0b10110111, 0b10100001, 0b10100011, 0b10100111, 0b10101111, 0b00010101, 0b00110101,
    0b00100101, 0b00101101, 0b00101001, 0b00101011, 0b01110101, 0b01100101, 0b01101101, 0b01101001,
    0b01101011, 0b01000101, 0b01001101, 0b01001001, 0b01001011, 0b01011101, 0b01011001, 0b01011011,
    0b01010001, 0b01010011, 0b01010111, 0b11101010, 0b11001010, 0b11011010, 0b11010010, 0b11010110,
    0b11010100, 0b10001010, 0b10011010, 0b10010010, 0b10010110, 0b10010100, 0b10111010, 0b10110010,
    0b10110110, 0b10110100, 0b10100010, 0b10100110, 0b10100100, 0b10101110, 0b10101100, 0b10101000,
    0b00101010, 0b01101010, 0b01001010, 0b01011010, 0b01010010, 0b01010110, 0b01010100, 0b11010101,
    0b10010101, 0b10110101, 0b10100101, 0b10101101, 0b10101001, 0b10101011, 0b01010101, 0b10101010,
];

#[derive(Debug)]
pub enum RLEU8 {
    Bit1,
    Bit2,
    Bit3,
    Bit4,
    Bit5,
    Bit6,
    Bit7,
    Bit8,
}

impl RLEU8 {
    pub fn encode(&self, ix: usize) -> Option<&u8> {
        match self {
            RLEU8::Bit1 => RLE_U8_B1.get(ix),
            RLEU8::Bit2 => RLE_U8_B2.get(ix),
            RLEU8::Bit3 => RLE_U8_B3.get(ix),
            RLEU8::Bit4 => RLE_U8_B4.get(ix),
            RLEU8::Bit5 => RLE_U8_B5.get(ix),
            RLEU8::Bit6 => RLE_U8_B6.get(ix),
            RLEU8::Bit7 => RLE_U8_B7.get(ix),
            RLEU8::Bit8 => RLE_U8_B8.get(ix),
        }
    }
    pub fn decode(&self, pos: u8) -> Option<usize> {
        match self {
            RLEU8::Bit1 => RLE_U8_B1.iter().position(|&x| x == pos),
            RLEU8::Bit2 => RLE_U8_B2.iter().position(|&x| x == pos),
            RLEU8::Bit3 => RLE_U8_B3.iter().position(|&x| x == pos),
            RLEU8::Bit4 => RLE_U8_B4.iter().position(|&x| x == pos),
            RLEU8::Bit5 => RLE_U8_B5.iter().position(|&x| x == pos),
            RLEU8::Bit6 => RLE_U8_B6.iter().position(|&x| x == pos),
            RLEU8::Bit7 => RLE_U8_B7.iter().position(|&x| x == pos),
            RLEU8::Bit8 => RLE_U8_B8.iter().position(|&x| x == pos),
        }
    }
    pub fn new() -> Self {
        RLEU8::Bit8
    }
    pub fn with_bitlength(length: usize) -> Self {
        match length {
            1 => RLEU8::Bit1,
            2 => RLEU8::Bit2,
            3 => RLEU8::Bit3,
            4 => RLEU8::Bit4,
            5 => RLEU8::Bit5,
            6 => RLEU8::Bit6,
            7 => RLEU8::Bit7,
            8 => RLEU8::Bit8,
            _ => panic!("Only 1-8 bits supported"),
        }
    }
    pub fn bitlength(&self) -> usize {
        match self {
            RLEU8::Bit1 => 1,
            RLEU8::Bit2 => 2,
            RLEU8::Bit3 => 3,
            RLEU8::Bit4 => 4,
            RLEU8::Bit5 => 5,
            RLEU8::Bit6 => 6,
            RLEU8::Bit7 => 7,
            RLEU8::Bit8 => 8,
        }
    }
}

impl Default for RLEU8 {
    fn default() -> Self {
        RLEU8::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_code_b8() {
        let b = RLEU8::Bit8;
        assert_eq!(b.encode(0), Some(&0));
        assert_eq!(b.encode(1), Some(&255));
    }

    #[test]
    fn get_code_b2() {
        let b = RLEU8::Bit2;
        assert_eq!(b.encode(0), Some(&0));
        assert_eq!(b.encode(1), Some(&3));
        assert_eq!(b.encode(2), Some(&1));
        assert_eq!(b.encode(3), Some(&2));
        assert_eq!(b.encode(4), None);
    }
}