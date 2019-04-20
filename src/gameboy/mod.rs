pub mod clock;
pub mod display;

mod cartridge;
mod cpu;
mod gpu;
mod irq;
mod joypad;
mod memory;
mod serial;
mod timer;

use self::clock::Clock;
use self::cpu::CPU;
use self::display::VideoDisplay;
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

pub struct GameBoy {
    cpu: CPU,
    mmu: MMU,
    clock: Box<dyn Clock>,
}

impl GameBoy {
    pub fn new(
        cartridge: &Vec<u8>,
        clock: Box<dyn Clock>,
        display: Box<dyn VideoDisplay>,
    ) -> GameBoy {
        // TODO: check for catridge too large?
        let rom = cartridge.to_owned();

        GameBoy {
            cpu: CPU::default(),
            mmu: MMU::new(rom, display),
            clock,
        }
    }

    pub fn step(&mut self) -> u8 {
        self.cpu.step(&mut self.mmu);
        let cycles = self.mmu.get_and_reset_cycles();
        self.clock.tick(cycles);
        cycles
    }

    pub fn button_down(&mut self, btn: Button) {
        self.mmu.button_down(btn);
    }

    pub fn button_up(&mut self, btn: Button) {
        self.mmu.button_up(btn);
    }
}
