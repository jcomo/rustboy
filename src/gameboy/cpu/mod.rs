mod flags;
mod instructions;
mod registers;

use self::registers::Registers;

use crate::bits;

pub trait MemoryBus {
    fn get_byte(&self, address: u16) -> u8;
    fn set_byte(&mut self, address: u16, byte: u8);

    fn get_word(&self, address: u16) -> u16 {
        let lsb = self.get_byte(address);
        let msb = self.get_byte(address.wrapping_add(1));
        bits::to_word(msb, lsb)
    }

    fn set_word(&mut self, address: u16, word: u16) {
        self.set_byte(address, bits::lsb_16(word));
        self.set_byte(address.wrapping_add(1), bits::msb_16(word));
    }
}

#[derive(Default)]
pub struct CPU {
    registers: Registers,
}

impl CPU {
    pub fn step(&mut self, memory: &mut MemoryBus) {
        let op_code = self.get_byte(memory);
        instructions::execute(op_code, self, memory);
    }

    pub fn get_byte(&mut self, memory: &MemoryBus) -> u8 {
        let old_pc = self.registers.increment_pc();
        memory.get_byte(old_pc)
    }

    pub fn get_word(&mut self, memory: &MemoryBus) -> u16 {
        let lsb = self.get_byte(memory);
        let msb = self.get_byte(memory);
        bits::to_word(msb, lsb)
    }
}
