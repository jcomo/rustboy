use crate::gameboy::cpu;

pub struct MMU {
    rom_bank_0: [u8; 0x4000],
}

impl MMU {
    pub fn new(rom: Vec<u8>) -> MMU {
        let mut rom_bank_0 = [0; 0x4000];
        for (i, byte) in rom.iter().enumerate() {
            rom_bank_0[i] = byte.clone();
        }

        MMU {
            rom_bank_0: rom_bank_0,
        }
    }
}

impl cpu::MemoryBus for MMU {
    fn get_byte(&self, address: u16) -> u8 {
        self.rom_bank_0[address as usize]
    }

    fn set_byte(&mut self, address: u16, byte: u8) {
        self.rom_bank_0[address as usize] = byte
    }
}
