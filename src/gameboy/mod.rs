mod cpu;

use self::cpu::CPU;

pub struct GameBoy {
    cpu: CPU,
}

impl GameBoy {
    pub fn load(cartridge: Vec<u8>) -> GameBoy {
        // TODO: check for catridge too large?
        println!("{:?}", cartridge);
        GameBoy {
            cpu: CPU::default(),
        }
    }

    pub fn run(&self) {
        println!("GameBoy is running!");
    }
}
