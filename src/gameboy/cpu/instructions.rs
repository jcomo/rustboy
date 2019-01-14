use crate::bits;

use super::flags::Flags;
use super::MemoryBus;
use super::CPU;

pub fn execute(op: u8, cpu: &mut CPU, memory: &mut MemoryBus) {
    match op {
        0x08 => {
            debug("LD (nn), SP");
            let address = cpu.get_word(memory);
            memory.set_word(address, cpu.registers.sp);
        }
        0x21 => {
            debug("SLA C");
            let (value, flags) = shift_left(cpu.registers.c);
            cpu.registers.c = value;
            cpu.registers.f = flags;
        }
        0x22 => {
            debug("LD (HLI), A");
            let address = cpu.registers.increment_hl();
            memory.set_byte(address, cpu.registers.a);
        }
        0x31 => {
            debug("LD SP, nn");
            let word = cpu.get_word(memory);
            cpu.registers.sp = word;
        }
        0xAF => {
            debug("XOR A, A");
            let (value, flags) = xor(cpu.registers.a, cpu.registers.a);
            cpu.registers.a = value;
            cpu.registers.f = flags;
        }
        0xFF => {
            debug("RST 38H");
            reset(cpu, memory, 0x38);
        }
        _ => {
            panic!(format!("Unknown operation 0x{:X}", op));
        }
    }
}

fn xor(left: u8, right: u8) -> (u8, Flags) {
    let result = left ^ right;
    let flags = Flags {
        zero: result == 0,
        subtract: false,
        half_carry: false,
        carry: false,
    };

    (result, flags)
}

fn shift_left(value: u8) -> (u8, Flags) {
    let result = value << 1;
    let flags = Flags {
        zero: result == 0,
        subtract: false,
        half_carry: false,
        carry: result < value,
    };

    (result, flags)
}

fn push(cpu: &mut CPU, memory: &mut MemoryBus, address: u16) {
    cpu.registers.decrement_sp();
    memory.set_byte(cpu.registers.sp, bits::msb_16(address));
    cpu.registers.decrement_sp();
    memory.set_byte(cpu.registers.sp, bits::lsb_16(address));
}

fn reset(cpu: &mut CPU, memory: &mut MemoryBus, new_pc: u16) {
    push(cpu, memory, cpu.registers.pc);
    cpu.registers.pc = new_pc;
}

fn debug(label: &'static str) {
    println!("{}", label)
}
