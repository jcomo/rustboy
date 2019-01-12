#[derive(Debug, Default)]
pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl From<&Flags> for u8 {
    fn from(flags: &Flags) -> u8 {
        bool_to_bit(flags.zero) << ZERO_FLAG_BYTE_POSITION
            | bool_to_bit(flags.subtract) << SUBTRACT_FLAG_BYTE_POSITION
            | bool_to_bit(flags.half_carry) << HALF_CARRY_FLAG_BYTE_POSITION
            | bool_to_bit(flags.carry) << CARRY_FLAG_BYTE_POSITION
    }
}

impl From<u8> for Flags {
    fn from(byte: u8) -> Flags {
        Flags {
            zero: bit_to_bool(byte >> ZERO_FLAG_BYTE_POSITION),
            subtract: bit_to_bool(byte >> SUBTRACT_FLAG_BYTE_POSITION),
            half_carry: bit_to_bool(byte >> HALF_CARRY_FLAG_BYTE_POSITION),
            carry: bit_to_bool(byte >> CARRY_FLAG_BYTE_POSITION),
        }
    }
}

fn bool_to_bit(value: bool) -> u8 {
    if value {
        1
    } else {
        0
    }
}

fn bit_to_bool(value: u8) -> bool {
    value & 0x1 != 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn flags_to_u8() {
        let flags = Flags {
            zero: true,
            subtract: true,
            half_carry: false,
            carry: true,
        };

        assert_eq!(u8::from(&flags), 0xD0);
    }

    #[test]
    fn u8_to_flags() {
        let flags = Flags::from(0x50);

        assert_eq!(flags.zero, false);
        assert_eq!(flags.subtract, true);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.carry, true);
    }
}
