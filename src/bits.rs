/// Gets the most significant bit of a word
pub fn msb_16(value: u16) -> u8 {
    ((value & 0xFF00) >> 8) as u8
}

/// Gets the least significant bit of a word
pub fn lsb_16(value: u16) -> u8 {
    (value & 0xFF) as u8
}

/// Adds to the value and discards the carry if there is any
pub fn add_16(value: u16, amount: u16) -> u16 {
    let (result, _) = value.overflowing_add(amount);
    result
}

/// Combines an most significant byte and least significant byte into a word
pub fn to_word(msb: u8, lsb: u8) -> u16 {
    (msb as u16) << 8 | lsb as u16
}
