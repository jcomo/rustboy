mod cpu;
mod memory;

use self::cpu::CPU;
use self::memory::MMU;

pub struct GameBoy {
    cpu: CPU,
    mmu: MMU,
}

impl GameBoy {
    pub fn load(cartridge: &Vec<u8>) -> GameBoy {
        // TODO: check for catridge too large?
        let rom = cartridge.to_owned();

        GameBoy {
            cpu: CPU::default(),
            mmu: MMU::new(rom),
        }
    }

    pub fn run(&mut self) {
        println!("[start] boot rom");
        loop {
            self.cpu.step(&mut self.mmu);
        }
    }
}
