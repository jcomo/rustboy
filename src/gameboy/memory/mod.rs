use crate::gameboy::cpu;

#[derive(Default)]
pub struct MMU {
    rom: Vec<u8>,
}

impl MMU {
    pub fn new(rom: Vec<u8>) -> MMU {
        MMU { rom: rom }
    }
}

impl cpu::MemoryBus for MMU {
    fn get_byte(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn set_byte(&mut self, address: u16, byte: u8) {
        self.rom[address as usize] = byte
    }
}
