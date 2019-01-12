mod flags;
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
        let old_pc = self.registers.increment_pc();
        let op_code = memory.get_byte(old_pc);
        println!("op code: 0x{:X}", op_code);
    }
}
