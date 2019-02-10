mod flags;
mod instructions;
mod registers;

use self::registers::Registers;

use crate::bits;

pub trait MemoryBus {
    fn ack_interrupt(&mut self) -> Option<u16>;
    fn get_byte(&mut self, address: u16) -> u8;
    fn set_byte(&mut self, address: u16, byte: u8);

    fn get_word(&mut self, address: u16) -> u16 {
        let lsb = self.get_byte(address);
        let msb = self.get_byte(address.wrapping_add(1));
        bits::to_word(msb, lsb)
    }

    fn set_word(&mut self, address: u16, word: u16) {
        self.set_byte(address, bits::lsb_16(word));
        self.set_byte(address.wrapping_add(1), bits::msb_16(word));
    }
}

#[derive(Default)]
pub struct CPU {
    registers: Registers,
    ime: bool,
    ime_queued: bool,
}

impl CPU {
    pub fn step(&mut self, bus: &mut MemoryBus) {
        self.service_interrupts(bus);
        if self.ime_queued {
            self.ime = true;
            self.ime_queued = false;
        }

        let op_code = self.get_byte(bus);
        instructions::execute(op_code, self, bus);
    }

    pub fn get_byte(&mut self, bus: &mut MemoryBus) -> u8 {
        let old_pc = self.registers.increment_pc();
        bus.get_byte(old_pc)
    }

    pub fn get_word(&mut self, bus: &mut MemoryBus) -> u16 {
        let lsb = self.get_byte(bus);
        let msb = self.get_byte(bus);
        bits::to_word(msb, lsb)
    }

    pub fn set_ime(&mut self) {
        self.ime_queued = true;
    }

    pub fn reset_ime(&mut self) {
        self.ime = false;
        self.ime_queued = false;
    }

    fn service_interrupts(&mut self, bus: &mut MemoryBus) {
        if !self.ime {
            return;
        }

        bus.ack_interrupt().map(|address| {
            self.reset_ime();

            // Push the current PC onto the stack
            self.registers.decrement_sp();
            self.registers.decrement_sp();
            bus.set_word(self.registers.sp, self.registers.pc);

            // Set the PC to the pre-determined address for the interrupt
            self.registers.pc = address;
        });
    }
}
