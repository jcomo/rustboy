use crate::gameboy::cpu;

pub struct MMU {
    // TODO: replace with individual sections
    ram: [u8; 0x10_000],
}

impl MMU {
    pub fn new(rom: Vec<u8>) -> MMU {
        let mut ram = [0; 0x10_000];
        for (i, byte) in rom.iter().enumerate() {
            ram[i] = byte.clone();
        }

        MMU { ram: ram }
    }
}

impl cpu::MemoryBus for MMU {
    fn get_byte(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    fn set_byte(&mut self, address: u16, byte: u8) {
        self.ram[address as usize] = byte
    }
}
