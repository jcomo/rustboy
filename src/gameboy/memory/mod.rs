mod boot;

use self::boot::DMG_BIN;
use crate::gameboy::cpu;

pub struct MMU {
    boot_rom: [u8; 0x100],
    ram: [u8; 0x10_000],
}

impl MMU {
    pub fn new(rom: Vec<u8>) -> MMU {
        let mut ram = [0; 0x10_000];
        for (i, byte) in rom.iter().enumerate() {
            ram[i] = byte.clone();
        }

        MMU {
            boot_rom: DMG_BIN,
            ram: ram,
        }
    }
}

impl cpu::MemoryBus for MMU {
    fn get_byte(&self, address: u16) -> u8 {
        println!("(get_byte) ADDRESS: 0x{:x} (0x{:x})", address, address >> 8);
        match address >> 8 {
            0x0 => self.boot_rom[address as usize],
            _ => self.ram[address as usize],
        }
    }

    fn set_byte(&mut self, address: u16, byte: u8) {
        println!("(set_byte) ADDRESS: 0x{:x} = 0x{:x}", address, byte);
        self.ram[address as usize] = byte
    }
}
