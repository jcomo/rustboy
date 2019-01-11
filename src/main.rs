#[derive(Debug)]
struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

impl FlagsRegister {
    fn blank() -> FlagsRegister {
        FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl From<&FlagsRegister> for u8 {
    fn from(flags: &FlagsRegister) -> u8 {
        bool_to_bit(flags.zero) << ZERO_FLAG_BYTE_POSITION
            | bool_to_bit(flags.subtract) << SUBTRACT_FLAG_BYTE_POSITION
            | bool_to_bit(flags.half_carry) << HALF_CARRY_FLAG_BYTE_POSITION
            | bool_to_bit(flags.carry) << CARRY_FLAG_BYTE_POSITION
    }
}

impl From<u8> for FlagsRegister {
    fn from(byte: u8) -> FlagsRegister {
        FlagsRegister {
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

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

impl Registers {
    fn blank() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::blank(),
            h: 0,
            l: 0,
        }
    }

    fn get_af(&self) -> u16 {
        self.get_combo(self.a, u8::from(&self.f))
    }

    fn set_af(&mut self, value: u16) {
        self.a = self.get_upper(value);
        self.f = FlagsRegister::from(self.get_lower(value));
    }

    fn get_bc(&self) -> u16 {
        self.get_combo(self.b, self.c)
    }

    fn set_bc(&mut self, value: u16) {
        self.b = self.get_upper(value);
        self.c = self.get_lower(value);
    }

    fn get_de(&self) -> u16 {
        self.get_combo(self.d, self.e)
    }

    fn set_de(&mut self, value: u16) {
        self.d = self.get_upper(value);
        self.e = self.get_lower(value);
    }

    fn get_hl(&self) -> u16 {
        self.get_combo(self.h, self.l)
    }

    fn set_hl(&mut self, value: u16) {
        self.h = self.get_upper(value);
        self.l = self.get_lower(value);
    }

    fn get_combo(&self, upper: u8, lower: u8) -> u16 {
        (upper as u16) << 8 | lower as u16
    }

    fn get_upper(&self, value: u16) -> u8 {
        ((value & 0xFF00) >> 8) as u8
    }

    fn get_lower(&self, value: u16) -> u8 {
        (value & 0xFF) as u8
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn af_combo() {
        let mut regs = Registers::blank();

        regs.set_af(0xF1A0);

        assert_eq!(regs.a, 0xF1);
        assert_eq!(regs.f.zero, true);
        assert_eq!(regs.f.subtract, false);
        assert_eq!(regs.f.half_carry, true);
        assert_eq!(regs.f.carry, false);
        assert_eq!(regs.get_af(), 0xF1A0);
    }

    #[test]
    fn bc_combo() {
        let mut regs = Registers::blank();

        regs.set_bc(0xF123);

        assert_eq!(regs.b, 0xF1);
        assert_eq!(regs.c, 0x23);
        assert_eq!(regs.get_bc(), 0xF123);
    }

    #[test]
    fn de_combo() {
        let mut regs = Registers::blank();

        regs.set_de(0xF123);

        assert_eq!(regs.d, 0xF1);
        assert_eq!(regs.e, 0x23);
        assert_eq!(regs.get_de(), 0xF123);
    }

    #[test]
    fn hl_combo() {
        let mut regs = Registers::blank();

        regs.set_hl(0xF123);

        assert_eq!(regs.h, 0xF1);
        assert_eq!(regs.l, 0x23);
        assert_eq!(regs.get_hl(), 0xF123);
    }

    #[test]
    fn flags_to_u8() {
        let flags = FlagsRegister {
            zero: true,
            subtract: true,
            half_carry: false,
            carry: true,
        };

        assert_eq!(u8::from(&flags), 0xD0);
    }

    #[test]
    fn u8_to_flags() {
        let flags = FlagsRegister::from(0x50);

        assert_eq!(flags.zero, false);
        assert_eq!(flags.subtract, true);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.carry, true);
    }
}

fn main() {}
