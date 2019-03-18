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
        let mut bytes: u8 = self.mask();

        for btn in self.buttons.iter() {
            if self.is_active(&btn) {
                bytes &= !btn.mask();
            }
        }

        bytes
    }

    pub fn set_data(&mut self, byte: u8) {
        self.data = byte & 0b0011_0000;
    }

    pub fn button_down(&mut self, irq: &mut IRQ, btn: Button) {
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
        self.mask() & btn.control_mask() == 0
    }

    fn mask(&self) -> u8 {
        0b1100_1111 | self.data
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Joypad {
        fn test() -> Joypad {
            let mut joypad = Joypad::new();
            joypad.set_data(!Button::A.control_mask());
            joypad
        }
    }

    #[test]
    fn test_button_down() {
        let mut joypad = Joypad::test();
        let mut irq = IRQ::enabled();

        joypad.set_data(!Button::A.control_mask());
        joypad.button_down(&mut irq, Button::A);
        joypad.button_down(&mut irq, Button::B);

        assert_eq!(joypad.get_data(), 0b1101_1100);

        joypad.set_data(!Button::Up.control_mask());
        joypad.button_down(&mut irq, Button::Up);
        joypad.button_down(&mut irq, Button::Down);

        assert_eq!(joypad.get_data(), 0b1110_0011);
    }

    #[test]
    fn test_button_down_int() {
        let mut joypad = Joypad::test();
        let mut irq = IRQ::enabled();

        joypad.button_down(&mut irq, Button::B);
        assert_eq!(irq.ack_interrupt(), Some(Interrupt::Joypad.get_addr()));

        joypad.button_down(&mut irq, Button::B);
        assert_eq!(irq.ack_interrupt(), None);
    }

    #[test]
    fn test_button_up() {
        let mut joypad = Joypad::test();
        let mut irq = IRQ::enabled();

        joypad.button_down(&mut irq, Button::A);
        assert_eq!(joypad.get_data(), 0b1101_1110);

        joypad.button_up(Button::A);
        assert_eq!(joypad.get_data(), 0b1101_1111);
    }
}
