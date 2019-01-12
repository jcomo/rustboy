use super::flags::Flags;

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: Flags,
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
            f: Flags::blank(),
            h: 0,
            l: 0,
        }
    }

    fn get_af(&self) -> u16 {
        self.get_combo(self.a, u8::from(&self.f))
    }

    fn set_af(&mut self, value: u16) {
        self.a = self.get_upper(value);
        self.f = Flags::from(self.get_lower(value));
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
}
