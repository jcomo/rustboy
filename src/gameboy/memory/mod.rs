mod boot;
mod dma;

use self::boot::DMG_BIN;
use self::dma::DMA;
use crate::gameboy::cartridge::Cartridge;
use crate::gameboy::cpu::MemoryBus;
use crate::gameboy::display::VideoDisplay;
use crate::gameboy::gpu::GPU;
use crate::gameboy::irq::IRQ;
use crate::gameboy::joypad::Joypad;
use crate::gameboy::serial::Serial;
use crate::gameboy::timer::Timer;
use crate::gameboy::Button;

const BOOT_ROM_SIZE: usize = 0x100;
const INTERNAL_RAM_SIZE: usize = 0x2000;
const HIRAM_SIZE: usize = 0x7F;
const EMPTY_READ: u8 = 0xFF;

pub struct MMU {
    elapsed_cycles: u8,
    is_checking_boot_rom: bool,
    boot_rom: [u8; BOOT_ROM_SIZE],
    internal_ram: [u8; INTERNAL_RAM_SIZE],
    hiram: [u8; HIRAM_SIZE],
    cartridge: Cartridge,
    gpu: GPU,
    irq: IRQ,
    timer: Timer,
    joypad: Joypad,
    serial: Serial,
    dma: DMA,
}

impl MMU {
    pub fn new(rom: Vec<u8>, display: Box<dyn VideoDisplay>) -> MMU {
        MMU {
            elapsed_cycles: 0,
            is_checking_boot_rom: true,
            boot_rom: DMG_BIN,
            internal_ram: [0; INTERNAL_RAM_SIZE],
            hiram: [0; HIRAM_SIZE],
            cartridge: Cartridge::new(rom),
            gpu: GPU::new(display),
            irq: IRQ::new(),
            timer: Timer::new(),
            joypad: Joypad::new(),
            serial: Serial::new(),
            dma: DMA::new(),
        }
    }

    pub fn button_down(&mut self, btn: Button) {
        self.joypad.button_down(&mut self.irq, btn);
    }

    pub fn button_up(&mut self, btn: Button) {
        self.joypad.button_up(btn);
    }

    pub fn get_and_reset_cycles(&mut self) -> u8 {
        let cycles = self.elapsed_cycles;
        self.elapsed_cycles = 0;
        cycles
    }

    fn tick_cycle(&mut self) {
        self.elapsed_cycles = self.elapsed_cycles.wrapping_add(1);
    }

    fn emulate(&mut self) {
        self.tick_cycle();
        self.emulate_oam_dma();
        self.gpu.emulate(&mut self.irq);
        self.timer.emulate(&mut self.irq);
    }

    fn emulate_oam_dma(&mut self) {
        self.dma.emulate().map(|address| {
            let value = self.get_byte_internal(address);
            self.gpu.write_oam(address as u8, value);
        });
    }

    fn get_byte_internal(&self, address: u16) -> u8 {
        let index = address as usize;
        match address >> 8 {
            0x00 if self.is_checking_boot_rom => self.boot_rom[index],
            0x00...0x3F => self.cartridge.read_rom_bank0(address),
            0x40...0x7F => self.cartridge.read_rom_bank1(address),
            0x80...0x97 => self.gpu.get_tile_row(address - 0x8000),
            0x98...0x9B => self.gpu.get_tile_map_0(address - 0x9800),
            0x9C...0x9F => self.gpu.get_tile_map_1(address - 0x9C00),
            0xA0...0xBF => self.cartridge.read_ram(address),
            0xC0...0xDF => self.internal_ram[index - 0xC000],
            0xFE => match address & 0xFF {
                0x00...0x9F => self.gpu.read_oam(address as u8),
                _ => EMPTY_READ,
            },
            0xFF => match address & 0xFF {
                0x00 => self.joypad.get_data(),
                0x01 => self.serial.get_data(),
                0x02 => self.serial.get_control(),
                0x04 => self.timer.get_div(),
                0x05 => self.timer.get_tima(),
                0x06 => self.timer.get_tma(),
                0x07 => self.timer.get_tac(),
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
                0x46 => self.dma.get_source(),
                0x47 => self.gpu.get_bg_palette(),
                0x48 => self.gpu.get_obj_palette_0(),
                0x49 => self.gpu.get_obj_palette_1(),
                0x4A => self.gpu.get_window_y(),
                0x4B => self.gpu.get_window_x(),
                0x4C...0x7F => EMPTY_READ,
                0x80...0xFE => self.hiram[index - 0xFF80],
                0xFF => self.irq.get_enabled_bits(),
                _ => panic!("unsupported read 0x{:X}", address),
            },
            _ => unreachable!(),
        }
    }

    fn set_byte_internal(&mut self, address: u16, byte: u8) {
        let index = address as usize;
        match address >> 8 {
            0x00...0x7F => self.cartridge.write_registers(address, byte),
            0x80...0x97 => self.gpu.set_tile_row(address - 0x8000, byte),
            0x98...0x9B => self.gpu.set_tile_map_0(address - 0x9800, byte),
            0x9C...0x9F => self.gpu.set_tile_map_1(address - 0x9C00, byte),
            0xA0...0xBF => self.cartridge.write_ram(address, byte),
            0xC0...0xDF => self.internal_ram[index - 0xC000] = byte,
            0xFE => match address & 0xFF {
                0x00...0x9F => self.gpu.write_oam(address as u8, byte),
                _ => (),
            },
            0xFF => match address & 0xFF {
                0x00 => self.joypad.set_data(byte),
                0x01 => self.serial.set_data(byte),
                0x02 => self.serial.set_control(byte),
                0x04 => self.timer.reset_div(),
                0x05 => self.timer.set_tima(byte),
                0x06 => self.timer.set_tma(byte),
                0x07 => self.timer.set_tac(byte),
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
                0x46 => self.dma.initialize(byte),
                0x47 => self.gpu.set_bg_palette(byte),
                0x48 => self.gpu.set_obj_palette_0(byte),
                0x49 => self.gpu.set_obj_palette_1(byte),
                0x50 => self.is_checking_boot_rom = false,
                0x4A => self.gpu.set_window_y(byte),
                0x4B => self.gpu.set_window_x(byte),
                0x4C...0x7F => (), // Empty
                0x80...0xFE => self.hiram[index - 0xFF80] = byte,
                0xFF => self.irq.set_enabled_bits(byte),
                _ => panic!("unsupported write 0x{:X} = {:X}", address, byte),
            },
            _ => unreachable!(),
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
