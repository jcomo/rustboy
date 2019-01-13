pub fn msb_16(value: u16) -> u8 {
    ((value & 0xFF00) >> 8) as u8
}

pub fn lsb_16(value: u16) -> u8 {
    (value & 0xFF) as u8
}
