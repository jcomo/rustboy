use crate::bits;

use super::MemoryBus;
use super::CPU;

pub fn execute(op: u8, cpu: &mut CPU, memory: &mut MemoryBus) {
    if op == 0xCB {
        let op = cpu.get_byte(memory);
        debug(&format!("op code: 0xCB{:X}", op));
        execute_extended(op, cpu, memory);
    } else {
        debug(&format!("op code: 0x{:X}", op));
        execute_standard(op, cpu, memory);
    }
}

/// Execute commands in the standard instruction space
fn execute_standard(op: u8, cpu: &mut CPU, memory: &mut MemoryBus) {
    match op {
        0x00 => debug("NOP"),
        0x03 => {
            debug("INC BC");
            cpu.registers.increment_bc();
        }
        0x05 => {
            debug("DEC B");
            cpu.registers.b = dec(cpu, cpu.registers.b);
        }
        0x08 => {
            debug("LD (nn), SP");
            let address = cpu.get_word(memory);
            memory.set_word(address, cpu.registers.sp);
        }
        0x0D => {
            debug("DEC C");
            cpu.registers.c = dec(cpu, cpu.registers.c);
        }
        0x13 => {
            debug("INC DE");
            cpu.registers.increment_de();
        }
        0x15 => {
            debug("DEC D");
            cpu.registers.d = dec(cpu, cpu.registers.d);
        }
        0x1D => {
            debug("DEC E");
            cpu.registers.e = dec(cpu, cpu.registers.e);
        }
        0x20 => {
            debug("JR NZ, n");
            jr_cc(cpu, memory, !cpu.registers.f.zero);
        }
        0x21 => {
            debug("LD HL, nn");
            let word = cpu.get_word(memory);
            cpu.registers.set_hl(word);
        }
        0x22 => {
            debug("LDI (HL), A");
            let address = cpu.registers.increment_hl();
            memory.set_byte(address, cpu.registers.a);
        }
        0x23 => {
            debug("INC HL");
            cpu.registers.increment_hl();
        }
        0x25 => {
            debug("DEC H");
            cpu.registers.h = dec(cpu, cpu.registers.h);
        }
        0x28 => {
            debug("JR Z, n");
            jr_cc(cpu, memory, cpu.registers.f.zero);
        }
        0x2D => {
            debug("DEC L");
            cpu.registers.l = dec(cpu, cpu.registers.l);
        }
        0x30 => {
            debug("JR NC, n");
            jr_cc(cpu, memory, !cpu.registers.f.carry);
        }
        0x31 => {
            debug("LD SP, nn");
            cpu.registers.sp = cpu.get_word(memory);
        }
        0x32 => {
            debug("LDD (HL), A");
            let address = cpu.registers.decrement_hl();
            memory.set_byte(address, cpu.registers.a);
        }
        0x33 => {
            debug("INC SP");
            cpu.registers.increment_sp();
        }
        0x38 => {
            debug("JR C, n");
            jr_cc(cpu, memory, cpu.registers.f.carry);
        }
        0x3D => {
            debug("DEC A");
            cpu.registers.a = dec(cpu, cpu.registers.a);
        }
        0xAF => {
            debug("XOR A, A");
            cpu.registers.a = xor(cpu, cpu.registers.a, cpu.registers.a);
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

/// Execute commands in the extended instruction space
fn execute_extended(op: u8, cpu: &mut CPU, memory: &mut MemoryBus) {
    match op {
        0x7C => {
            debug("BIT 7, H");
            test_bit(cpu, cpu.registers.h, 7);
        }
        0x7D => {
            debug("BIT 7, L");
            test_bit(cpu, cpu.registers.l, 7);
        }
        _ => {
            panic!(format!("Unknown operation 0xCB{:X}", op));
        }
    }
}

fn dec(cpu: &mut CPU, value: u8) -> u8 {
    sub(cpu, value, 1)
}

fn sub(cpu: &mut CPU, value: u8, amount: u8) -> u8 {
    let result = value.wrapping_sub(amount);
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = true;
    cpu.registers.f.half_carry = (value & 0x0F) == 0;
    result
}

fn xor(cpu: &mut CPU, left: u8, right: u8) -> u8 {
    let result = left ^ right;
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
    result
}

fn shift_left(cpu: &mut CPU, value: u8) -> u8 {
    let result = value << 1;
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = result < value;
    result
}

fn test_bit(cpu: &mut CPU, value: u8, index: u8) {
    let result = value & (1 << index);
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = true;
}

fn push(cpu: &mut CPU, memory: &mut MemoryBus, address: u16) {
    cpu.registers.decrement_sp();
    cpu.registers.decrement_sp();
    memory.set_word(cpu.registers.sp, address);
}

fn reset(cpu: &mut CPU, memory: &mut MemoryBus, new_pc: u16) {
    push(cpu, memory, cpu.registers.pc);
    cpu.registers.pc = new_pc;
}

fn jr_cc(cpu: &mut CPU, memory: &mut MemoryBus, check: bool) {
    let offset = cpu.get_byte(memory) as i8;
    if check {
        // Effectively subtracts because of wrap
        // eg. 0xeb (i8) becomes 0xffeb (u16)
        cpu.registers.add_pc(offset as u16);
    }
}

fn debug(label: &str) {
    println!("{}", label)
}
