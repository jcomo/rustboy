pub enum Interrupt {
    VBlank,
    LCD,
    Timer,
    Serial,
    Joypad,
}

impl Interrupt {
    fn get_addr(self) -> u16 {
        match self {
            Interrupt::VBlank => 0x40,
            Interrupt::LCD => 0x48,
            Interrupt::Timer => 0x50,
            Interrupt::Serial => 0x58,
            Interrupt::Joypad => 0x60,
        }
    }
}

pub struct IRQ {}

impl IRQ {
    pub fn new() -> IRQ {
        IRQ {}
    }

    pub fn ack_interrupt(&mut self) -> Option<u16> {
        None
    }
}
