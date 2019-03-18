use std::collections::HashSet;

use crate::gameboy::irq::Interrupt;
use crate::gameboy::irq::IRQ;
use crate::gameboy::Button;

impl Button {
    fn control_mask(&self) -> u8 {
        use self::Button::*;

        match *self {
            A | B | Start | Select => 0b0010_0000,
            Up | Down | Left | Right => 0b0001_0000,
        }
    }

    fn mask(&self) -> u8 {
        use self::Button::*;

        match self {
            A | Right => 0b0001,
            B | Left => 0b0010,
            Select | Up => 0b0100,
            Start | Down => 0b1000,
        }
    }
}

pub struct Joypad {
    data: u8,
    buttons: HashSet<Button>,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            data: 0,
            buttons: HashSet::new(),
        }
    }

    pub fn get_data(&self) -> u8 {
        let mut bytes: u8 = 0;

        for btn in self.buttons.iter() {
            if self.is_active(&btn) {
                bytes |= btn.mask();
            }
        }

        bytes
    }

    pub fn set_data(&mut self, byte: u8) {
        self.data = byte;
    }

    pub fn button_down(&mut self, irq: &mut IRQ, btn: Button) {
        if !self.is_active(&btn) {
            return;
        }

        if !self.buttons.contains(&btn) {
            // Newly pressed
            irq.set_interrupt(&Interrupt::Joypad);
        }

        self.buttons.insert(btn);
    }

    pub fn button_up(&mut self, btn: Button) {
        self.buttons.remove(&btn);
    }

    fn is_active(&self, btn: &Button) -> bool {
        self.data & btn.control_mask() > 0
    }
}
