pub struct Timer {}

impl Timer {
    pub fn new() -> Timer {
        Timer {}
    }

    pub fn get_div(&self) -> u8 {
        println!("[timer] get_div");
        0
    }

    pub fn reset_div(&mut self) {
        println!("[timer] reset_div");
    }

    pub fn get_tima(&self) -> u8 {
        println!("[timer] get_tima");
        0
    }

    pub fn set_tima(&mut self, byte: u8) {
        println!("[timer] set_tima");
    }

    pub fn get_tma(&self) -> u8 {
        println!("[timer] get_tma");
        0
    }

    pub fn set_tma(&mut self, byte: u8) {
        println!("[timer] set_tma");
    }

    pub fn get_tac(&self) -> u8 {
        println!("[timer] get_tac");
        0
    }

    pub fn set_tac(&mut self, byte: u8) {
        println!("[timer] set_tac");
    }
}
