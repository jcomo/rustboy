mod cpu;

use self::cpu::CPU;

pub struct GameBoy {
    cpu: CPU,
}

impl GameBoy {
    pub fn run() {
        println!("GameBoy is running!");
    }
}
