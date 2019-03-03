use crate::bits;
use crate::gameboy::irq::Interrupt;
use crate::gameboy::irq::IRQ;

/**
 * The TAC contains control bits for the timer. The Gameboy clock speed is
 * ~1.048 MHz. Use the input clock select frequency as a denominator to determine
 * how many machine cycles it will take to increment once.
 *
 * For example, if the input clock select is 4096 Hz, the number of machine cycles to increment
 * would be 1.048 MHz / 4096 Hz ~= 256.
 *
 * Bit  2   - Timer Enable
 * Bits 1-0 - Input Clock Select
 *    00: 4096 Hz; 256 cycles
 *    01: 262144 Hz; 4 cycles
 *    10: 65536 Hz; 16 cycles
 *    11: 16384 Hz; 64 cycles
 *
 * The divider register always increments, but TIMA only increments when the timer
 * is enabled.
 */
#[derive(Default, Debug)]
struct Control {
    enabled: bool,
    clock_bit_1: bool,
    clock_bit_0: bool,
}

impl Control {
    fn counter_mask(&self) -> u16 {
        let cycles = match self.clock_select() {
            0b00 => 256,
            0b01 => 4,
            0b10 => 16,
            0b11 => 64,
            _ => unreachable!(),
        };

        cycles - 1
    }

    fn clock_select(&self) -> u8 {
        bits::from_bool(self.clock_bit_1) << 1 | bits::from_bool(self.clock_bit_0)
    }
}

impl From<u8> for Control {
    fn from(byte: u8) -> Control {
        Control {
            enabled: bits::is_set(byte, 2),
            clock_bit_1: bits::is_set(byte, 1),
            clock_bit_0: bits::is_set(byte, 0),
        }
    }
}

impl From<&Control> for u8 {
    fn from(control: &Control) -> u8 {
        bits::from_bool(control.enabled) << 2
            | bits::from_bool(control.clock_bit_1) << 1
            | bits::from_bool(control.clock_bit_0)
    }
}

pub struct Timer {
    tima: u8,
    tma: u8,
    tac: Control,
    counter: u16,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            tima: 0,
            tma: 0,
            tac: Control::default(),
            counter: 0,
        }
    }

    pub fn emulate(&mut self, irq: &mut IRQ) {
        self.counter = self.counter.wrapping_add(1);
        if !self.tac.enabled {
            return;
        }

        if !self.can_increment_tima() {
            return;
        }

        let (value, overflow) = self.tima.overflowing_add(1);
        if overflow {
            self.tima = self.tma;
            irq.set_interrupt(&Interrupt::Timer);
        } else {
            self.tima = value;
        }
    }

    fn can_increment_tima(&self) -> bool {
        let mask = self.tac.counter_mask();
        self.counter & mask == 0
    }

    pub fn get_div(&self) -> u8 {
        // Set frequency of 16384 Hz, so will count up every 2^6 = 64 cycles
        println!("[timer] get_div");
        (self.counter >> 6) as u8
    }

    pub fn reset_div(&mut self) {
        println!("[timer] reset_div");
        self.counter = 0;
    }

    pub fn get_tima(&self) -> u8 {
        println!("[timer] get_tima");
        self.tima
    }

    pub fn set_tima(&mut self, byte: u8) {
        println!("[timer] set_tima");
        self.tima = byte;
    }

    pub fn get_tma(&self) -> u8 {
        println!("[timer] get_tma");
        self.tma
    }

    pub fn set_tma(&mut self, byte: u8) {
        println!("[timer] set_tma");
        self.tma = byte;
    }

    pub fn get_tac(&self) -> u8 {
        println!("[timer] get_tac");
        u8::from(&self.tac)
    }

    pub fn set_tac(&mut self, byte: u8) {
        println!("[timer] set_tac");
        self.tac = Control::from(byte);
    }
}
