use super::flags::Flags;

#[derive(Debug, Default)]
pub struct Registers {
    pub pc: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: Flags,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    /// Increments the program counter and returns the old value
    pub fn increment_pc(&mut self) -> u16 {
        // TODO: should this wrap?
        let old_pc = self.pc;
        self.pc = self.pc + 1;
        old_pc
    }

    pub fn get_af(&self) -> u16 {
        self.get_word(self.a, u8::from(&self.f))
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = self.get_msb(value);
        self.f = Flags::from(self.get_lsb(value));
    }

    pub fn get_bc(&self) -> u16 {
        self.get_word(self.b, self.c)
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = self.get_msb(value);
        self.c = self.get_lsb(value);
    }

    pub fn get_de(&self) -> u16 {
        self.get_word(self.d, self.e)
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = self.get_msb(value);
        self.e = self.get_lsb(value);
    }

    pub fn get_hl(&self) -> u16 {
        self.get_word(self.h, self.l)
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = self.get_msb(value);
        self.l = self.get_lsb(value);
    }

    fn get_word(&self, upper: u8, lower: u8) -> u16 {
        (upper as u16) << 8 | lower as u16
    }

    fn get_msb(&self, value: u16) -> u8 {
        ((value & 0xFF00) >> 8) as u8
    }

    fn get_lsb(&self, value: u16) -> u8 {
        (value & 0xFF) as u8
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn incr_pc() {
        let mut regs = Registers::default();

        assert_eq!(regs.pc, 0);

        let old_pc = regs.increment_pc();

        assert_eq!(old_pc, 0);
        assert_eq!(regs.pc, 1);
    }

    #[test]
    fn af_combo() {
        let mut regs = Registers::default();

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
        let mut regs = Registers::default();

        regs.set_bc(0xF123);

        assert_eq!(regs.b, 0xF1);
        assert_eq!(regs.c, 0x23);
        assert_eq!(regs.get_bc(), 0xF123);
    }

    #[test]
    fn de_combo() {
        let mut regs = Registers::default();

        regs.set_de(0xF123);

        assert_eq!(regs.d, 0xF1);
        assert_eq!(regs.e, 0x23);
        assert_eq!(regs.get_de(), 0xF123);
    }

    #[test]
    fn hl_combo() {
        let mut regs = Registers::default();

        regs.set_hl(0xF123);

        assert_eq!(regs.h, 0xF1);
        assert_eq!(regs.l, 0x23);
        assert_eq!(regs.get_hl(), 0xF123);
    }
}
