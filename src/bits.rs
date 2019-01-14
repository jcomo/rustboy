/// Gets the most significant bit of a word
pub fn msb_16(value: u16) -> u8 {
    ((value & 0xFF00) >> 8) as u8
}

/// Gets the least significant bit of a word
pub fn lsb_16(value: u16) -> u8 {
    (value & 0xFF) as u8
}

/// Combines an most significant byte and least significant byte into a word
pub fn to_word(msb: u8, lsb: u8) -> u16 {
    (msb as u16) << 8 | lsb as u16
}
