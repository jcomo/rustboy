use crate::bits;

use super::flags::Flags;

#[derive(Debug, Default)]
pub struct Registers {
    pub pc: u16,
    pub sp: u16,
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
    /// Increments the program counter; returns the old value
    pub fn increment_pc(&mut self) -> u16 {
        self.add_pc(1)
    }

    /// Adds n to the program counter; returns the old value
    pub fn add_pc(&mut self, n: u8) -> u16 {
        let old_value = self.pc;
        self.pc = old_value.wrapping_add(n as u16);
        old_value
    }

    /// Increments the stack pointer; returns the old value
    pub fn increment_sp(&mut self) -> u16 {
        let old_value = self.sp;
        self.sp = old_value.wrapping_add(1);
        old_value
    }

    /// Decrements the stack pointer; returns the old value
    pub fn decrement_sp(&mut self) -> u16 {
        let old_value = self.sp;
        self.sp = old_value.wrapping_sub(1);
        old_value
    }

    pub fn get_af(&self) -> u16 {
        bits::to_word(self.a, u8::from(&self.f))
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = bits::msb_16(value);
        self.f = Flags::from(bits::lsb_16(value));
    }

    pub fn get_bc(&self) -> u16 {
        bits::to_word(self.b, self.c)
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = bits::msb_16(value);
        self.c = bits::lsb_16(value);
    }

    /// Increments the BC register; returns the old value
    pub fn increment_bc(&mut self) -> u16 {
        let old_value = self.get_bc();
        self.set_bc(old_value.wrapping_add(1));
        old_value
    }

    pub fn get_de(&self) -> u16 {
        bits::to_word(self.d, self.e)
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = bits::msb_16(value);
        self.e = bits::lsb_16(value);
    }

    /// Increments the DE register; returns the old value
    pub fn increment_de(&mut self) -> u16 {
        let old_value = self.get_de();
        self.set_de(old_value.wrapping_add(1));
        old_value
    }

    pub fn get_hl(&self) -> u16 {
        bits::to_word(self.h, self.l)
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = bits::msb_16(value);
        self.l = bits::lsb_16(value);
    }

    /// Increments the HL register; returns the old value
    pub fn increment_hl(&mut self) -> u16 {
        let old_value = self.get_hl();
        self.set_hl(old_value.wrapping_add(1));
        old_value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn increment_pc() {
        let mut regs = Registers::default();
        regs.pc = 0;

        assert_eq!(regs.increment_pc(), 0);
        assert_eq!(regs.pc, 1);
    }

    #[test]
    fn increment_sp() {
        let mut regs = Registers::default();
        regs.sp = 0x1;

        assert_eq!(regs.increment_sp(), 1);
        assert_eq!(regs.sp, 2);
    }

    #[test]
    fn decrement_sp() {
        let mut regs = Registers::default();
        regs.sp = 0x1;

        assert_eq!(regs.decrement_sp(), 1);
        assert_eq!(regs.sp, 0);
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

        assert_eq!(regs.increment_bc(), 0xF123);
        assert_eq!(regs.get_bc(), 0xF124);
    }

    #[test]
    fn de_combo() {
        let mut regs = Registers::default();

        regs.set_de(0xF123);

        assert_eq!(regs.d, 0xF1);
        assert_eq!(regs.e, 0x23);
        assert_eq!(regs.get_de(), 0xF123);

        assert_eq!(regs.increment_de(), 0xF123);
        assert_eq!(regs.get_de(), 0xF124);
    }

    #[test]
    fn hl_combo() {
        let mut regs = Registers::default();

        regs.set_hl(0xF123);

        assert_eq!(regs.h, 0xF1);
        assert_eq!(regs.l, 0x23);
        assert_eq!(regs.get_hl(), 0xF123);

        assert_eq!(regs.increment_hl(), 0xF123);
        assert_eq!(regs.get_hl(), 0xF124);
    }
}
