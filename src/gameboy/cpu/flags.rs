use crate::bits;

#[derive(Debug, Default)]
pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl From<&Flags> for u8 {
    fn from(flags: &Flags) -> u8 {
        bits::set(7, flags.zero)
            | bits::set(6, flags.subtract)
            | bits::set(5, flags.half_carry)
            | bits::set(4, flags.carry)
    }
}

impl From<u8> for Flags {
    fn from(byte: u8) -> Flags {
        Flags {
            zero: bits::is_set(byte, 7),
            subtract: bits::is_set(byte, 6),
            half_carry: bits::is_set(byte, 5),
            carry: bits::is_set(byte, 4),
        }
    }
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
