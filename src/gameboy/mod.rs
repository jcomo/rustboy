mod cpu;
mod memory;

use self::cpu::CPU;
use self::memory::MMU;

pub struct GameBoy {
    cpu: CPU,
}

impl GameBoy {
    pub fn load(cartridge: Vec<u8>) -> GameBoy {
        // TODO: check for catridge too large?
        println!("{:?}", cartridge);
        let mmu = Box::new(MMU::default());
        GameBoy { cpu: CPU::new(mmu) }
    }

    pub fn run(&self) {
        println!("GameBoy is running!");
    }
}
