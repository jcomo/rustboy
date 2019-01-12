use crate::gameboy::cpu;

#[derive(Default)]
pub struct MMU {
    test: bool,
}

impl cpu::MemoryBus for MMU {
    fn get_byte(&self, address: u16) -> u8 {
        0
    }

    fn set_byte(&mut self, address: u16, byte: u8) {}
}
