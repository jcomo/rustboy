mod flags;
mod instructions;
mod registers;

use self::registers::Registers;

pub trait MemoryBus {
    fn get_byte(&self, address: u16) -> u8;
    fn set_byte(&mut self, address: u16, byte: u8);
}

#[derive(Default)]
pub struct CPU {
    registers: Registers,
}

impl CPU {
    pub fn step(&mut self, memory: &mut MemoryBus) {
        let op_code = self.get_byte(memory);
        println!("op code: 0x{:X}", op_code);
        instructions::execute(op_code, self, memory);
    }

    pub fn get_byte(&mut self, memory: &MemoryBus) -> u8 {
        let old_pc = self.registers.increment_pc();
        memory.get_byte(old_pc)
    }

    pub fn get_word(&mut self, memory: &MemoryBus) -> u16 {
        let lsb = self.get_byte(memory);
        let msb = self.get_byte(memory);
        (msb as u16) << 8 | lsb as u16
    }
}
