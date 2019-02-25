pub struct Timer {}

impl Timer {
    pub fn new() -> Timer {
        Timer {}
    }

    pub fn get_div(&self) -> u8 {
        0
    }

    pub fn reset_div(&mut self) {}

    pub fn get_tima(&self) -> u8 {
        0
    }

    pub fn set_tima(&mut self, byte: u8) {}

    pub fn get_tma(&self) -> u8 {
        0
    }

    pub fn set_tma(&mut self, byte: u8) {}

    pub fn get_tac(&self) -> u8 {
        0
    }

    pub fn set_tac(&mut self, byte: u8) {}
}
