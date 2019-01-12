mod flags;
mod registers;

use self::registers::Registers;

#[derive(Default)]
pub struct CPU {
    registers: Registers,
}

impl CPU {
    pub fn step(&self) {}
}
