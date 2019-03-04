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
        0x01 => {
            debug("LD BC, nn");
            let word = cpu.get_word(memory);
            cpu.registers.set_bc(word);
        }
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
        0x0A => {
            debug("LD A, (BC)");
            let value = memory.get_byte(cpu.registers.get_bc());
            cpu.registers.a = value;
        }
        0x0B => {
            debug("DEC BC");
            cpu.registers.decrement_bc();
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
        0x12 => {
            debug("LD (DE), A");
            let address = cpu.registers.get_de();
            memory.set_byte(address, cpu.registers.a);
        }
        0x13 => {
            debug("INC DE");
            cpu.registers.increment_de();
        }
        0x14 => {
            debug("INC D");
            let result = inc(cpu, cpu.registers.d);
            cpu.registers.d = result;
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
        0x19 => {
            debug("ADD HL, DE");
            let result = add_16(cpu, cpu.registers.get_hl(), cpu.registers.get_de());
            cpu.registers.set_hl(result);
        }
        0x1A => {
            debug("LD A, (DE)");
            let address = cpu.registers.get_de();
            cpu.registers.a = memory.get_byte(address);
        }
        0x1C => {
            debug("INC E");
            cpu.registers.e = inc(cpu, cpu.registers.e);
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
        0x1F => {
            debug("RRA");
            let result = rotate_right_carry(cpu, cpu.registers.a);
            cpu.registers.a = result;
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
        0x26 => {
            debug("LD H, n");
            cpu.registers.h = cpu.get_byte(memory);
        }
        0x28 => {
            debug("JR Z, n");
            jr_cc(cpu, memory, cpu.registers.f.zero);
        }
        0x29 => {
            debug("ADD HL, HL");
            let result = add_16(cpu, cpu.registers.get_hl(), cpu.registers.get_hl());
            cpu.registers.set_hl(result);
        }
        0x2A => {
            debug("LDI A, (HL)");
            cpu.registers.a = memory.get_byte(cpu.registers.get_hl());
            cpu.registers.increment_hl();
        }
        0x2C => {
            debug("INC L");
            cpu.registers.l = inc(cpu, cpu.registers.l);
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
        0x2F => {
            debug("CPL");
            cpu.registers.a = complement(cpu, cpu.registers.a);
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
        0x34 => {
            debug("INC (HL)");
            let address = cpu.registers.get_hl();
            let byte = memory.get_byte(address);
            memory.set_byte(address, inc(cpu, byte));
        }
        0x35 => {
            debug("DEC (HL)");
            let address = cpu.registers.get_hl();
            let byte = memory.get_byte(address);
            memory.set_byte(address, dec(cpu, byte));
        }
        0x36 => {
            debug("LD (HL), n");
            let byte = cpu.get_byte(memory);
            let address = cpu.registers.get_hl();
            memory.set_byte(address, byte);
        }
        0x38 => {
            debug("JR C, n");
            jr_cc(cpu, memory, cpu.registers.f.carry);
        }
        0x39 => {
            debug("ADD HL, SP");
            let result = add_16(cpu, cpu.registers.get_hl(), cpu.registers.sp);
            cpu.registers.set_hl(result);
        }
        0x3B => {
            debug("DEC SP");
            cpu.registers.decrement_sp();
        }
        0x3C => {
            debug("INC A");
            let result = inc(cpu, cpu.registers.a);
            cpu.registers.a = result;
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
        0x46 => {
            debug("LD B, (HL)");
            let value = memory.get_byte(cpu.registers.get_hl());
            cpu.registers.b = value;
        }
        0x47 => {
            debug("LD B, A");
            cpu.registers.b = cpu.registers.a;
        }
        0x4E => {
            debug("LD C, (HL)");
            let value = memory.get_byte(cpu.registers.get_hl());
            cpu.registers.c = value;
        }
        0x4F => {
            debug("LD C, A");
            cpu.registers.c = cpu.registers.a;
        }
        0x56 => {
            debug("LD D, (HL)");
            let byte = memory.get_byte(cpu.registers.get_hl());
            cpu.registers.d = byte;
        }
        0x57 => {
            debug("LD D, A");
            cpu.registers.d = cpu.registers.a;
        }
        0x5D => {
            debug("LD E, L");
            cpu.registers.e = cpu.registers.l;
        }
        0x5E => {
            debug("LD E, (HL)");
            let byte = memory.get_byte(cpu.registers.get_hl());
            cpu.registers.e = byte;
        }
        0x5F => {
            debug("LD E, A");
            cpu.registers.e = cpu.registers.a;
        }
        0x62 => {
            debug("LD H, D");
            cpu.registers.h = cpu.registers.d;
        }
        0x63 => {
            debug("LD H, E");
            cpu.registers.h = cpu.registers.e;
        }
        0x64 => {
            debug("LD H, H");
            cpu.registers.h = cpu.registers.h;
        }
        0x65 => {
            debug("LD H, L");
            cpu.registers.h = cpu.registers.l;
        }
        0x66 => {
            debug("LD H, (HL)");
            let byte = memory.get_byte(cpu.registers.get_hl());
            cpu.registers.h = byte;
        }
        0x67 => {
            debug("LD H, A");
            cpu.registers.h = cpu.registers.a;
        }
        0x68 => {
            debug("LD L, B");
            cpu.registers.l = cpu.registers.b;
        }
        0x69 => {
            debug("LD L, C");
            cpu.registers.l = cpu.registers.c;
        }
        0x6A => {
            debug("LD L, D");
            cpu.registers.l = cpu.registers.d;
        }
        0x6B => {
            debug("LD L, E");
            cpu.registers.l = cpu.registers.e;
        }
        0x6C => {
            debug("LD L, H");
            cpu.registers.l = cpu.registers.h;
        }
        0x6D => {
            debug("LD L, L");
            cpu.registers.l = cpu.registers.l;
        }
        0x6E => {
            debug("LD L, (HL)");
            let byte = memory.get_byte(cpu.registers.get_hl());
            cpu.registers.l = byte;
        }
        0x6F => {
            debug("LD L, A");
            cpu.registers.l = cpu.registers.a;
        }
        0x70 => {
            debug("LD (HL), B");
            let address = cpu.registers.get_hl();
            memory.set_byte(address, cpu.registers.b);
        }
        0x71 => {
            debug("LD (HL), C");
            let address = cpu.registers.get_hl();
            memory.set_byte(address, cpu.registers.c);
        }
        0x72 => {
            debug("LD (HL), D");
            let address = cpu.registers.get_hl();
            memory.set_byte(address, cpu.registers.d);
        }
        0x73 => {
            debug("LD (HL), E");
            let address = cpu.registers.get_hl();
            memory.set_byte(address, cpu.registers.e);
        }
        0x74 => {
            debug("LD (HL), H");
            let address = cpu.registers.get_hl();
            memory.set_byte(address, cpu.registers.h);
        }
        0x75 => {
            debug("LD (HL), L");
            let address = cpu.registers.get_hl();
            memory.set_byte(address, cpu.registers.l);
        }
        0x77 => {
            debug("LD (HL), A");
            let address = cpu.registers.get_hl();
            memory.set_byte(address, cpu.registers.a);
        }
        0x78 => {
            debug("LD A, B");
            cpu.registers.a = cpu.registers.b;
        }
        0x79 => {
            debug("LD A, C");
            cpu.registers.a = cpu.registers.c;
        }
        0x7A => {
            debug("LD A, D");
            cpu.registers.a = cpu.registers.d;
        }
        0x7B => {
            debug("LD A, E");
            cpu.registers.a = cpu.registers.e;
        }
        0x7C => {
            debug("LD A, H");
            cpu.registers.a = cpu.registers.h;
        }
        0x7D => {
            debug("LD A, L");
            cpu.registers.a = cpu.registers.l;
        }
        0x7E => {
            debug("LD A, (HL)");
            let byte = memory.get_byte(cpu.registers.get_hl());
            cpu.registers.a = byte;
        }
        0x81 => {
            debug("ADD A, C");
            cpu.registers.a = add(cpu, cpu.registers.a, cpu.registers.c);
        }
        0x86 => {
            debug("ADD A, (HL)");
            let byte = memory.get_byte(cpu.registers.get_hl());
            cpu.registers.a = add(cpu, cpu.registers.a, byte);
        }
        0x87 => {
            debug("ADD A, A");
            cpu.registers.a = add(cpu, cpu.registers.a, cpu.registers.a);
        }
        0x90 => {
            debug("SUB B");
            cpu.registers.a = sub(cpu, cpu.registers.a, cpu.registers.b);
        }
        0xA1 => {
            debug("AND C");
            let value = and(cpu, cpu.registers.a, cpu.registers.c);
            cpu.registers.a = value;
        }
        0xA7 => {
            debug("AND A");
            let value = and(cpu, cpu.registers.a, cpu.registers.a);
            cpu.registers.a = value;
        }
        0xA8 => {
            debug("XOR B");
            let value = xor(cpu, cpu.registers.a, cpu.registers.b);
            cpu.registers.a = value;
        }
        0xA9 => {
            debug("XOR C");
            let value = xor(cpu, cpu.registers.a, cpu.registers.c);
            cpu.registers.a = value;
        }
        0xAA => {
            debug("XOR D");
            let value = xor(cpu, cpu.registers.a, cpu.registers.d);
            cpu.registers.a = value;
        }
        0xAB => {
            debug("XOR E");
            let value = xor(cpu, cpu.registers.a, cpu.registers.e);
            cpu.registers.a = value;
        }
        0xAC => {
            debug("XOR H");
            let value = xor(cpu, cpu.registers.a, cpu.registers.h);
            cpu.registers.a = value;
        }
        0xAD => {
            debug("XOR L");
            let value = xor(cpu, cpu.registers.a, cpu.registers.l);
            cpu.registers.a = value;
        }
        0xAE => {
            debug("XOR (HL)");
            let value = memory.get_byte(cpu.registers.get_hl());
            cpu.registers.a = xor(cpu, cpu.registers.a, value);
        }
        0xAF => {
            debug("XOR A");
            let value = xor(cpu, cpu.registers.a, cpu.registers.a);
            cpu.registers.a = value;
        }
        0xB0 => {
            debug("OR B");
            let value = or(cpu, cpu.registers.a, cpu.registers.b);
            cpu.registers.a = value;
        }
        0xB1 => {
            debug("OR C");
            let value = or(cpu, cpu.registers.a, cpu.registers.c);
            cpu.registers.a = value;
        }
        0xB6 => {
            debug("OR (HL)");
            let value = memory.get_byte(cpu.registers.get_hl());
            cpu.registers.a = or(cpu, cpu.registers.a, value);
        }
        0xB7 => {
            debug("OR A");
            let value = or(cpu, cpu.registers.a, cpu.registers.a);
            cpu.registers.a = value;
        }
        0xBC => {
            debug("CP H");
            sub(cpu, cpu.registers.a, cpu.registers.h);
        }
        0xBE => {
            debug("CP (HL)");
            let byte = memory.get_byte(cpu.registers.get_hl());
            sub(cpu, cpu.registers.a, byte);
        }
        0xC0 => {
            debug("RET NZ");
            ret_cc(cpu, memory, !cpu.registers.f.zero);
        }
        0xC1 => {
            debug("POP BC");
            let address = pop(cpu, memory);
            cpu.registers.set_bc(address);
        }
        0xC2 => {
            debug("JP NZ, nn");
            let address = cpu.get_word(memory);
            jp_cc(cpu, memory, !cpu.registers.f.zero);
        }
        0xC3 => {
            debug("JP nn");
            jp_n(cpu, memory);
        }
        0xC4 => {
            debug("CALL NZ, nn");
            call_cc(cpu, memory, !cpu.registers.f.zero);
        }
        0xC5 => {
            debug("PUSH BC");
            let address = cpu.registers.get_bc();
            push(cpu, memory, address);
        }
        0xC6 => {
            debug("ADD A, n");
            let value = cpu.get_byte(memory);
            cpu.registers.a = add(cpu, cpu.registers.a, value);
        }
        0xC8 => {
            debug("RET Z");
            ret_cc(cpu, memory, cpu.registers.f.zero);
        }
        0xC9 => {
            debug("RET");
            ret(cpu, memory);
        }
        0xCA => {
            debug("JP Z, nn");
            jp_cc(cpu, memory, cpu.registers.f.zero);
        }
        0xCD => {
            debug("CALL nn");
            call(cpu, memory);
        }
        0xCE => {
            debug("ADC A, n");
            let value = cpu.get_byte(memory);
            cpu.registers.a = add_carry(cpu, cpu.registers.a, value);
        }
        0xD0 => {
            debug("RET NC");
            ret_cc(cpu, memory, !cpu.registers.f.carry);
        }
        0xD1 => {
            debug("POP DE");
            let address = pop(cpu, memory);
            cpu.registers.set_de(address);
        }
        0xD5 => {
            debug("PUSH DE");
            push(cpu, memory, cpu.registers.get_de());
        }
        0xD6 => {
            debug("SUB n");
            let value = cpu.get_byte(memory);
            cpu.registers.a = sub(cpu, cpu.registers.a, value);
        }
        0xD8 => {
            debug("RET C");
            ret_cc(cpu, memory, cpu.registers.f.carry);
        }
        0xD9 => {
            debug("RETI");
            cpu.set_ime();
            ret(cpu, memory);
        }
        0xDE => {
            debug("SBC A, n");
            let byte = cpu.get_byte(memory);
            cpu.registers.a = sub_carry(cpu, cpu.registers.a, byte);
        }
        0xE0 => {
            debug("LDH (n), A");
            let offset = cpu.get_byte(memory);
            let address = bits::to_word(0xFF, offset);
            memory.set_byte(address, cpu.registers.a);
        }
        0xE1 => {
            debug("POP HL");
            let address = pop(cpu, memory);
            cpu.registers.set_hl(address);
        }
        0xE2 => {
            debug("LD (C), A");
            let address = bits::to_word(0xFF, cpu.registers.c);
            memory.set_byte(address, cpu.registers.a);
        }
        0xE5 => {
            debug("PUSH HL");
            push(cpu, memory, cpu.registers.get_hl());
        }
        0xE6 => {
            debug("AND n");
            let byte = cpu.get_byte(memory);
            cpu.registers.a = and(cpu, cpu.registers.a, byte);
        }
        0xE8 => {
            debug("ADD SP, n");
            let byte = cpu.get_byte(memory);
            cpu.registers.sp = add_sp(cpu, byte);
        }
        0xE9 => {
            debug("JP HL");
            let address = cpu.registers.get_hl();
            cpu.registers.pc = address;
        }
        0xEA => {
            debug("LD (nn), A");
            let address = cpu.get_word(memory);
            memory.set_byte(address, cpu.registers.a);
        }
        0xEE => {
            debug("XOR n");
            let value = cpu.get_byte(memory);
            cpu.registers.a = xor(cpu, cpu.registers.a, value);
        }
        0xF0 => {
            debug("LDH A, n");
            let offset = cpu.get_byte(memory);
            let address = bits::to_word(0xFF, offset);
            cpu.registers.a = memory.get_byte(address);
        }
        0xF1 => {
            debug("POP AF");
            let address = pop(cpu, memory);
            cpu.registers.set_af(address);
        }
        0xF3 => {
            debug("DI");
            cpu.reset_ime();
        }
        0xF5 => {
            debug("PUSH AF");
            push(cpu, memory, cpu.registers.get_af());
        }
        0xF6 => {
            debug("OR n");
            let byte = cpu.get_byte(memory);
            let value = or(cpu, cpu.registers.a, byte);
            cpu.registers.a = value;
        }
        0xF8 => {
            debug("LDHL SP, n");
            let byte = cpu.get_byte(memory);
            let result = add_sp(cpu, byte);
            cpu.registers.set_hl(result);
        }
        0xF9 => {
            debug("LD SP, HL");
            cpu.registers.sp = cpu.registers.get_hl();
        }
        0xFA => {
            debug("LD A, (nn)");
            let address = cpu.get_word(memory);
            cpu.registers.a = memory.get_byte(address);
        }
        0xFB => {
            debug("EI");
            cpu.set_ime_delayed();
        }
        0xFE => {
            debug("CP n");
            let value = cpu.get_byte(memory);
            sub(cpu, cpu.registers.a, value);
        }
        0xEF => {
            debug("RST 28H");
            reset(cpu, memory, 0x28);
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
        0x19 => {
            debug("RR C");
            let result = rotate_right_carry(cpu, cpu.registers.c);
            cpu.registers.c = result;
        }
        0x1A => {
            debug("RR D");
            let result = rotate_right_carry(cpu, cpu.registers.d);
            cpu.registers.d = result;
        }
        0x1B => {
            debug("RR E");
            let result = rotate_right_carry(cpu, cpu.registers.e);
            cpu.registers.e = result;
        }
        0x37 => {
            debug("SWAP A");
            cpu.registers.a = swap(cpu, cpu.registers.a);
        }
        0x38 => {
            debug("SRL B");
            // TODO
        }
        0x7C => {
            debug("BIT 7, H");
            test_bit(cpu, cpu.registers.h, 7);
        }
        0x7D => {
            debug("BIT 7, L");
            test_bit(cpu, cpu.registers.l, 7);
        }
        0x87 => {
            debug("RES 0, A");
            cpu.registers.a = bits::reset(cpu.registers.a, 0);
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
    let half_carry = (left & 0x0F) + (right & 0x0F) > 0x0F;
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = half_carry;
    cpu.registers.f.carry = carry;
    result
}

fn add_carry(cpu: &mut CPU, left: u8, right: u8) -> u8 {
    let carry_value = bits::from_bool(cpu.registers.f.carry);
    let result = left.wrapping_add(right).wrapping_add(carry_value);

    let carry = (left as u16 + right as u16 + carry_value as u16) > 0xFF;
    let half_carry = ((left & 0x0F) + (right & 0x0F) + carry_value) > 0x0F;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = half_carry;
    cpu.registers.f.carry = carry;
    result
}

fn add_16(cpu: &mut CPU, left: u16, right: u16) -> u16 {
    let (result, carry) = left.overflowing_add(right);
    let half_carry = (left & 0x0FFF) + (right & 0x0FFF) > 0x0FFF;
    cpu.registers.f.subtract = false;
    cpu.registers.f.carry = carry;
    cpu.registers.f.half_carry = half_carry;
    result
}

fn add_sp(cpu: &mut CPU, byte: u8) -> u16 {
    // TODO: go over this implementation
    let sp = cpu.registers.sp;
    let value = byte as i8 as i16 as u16;
    let result = sp.wrapping_add(value);

    let carry = (sp & 0xFF) + (value & 0xFF) > 0xFF;
    let half_carry = (sp & 0x0F) + (value & 0x0F) > 0x0F;

    cpu.registers.f.zero = false;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = half_carry;
    cpu.registers.f.carry = carry;
    result
}

fn sub(cpu: &mut CPU, left: u8, right: u8) -> u8 {
    let result = left.wrapping_sub(right);
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = true;
    cpu.registers.f.half_carry = (right & 0x0F) > (left & 0x0F);
    cpu.registers.f.carry = right > left;
    result
}

fn sub_carry(cpu: &mut CPU, left: u8, right: u8) -> u8 {
    let carry_value = bits::from_bool(cpu.registers.f.carry);
    let result = left.wrapping_sub(right).wrapping_sub(carry_value);

    let carry = (right as u16 + carry_value as u16) > left as u16;
    let half_carry = ((right & 0x0F) + carry_value) > (left & 0x0F);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = true;
    cpu.registers.f.half_carry = half_carry;
    cpu.registers.f.carry = carry;
    result
}

fn and(cpu: &mut CPU, left: u8, right: u8) -> u8 {
    let result = left & right;
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = true;
    cpu.registers.f.carry = false;
    result
}

fn or(cpu: &mut CPU, left: u8, right: u8) -> u8 {
    let result = left | right;
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
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

fn complement(cpu: &mut CPU, value: u8) -> u8 {
    let result = value ^ 0xFF;
    cpu.registers.f.subtract = true;
    cpu.registers.f.half_carry = true;
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

fn rotate_right(cpu: &mut CPU, value: u8) -> u8 {
    let result = value.rotate_right(1);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = bits::to_bool(value & 0x1);
    result
}

fn rotate_right_carry(cpu: &mut CPU, value: u8) -> u8 {
    let carry = bits::from_bool(cpu.registers.f.carry);
    let result = (carry << 7) | (value >> 1);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = bits::to_bool(value & 0x1);
    result
}

fn rotate_left(cpu: &mut CPU, value: u8) -> u8 {
    let result = value.rotate_left(1);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = bits::to_bool(value >> 7);
    result
}

fn rotate_left_carry(cpu: &mut CPU, value: u8) -> u8 {
    let carry = bits::from_bool(cpu.registers.f.carry);
    let result = (value << 1) | carry;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = bits::to_bool(value >> 7);
    result
}

fn test_bit(cpu: &mut CPU, value: u8, index: u8) {
    let result = value & (1 << index);
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = true;
}

fn swap(cpu: &mut CPU, value: u8) -> u8 {
    let high_nibble = (value & 0xF0) >> 4;
    let low_nibble = (value & 0x0F) << 4;
    let result = low_nibble | high_nibble;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;

    result
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

fn call(cpu: &mut CPU, memory: &mut MemoryBus) {
    call_cc(cpu, memory, true);
}

fn call_cc(cpu: &mut CPU, memory: &mut MemoryBus, check: bool) {
    // Must get before checking to advance cycles
    let address = cpu.get_word(memory);

    if check {
        push(cpu, memory, cpu.registers.pc);
        cpu.registers.pc = address;
    }
}

fn ret(cpu: &mut CPU, memory: &mut MemoryBus) {
    ret_cc(cpu, memory, true);
}

fn ret_cc(cpu: &mut CPU, memory: &mut MemoryBus, check: bool) {
    if check {
        cpu.registers.pc = pop(cpu, memory);
    }
}

fn jp_n(cpu: &mut CPU, memory: &mut MemoryBus) {
    jp_cc(cpu, memory, true);
}

fn jp_cc(cpu: &mut CPU, memory: &mut MemoryBus, check: bool) {
    // Must get before checking to advance cycles
    let address = cpu.get_word(memory);

    if check {
        cpu.registers.pc = address;
    }
}

fn jr_n(cpu: &mut CPU, memory: &mut MemoryBus) {
    jr_cc(cpu, memory, true);
}

fn jr_cc(cpu: &mut CPU, memory: &mut MemoryBus, check: bool) {
    // Must get before checking to advance cycles
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
