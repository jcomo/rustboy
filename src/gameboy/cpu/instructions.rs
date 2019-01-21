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
        0x04 => {
            debug("INC B");
            let result = inc(cpu, cpu.registers.b);
            cpu.registers.b = result;
        }
        0x05 => {
            debug("DEC B");
            let result = dec(cpu, cpu.registers.b);
            cpu.registers.b = result;
        }
        0x06 => {
            debug("LD B, n");
            let byte = cpu.get_byte(memory);
            cpu.registers.b = byte;
        }
        0x08 => {
            debug("LD (nn), SP");
            let address = cpu.get_word(memory);
            memory.set_word(address, cpu.registers.sp);
        }
        0x0C => {
            debug("INC C");
            let result = inc(cpu, cpu.registers.c);
            cpu.registers.c = result;
        }
        0x0D => {
            debug("DEC C");
            let result = dec(cpu, cpu.registers.c);
            cpu.registers.c = result;
        }
        0x0E => {
            debug("LD C, n");
            let byte = cpu.get_byte(memory);
            cpu.registers.c = byte;
        }
        0x11 => {
            debug("LD DE, nn");
            let word = cpu.get_word(memory);
            cpu.registers.set_de(word);
        }
        0x13 => {
            debug("INC DE");
            cpu.registers.increment_de();
        }
        0x15 => {
            debug("DEC D");
            cpu.registers.d = dec(cpu, cpu.registers.d);
        }
        0x16 => {
            debug("LD D, n");
            let byte = cpu.get_byte(memory);
            cpu.registers.d = byte;
        }
        0x17 => {
            debug("RLA");
            let result = rotate_left_carry(cpu, cpu.registers.a);
            cpu.registers.a = result;
        }
        0x18 => {
            debug("JR n");
            jr_n(cpu, memory);
        }
        0x1A => {
            debug("LD A, (DE)");
            let address = cpu.registers.get_de();
            cpu.registers.a = memory.get_byte(address);
        }
        0x1D => {
            debug("DEC E");
            cpu.registers.e = dec(cpu, cpu.registers.e);
        }
        0x1E => {
            debug("LD E, n");
            let byte = cpu.get_byte(memory);
            cpu.registers.e = byte;
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
        0x24 => {
            debug("INC H");
            cpu.registers.h = inc(cpu, cpu.registers.h);
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
        0x2E => {
            debug("LD L, n");
            let byte = cpu.get_byte(memory);
            cpu.registers.l = byte;
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
        0x3E => {
            debug("LD A, n");
            let byte = cpu.get_byte(memory);
            cpu.registers.a = byte;
        }
        0x4F => {
            debug("LD C, A");
            cpu.registers.c = cpu.registers.a;
        }
        0x57 => {
            debug("LD D, A");
            cpu.registers.d = cpu.registers.a;
        }
        0x67 => {
            debug("LD H, A");
            cpu.registers.h = cpu.registers.a;
        }
        0x77 => {
            debug("LD (HL), A");
            let address = cpu.registers.get_hl();
            memory.set_byte(address, cpu.registers.a);
        }
        0x7B => {
            debug("LD A, E");
            cpu.registers.a = cpu.registers.e;
        }
        0x7C => {
            debug("LD A, H");
            cpu.registers.a = cpu.registers.h;
        }
        0x90 => {
            debug("SUB B");
            cpu.registers.a = sub(cpu, cpu.registers.a, cpu.registers.b);
        }
        0xAF => {
            debug("XOR A, A");
            let value = xor(cpu, cpu.registers.a, cpu.registers.a);
            cpu.registers.a = value;
        }
        0xC1 => {
            debug("POP BC");
            let word = pop(cpu, memory);
            cpu.registers.set_bc(word);
        }
        0xC5 => {
            debug("PUSH BC");
            let address = cpu.registers.get_bc();
            push(cpu, memory, address);
        }
        0xC9 => {
            debug("RET");
            ret(cpu, memory);
        }
        0xCD => {
            debug("CALL nn");
            let address = cpu.get_word(memory);
            call(cpu, memory, address);
        }
        0xE0 => {
            debug("LDH (n), A");
            let offset = cpu.get_byte(memory);
            let address = bits::to_word(0xFF, offset);
            memory.set_byte(address, cpu.registers.a);
        }
        0xE2 => {
            debug("LD (C), A");
            let address = bits::to_word(0xFF, cpu.registers.c);
            memory.set_byte(address, cpu.registers.a);
        }
        0xEA => {
            debug("LD (nn), A");
            let address = cpu.get_word(memory);
            memory.set_byte(address, cpu.registers.a);
        }
        0xF0 => {
            debug("LDH A, (n)");
            let offset = cpu.get_byte(memory);
            let address = bits::to_word(0xFF, offset);
            cpu.registers.a = memory.get_byte(address);
        }
        0xFE => {
            debug("CP n");
            let value = cpu.get_byte(memory);
            sub(cpu, cpu.registers.a, value);
        }
        0xFF => {
            debug("RST 38H");
            reset(cpu, memory, 0x38);
        }
        _ => {
            println!("{:?}", cpu.registers);
            panic!(format!("Unknown operation 0x{:X}", op));
        }
    }
}

/// Execute commands in the extended instruction space
fn execute_extended(op: u8, cpu: &mut CPU, memory: &mut MemoryBus) {
    match op {
        0x11 => {
            debug("RL C");
            let result = rotate_left_carry(cpu, cpu.registers.c);
            cpu.registers.c = result;
        }
        0x7C => {
            debug("BIT 7, H");
            test_bit(cpu, cpu.registers.h, 7);
        }
        0x7D => {
            debug("BIT 7, L");
            test_bit(cpu, cpu.registers.l, 7);
        }
        _ => {
            println!("{:?}", cpu.registers);
            panic!(format!("Unknown operation 0xCB{:X}", op));
        }
    }
}

fn inc(cpu: &mut CPU, value: u8) -> u8 {
    let result = value.wrapping_add(1);
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = (value & 0x0F) == 0;
    result
}

fn dec(cpu: &mut CPU, value: u8) -> u8 {
    let result = value.wrapping_sub(1);
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = true;
    cpu.registers.f.half_carry = (value & 0x0f) == 0;
    result
}

fn add(cpu: &mut CPU, left: u8, right: u8) -> u8 {
    let (result, carry) = left.overflowing_add(right);
    let half_carry = (left & 0xF) + (right & 0xF) > 0xF;
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = half_carry;
    cpu.registers.f.carry = carry;
    result
}

fn sub(cpu: &mut CPU, left: u8, right: u8) -> u8 {
    let result = left.wrapping_sub(right);
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = true;
    cpu.registers.f.half_carry = (right & 0xF) > (left & 0xF);
    cpu.registers.f.carry = right > left;
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

fn rotate_left_carry(cpu: &mut CPU, value: u8) -> u8 {
    let carry = bits::from_bool(cpu.registers.f.carry);
    let result = (value << 1) | carry;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = (value >> 7) != 0;
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

fn pop(cpu: &mut CPU, memory: &mut MemoryBus) -> u16 {
    let result = memory.get_word(cpu.registers.sp);
    cpu.registers.increment_sp();
    cpu.registers.increment_sp();
    result
}

fn reset(cpu: &mut CPU, memory: &mut MemoryBus, new_pc: u16) {
    push(cpu, memory, cpu.registers.pc);
    cpu.registers.pc = new_pc;
}

fn call(cpu: &mut CPU, memory: &mut MemoryBus, address: u16) {
    push(cpu, memory, cpu.registers.pc);
    cpu.registers.pc = address;
}

fn ret(cpu: &mut CPU, memory: &mut MemoryBus) {
    cpu.registers.pc = pop(cpu, memory);
}

fn jr_n(cpu: &mut CPU, memory: &mut MemoryBus) {
    jr_cc(cpu, memory, true);
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
    // println!("{}", label)
}
