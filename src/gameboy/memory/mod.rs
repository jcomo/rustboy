mod boot;

use self::boot::DMG_BIN;
use crate::gameboy::cpu::MemoryBus;
use crate::gameboy::gpu::GPU;
use crate::gameboy::irq::IRQ;
use crate::gameboy::serial::Serial;
use crate::gameboy::VideoDisplay;

pub struct MMU {
    is_checking_boot_rom: bool,
    boot_rom: [u8; 0x100],
    ram: [u8; 0x10_000],
    gpu: GPU,
    irq: IRQ,
    serial: Serial,
}

impl MMU {
    pub fn new(rom: Vec<u8>, display: Box<dyn VideoDisplay>) -> MMU {
        let mut ram = [0; 0x10_000];
        for (i, byte) in rom.iter().enumerate() {
            ram[i] = byte.clone();
        }

        MMU {
            is_checking_boot_rom: true,
            boot_rom: DMG_BIN,
            ram: ram,
            gpu: GPU::new(display),
            irq: IRQ::new(),
            serial: Serial::new(),
        }
    }

    fn emulate(&mut self) {
        self.gpu.emulate(&mut self.irq);
    }

    fn get_byte_internal(&self, address: u16) -> u8 {
        let index = address as usize;
        match address >> 8 {
            0x00 => {
                if self.is_checking_boot_rom {
                    self.boot_rom[index]
                } else {
                    self.ram[index]
                }
            }
            0x80...0x97 => self.gpu.get_tile_row(address - 0x8000),
            0x98...0x9B => self.gpu.get_tile_map_0(address - 0x9800),
            0x9C...0x9F => self.gpu.get_tile_map_1(address - 0x9C00),
            0xFF => match address & 0xFF {
                0x01 => self.serial.get_data(),
                0x02 => self.serial.get_control(),
                0x0F => self.irq.get_interrupt_bits(),
                0x10...0x14 => self.read_sound_byte(address),
                0x16...0x2F => self.read_sound_byte(address),
                0x30...0x3F => self.read_sound_byte(address),
                0x40 => self.gpu.get_control(),
                0x41 => self.gpu.get_stat(),
                0x42 => self.gpu.get_scroll_y(),
                0x43 => self.gpu.get_scroll_x(),
                0x44 => self.gpu.get_current_line(),
                0x45 => self.gpu.get_compare_line(),
                0x46 => panic!("get DMA source"),
                0x47 => self.gpu.get_bg_palette(),
                0x48 => self.gpu.get_obj_palette_0(),
                0x49 => self.gpu.get_obj_palette_1(),
                0x4A => self.gpu.get_window_y(),
                0x4B => self.gpu.get_window_x(),
                0x4C...0x7F => 0xFF, // Empty
                0x80...0xFE => self.ram[index],
                0xFF => self.irq.get_enabled_bits(),
                _ => panic!("unsupported read 0x{:X}", address),
            },
            _ => {
                println!("(get_byte) ADDRESS: 0x{:x} (0x{:x})", address, address >> 8);
                self.ram[index]
            }
        }
    }

    fn set_byte_internal(&mut self, address: u16, byte: u8) {
        let index = address as usize;
        match address >> 8 {
            0x00 => self.boot_rom[index] = byte,
            0x80...0x97 => self.gpu.set_tile_row(address - 0x8000, byte),
            0x98...0x9B => self.gpu.set_tile_map_0(address - 0x9800, byte),
            0x9C...0x9F => self.gpu.set_tile_map_1(address - 0x9C00, byte),
            0xFF => match address & 0xFF {
                0x01 => self.serial.set_data(byte),
                0x02 => self.serial.set_control(byte),
                0x0F => self.irq.set_interrupt_bits(byte),
                0x10...0x14 => self.write_sound_byte(address, byte),
                0x16...0x2F => self.write_sound_byte(address, byte),
                0x30...0x3F => self.write_sound_byte(address, byte),
                0x40 => self.gpu.set_control(byte),
                0x41 => self.gpu.set_stat(byte),
                0x42 => self.gpu.set_scroll_y(byte),
                0x43 => self.gpu.set_scroll_x(byte),
                0x44 => self.gpu.reset_current_line(),
                0x45 => self.gpu.set_compare_line(byte),
                0x46 => println!("DMA request"),
                0x47 => self.gpu.set_bg_palette(byte),
                0x48 => self.gpu.set_obj_palette_0(byte),
                0x49 => self.gpu.set_obj_palette_1(byte),
                0x50 => self.is_checking_boot_rom = false,
                0x4A => self.gpu.set_window_y(byte),
                0x4B => self.gpu.set_window_x(byte),
                0x4C...0x7F => (/* Empty */),
                0x80...0xFE => self.ram[index] = byte,
                0xFF => self.irq.set_enabled_bits(byte),
                _ => panic!("unsupported write 0x{:X} = {:X}", address, byte),
            },
            _ => {
                println!("(set_byte) ADDRESS: 0x{:x} = 0x{:x}", address, byte);
                self.ram[index] = byte;
            }
        }
    }

    fn read_sound_byte(&self, address: u16) -> u8 {
        println!("[sound] not implemented; read (0x{:x})", address);
        0xFF
    }

    fn write_sound_byte(&mut self, address: u16, byte: u8) {
        println!(
            "[sound] not implemented; write (0x{:x}) = 0x{:x}",
            address, byte
        );
    }
}

impl MemoryBus for MMU {
    fn ack_interrupt(&mut self) -> Option<u16> {
        self.irq.ack_interrupt()
    }

    fn get_byte(&mut self, address: u16) -> u8 {
        self.emulate();
        self.get_byte_internal(address)
    }

    fn set_byte(&mut self, address: u16, byte: u8) {
        self.emulate();
        self.set_byte_internal(address, byte)
    }
}
