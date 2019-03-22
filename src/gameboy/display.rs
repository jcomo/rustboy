use crate::gameboy::Color;

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
