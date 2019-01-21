mod boot;

use self::boot::DMG_BIN;
use crate::gameboy::cpu::MemoryBus;
use crate::gameboy::gpu::GPU;

pub struct MMU {
    boot_rom: [u8; 0x100],
    ram: [u8; 0x10_000],
    gpu: GPU,
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
            gpu: GPU::new(),
        }
    }

    fn emulate(&mut self) {
        self.gpu.emulate();
    }

    fn get_byte_internal(&self, address: u16) -> u8 {
        println!("(get_byte) ADDRESS: 0x{:x} (0x{:x})", address, address >> 8);
        let index = address as usize;
        match address >> 8 {
            0x00 => self.boot_rom[index],
            0xff => match address & 0xff {
                0x40 => self.gpu.get_control(),
                0x42 => self.gpu.get_scroll_y(),
                0x43 => self.gpu.get_scroll_x(),
                0x44 => self.gpu.get_current_line(),
                0x45 => self.gpu.get_compare_line(),
                0x46 => panic!("get DMA source"),
                0x47 => self.gpu.get_bg_palette(),
                0x48 => self.gpu.get_object_palette_0(),
                0x49 => self.gpu.get_object_palette_1(),
                0x4A => self.gpu.get_window_y(),
                0x4B => self.gpu.get_window_x(),
                0x80...0xFE => self.ram[index],
                _ => panic!("unsupported read 0x{:X}", address),
            },
            _ => self.ram[index],
        }
    }

    fn set_byte_internal(&mut self, address: u16, byte: u8) {
        println!("(set_byte) ADDRESS: 0x{:x} = 0x{:x}", address, byte);
        let index = address as usize;
        match address >> 8 {
            0x00 => self.boot_rom[index] = byte,
            0xff => match address & 0xff {
                0x11 => println!("SOUND NOT IMPLEMENTED"),
                0x12 => println!("SOUND NOT IMPLEMENTED"),
                0x24 => println!("SOUND NOT IMPLEMENTED"),
                0x25 => println!("SOUND NOT IMPLEMENTED"),
                0x26 => println!("SOUND NOT IMPLEMENTED"),
                0x40 => self.gpu.set_control(byte),
                0x42 => self.gpu.set_scroll_y(byte),
                0x43 => self.gpu.set_scroll_x(byte),
                0x44 => self.gpu.reset_current_line(),
                0x45 => self.gpu.set_compare_line(byte),
                0x46 => panic!("DMA request"),
                0x47 => self.gpu.set_bg_palette(byte),
                0x48 => self.gpu.set_object_palette_0(byte),
                0x49 => self.gpu.set_object_palette_1(byte),
                0x4A => self.gpu.set_window_y(byte),
                0x4B => self.gpu.set_window_x(byte),
                0x80...0xFE => self.ram[index] = byte,
                _ => panic!("unsupported write 0x{:X} = {:X}", address, byte),
            },
            _ => self.ram[index] = byte,
        }
    }
}

impl MemoryBus for MMU {
    fn get_byte(&mut self, address: u16) -> u8 {
        self.emulate();
        self.get_byte_internal(address)
    }

    fn set_byte(&mut self, address: u16, byte: u8) {
        self.emulate();
        self.set_byte_internal(address, byte)
    }
}
