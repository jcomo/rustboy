mod flags;
mod registers;

use self::registers::Registers;

pub trait MemoryBus {
    fn get_byte(&self, address: u16) -> u8;
    fn set_byte(&mut self, address: u16, byte: u8);
}

pub struct CPU {
    registers: Registers,
    memory: Box<MemoryBus>,
}

impl CPU {
    pub fn new(memory: Box<MemoryBus>) -> CPU {
        CPU {
            registers: Registers::default(),
            memory: memory,
        }
    }

    pub fn step(&self) {}
}
