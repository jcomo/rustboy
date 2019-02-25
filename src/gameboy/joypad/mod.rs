pub struct Joypad {
    data: u8,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad { data: 0 }
    }

    pub fn get_data(&self) -> u8 {
        println!("[joypad] get_data");
        self.data
    }

    pub fn set_data(&mut self, byte: u8) {
        println!("[joypad] set_data");
        self.data = byte;
    }
}
