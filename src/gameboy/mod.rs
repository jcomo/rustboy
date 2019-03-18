mod cpu;
mod gpu;
mod irq;
mod joypad;
mod memory;
mod serial;
mod timer;

use self::cpu::CPU;
use self::memory::MMU;

#[derive(Hash, Eq, PartialEq)]
pub enum Button {
    A,
    B,
    Start,
    Select,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Color {
    White = 0b00,
    Light = 0b01,
    Dark = 0b10,
    Black = 0b11,
}

pub trait VideoDisplay {
    fn set_pixel(&mut self, x: u8, y: u8, color: Color);
    fn vsync(&mut self);
}

pub struct NoDisplay {}

impl NoDisplay {
    pub fn new() -> NoDisplay {
        NoDisplay {}
    }
}

impl VideoDisplay for NoDisplay {
    fn set_pixel(&mut self, x: u8, y: u8, color: Color) {}
    fn vsync(&mut self) {}
}

pub struct GameBoy {
    cpu: CPU,
    mmu: MMU,
}

impl GameBoy {
    pub fn new(cartridge: &Vec<u8>, display: Box<dyn VideoDisplay>) -> GameBoy {
        // TODO: check for catridge too large?
        let rom = cartridge.to_owned();

        GameBoy {
            cpu: CPU::default(),
            mmu: MMU::new(rom, display),
        }
    }

    pub fn step(&mut self) {
        self.cpu.step(&mut self.mmu);
    }

    pub fn button_down(&mut self, btn: Button) {
        self.mmu.button_down(btn);
    }

    pub fn button_up(&mut self, btn: Button) {
        self.mmu.button_up(btn);
    }
}
