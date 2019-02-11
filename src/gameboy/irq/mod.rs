use crate::bits;

pub enum Interrupt {
    VBlank,
    LCDC,
    Timer,
    Serial,
    Joypad,
}

impl Interrupt {
    fn from_bit(bit: u8) -> Option<Interrupt> {
        use self::Interrupt::*;

        match bit & 0b00011111 {
            0b00001 => Some(VBlank),
            0b00010 => Some(LCDC),
            0b00100 => Some(Timer),
            0b01000 => Some(Serial),
            0b10000 => Some(Joypad),
            _ => None,
        }
    }

    fn bit_position(&self) -> u8 {
        use self::Interrupt::*;

        match self {
            VBlank => 0,
            LCDC => 1,
            Timer => 2,
            Serial => 3,
            Joypad => 4,
        }
    }

    pub fn get_addr(&self) -> u16 {
        use self::Interrupt::*;

        match self {
            VBlank => 0x40,
            LCDC => 0x48,
            Timer => 0x50,
            Serial => 0x58,
            Joypad => 0x60,
        }
    }
}

/// Returns a u8 with the first rightmost enabled bit as the only enabled bit
fn isolate_rightmost_bit(byte: u8) -> u8 {
    for pos in 0..8 {
        let value = byte & (1 << pos);
        if value > 0 {
            return value;
        }
    }

    0
}

pub struct IRQ {
    enabled_bits: u8,
    interrupt_bits: u8,
}

impl IRQ {
    pub fn new() -> IRQ {
        IRQ {
            enabled_bits: 0,
            interrupt_bits: 0,
        }
    }

    pub fn ack_interrupt(&mut self) -> Option<u16> {
        let enabled_interrupts = self.enabled_bits & self.interrupt_bits;
        let bit = isolate_rightmost_bit(enabled_interrupts);
        Interrupt::from_bit(bit).map(|int| {
            self.reset_interrupt(&int);
            int.get_addr()
        })
    }

    pub fn get_enabled_bits(&self) -> u8 {
        self.enabled_bits
    }

    pub fn set_enabled_bits(&mut self, bits: u8) {
        self.enabled_bits = bits
    }

    pub fn get_interrupt_bits(&self) -> u8 {
        self.interrupt_bits
    }

    pub fn set_interrupt_bits(&mut self, bits: u8) {
        self.interrupt_bits = bits
    }

    pub fn set_interrupt(&mut self, interrupt: &Interrupt) {
        let pos = interrupt.bit_position();
        self.interrupt_bits = bits::set(self.interrupt_bits, pos);
    }

    pub fn reset_interrupt(&mut self, interrupt: &Interrupt) {
        let pos = interrupt.bit_position();
        self.interrupt_bits = bits::reset(self.interrupt_bits, pos);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bit_helpers() {
        assert_eq!(isolate_rightmost_bit(0b110100), 0b100);
        assert_eq!(isolate_rightmost_bit(0b11), 0b1);
        assert_eq!(isolate_rightmost_bit(0b0000), 0);
    }

    #[test]
    fn ack_interrupt() {
        let mut irq = IRQ::new();

        irq.set_interrupt(&Interrupt::VBlank);
        irq.set_interrupt(&Interrupt::Timer);
        irq.set_interrupt(&Interrupt::Joypad);

        assert_eq!(irq.ack_interrupt(), None);

        // Enable all but VBlank
        irq.enabled_bits = 0b00011110;

        assert_eq!(irq.ack_interrupt(), Some(Interrupt::Timer.get_addr()));

        // Now enable VBlank, which will take priority
        irq.enabled_bits = irq.enabled_bits | 0b1;

        assert_eq!(irq.ack_interrupt(), Some(Interrupt::VBlank.get_addr()));
        assert_eq!(irq.ack_interrupt(), Some(Interrupt::Joypad.get_addr()));

        assert_eq!(irq.ack_interrupt(), None);
        assert_eq!(irq.interrupt_bits, 0);
    }

    #[test]
    fn set_interrupt() {
        let mut irq = IRQ::new();

        irq.set_interrupt(&Interrupt::VBlank);
        assert_eq!(irq.interrupt_bits, 0b00000001);

        irq.set_interrupt(&Interrupt::LCDC);
        assert_eq!(irq.interrupt_bits, 0b00000011);

        irq.set_interrupt(&Interrupt::Timer);
        assert_eq!(irq.interrupt_bits, 0b00000111);

        irq.set_interrupt(&Interrupt::Serial);
        assert_eq!(irq.interrupt_bits, 0b00001111);

        irq.set_interrupt(&Interrupt::Joypad);
        assert_eq!(irq.interrupt_bits, 0b00011111);
    }

    #[test]
    fn reset_interrupt() {
        let mut irq = IRQ::new();
        irq.interrupt_bits = 0b00011111;

        irq.reset_interrupt(&Interrupt::VBlank);
        assert_eq!(irq.interrupt_bits, 0b00011110);

        irq.reset_interrupt(&Interrupt::LCDC);
        assert_eq!(irq.interrupt_bits, 0b00011100);

        irq.reset_interrupt(&Interrupt::Timer);
        assert_eq!(irq.interrupt_bits, 0b00011000);

        irq.reset_interrupt(&Interrupt::Serial);
        assert_eq!(irq.interrupt_bits, 0b00010000);

        irq.reset_interrupt(&Interrupt::Joypad);
        assert_eq!(irq.interrupt_bits, 0b00000000);
    }
}
