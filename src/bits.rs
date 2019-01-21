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

/// Converts a bool value to a bit
pub fn from_bool(value: bool) -> u8 {
    if value {
        1
    } else {
        0
    }
}

/// Converts a bit to a bool value
pub fn to_bool(value: u8) -> bool {
    value & 0x1 != 0
}

/// Returns u8 with value of bit set
pub fn set(bit: u8, value: bool) -> u8 {
    from_bool(value) << bit
}

/// Returns true if the index of the bit is set
pub fn is_set(value: u8, bit: u8) -> bool {
    to_bool(value >> bit)
}
