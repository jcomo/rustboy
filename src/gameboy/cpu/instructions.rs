use crate::bits;

use super::MemoryBus;
use super::CPU;

enum Loc8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    Imm8,
    AddrC,
    AddrBC,
    AddrDE,
    AddrHL,
    AddrImm16,
}

impl Loc8 {
    fn read(&self, cpu: &mut CPU, memory: &mut MemoryBus) -> u8 {
        use self::Loc8::*;

        match self {
            A => cpu.registers.a,
            B => cpu.registers.b,
            C => cpu.registers.c,
            D => cpu.registers.d,
            E => cpu.registers.e,
            H => cpu.registers.h,
            L => cpu.registers.l,
            Imm8 => cpu.get_byte(memory),
            AddrC => memory.get_byte(bits::to_word(0xFF, cpu.registers.c)),
            AddrBC => memory.get_byte(cpu.registers.get_bc()),
            AddrDE => memory.get_byte(cpu.registers.get_de()),
            AddrHL => memory.get_byte(cpu.registers.get_hl()),
            AddrImm16 => {
                let address = cpu.get_word(memory);
                memory.get_byte(address)
            }
        }
    }

    fn write(&self, cpu: &mut CPU, memory: &mut MemoryBus, value: u8) {
        use self::Loc8::*;

        match self {
            A => cpu.registers.a = value,
            B => cpu.registers.b = value,
            C => cpu.registers.c = value,
            D => cpu.registers.d = value,
            E => cpu.registers.e = value,
            H => cpu.registers.h = value,
            L => cpu.registers.l = value,
            Imm8 => panic!("Cannot write to Imm8"),
            AddrC => memory.set_byte(bits::to_word(0xFF, cpu.registers.c), value),
            AddrBC => memory.set_byte(cpu.registers.get_bc(), value),
            AddrDE => memory.set_byte(cpu.registers.get_de(), value),
            AddrHL => memory.set_byte(cpu.registers.get_hl(), value),
            AddrImm16 => {
                let address = cpu.get_word(memory);
                memory.set_byte(address, value)
            }
        }
    }
}

enum Loc16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
    Imm16,
}

impl Loc16 {
    fn read(&self, cpu: &mut CPU, memory: &mut MemoryBus) -> u16 {
        use self::Loc16::*;

        match self {
            AF => cpu.registers.get_af(),
            BC => cpu.registers.get_bc(),
            DE => cpu.registers.get_de(),
            HL => cpu.registers.get_hl(),
            SP => cpu.registers.sp,
            PC => cpu.registers.pc,
            Imm16 => cpu.get_word(memory),
        }
    }

    fn write(&self, cpu: &mut CPU, memory: &mut MemoryBus, value: u16) {
        use self::Loc16::*;

        match self {
            AF => cpu.registers.set_af(value),
            BC => cpu.registers.set_bc(value),
            DE => cpu.registers.set_de(value),
            HL => cpu.registers.set_hl(value),
            SP => cpu.registers.sp = value,
            PC => cpu.registers.pc = value,
            Imm16 => {
                let address = cpu.get_word(memory);
                memory.set_word(address, value)
            }
        }
    }
}

enum Check {
    C,
    Z,
    NC,
    NZ,
    True,
}

impl Check {
    fn evaluate(&self, cpu: &CPU) -> bool {
        use self::Check::*;

        match self {
            C => cpu.registers.f.carry,
            Z => cpu.registers.f.zero,
            NC => !cpu.registers.f.carry,
            NZ => !cpu.registers.f.zero,
            True => true,
        }
    }
}

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
    use self::Loc16::*;
    use self::Loc8::*;

    match op {
        0x00 => { /* NOP */ }
        0x01 => load_16(cpu, memory, BC, Imm16),
        0x02 => load_8(cpu, memory, AddrBC, A),
        0x03 => inc_16(cpu, memory, BC),
        0x04 => inc_8(cpu, memory, B),
        0x05 => dec_8(cpu, memory, B),
        0x06 => load_8(cpu, memory, B, Imm8),
        0x07 => rlc(cpu, memory, A),
        0x08 => load_16(cpu, memory, Imm16, SP),
        0x09 => add_16(cpu, memory, BC),
        0x0A => load_8(cpu, memory, A, AddrBC),
        0x0B => dec_16(cpu, memory, BC),
        0x0C => inc_8(cpu, memory, C),
        0x0D => dec_8(cpu, memory, C),
        0x0E => load_8(cpu, memory, C, Imm8),
        0x0F => rrc(cpu, memory, A),
        0x11 => load_16(cpu, memory, DE, Imm16),
        0x12 => load_8(cpu, memory, AddrDE, A),
        0x13 => inc_16(cpu, memory, DE),
        0x14 => inc_8(cpu, memory, D),
        0x15 => dec_8(cpu, memory, D),
        0x16 => load_8(cpu, memory, D, Imm8),
        0x17 => rl(cpu, memory, A),
        0x18 => jr_n(cpu, memory, Check::True),
        0x19 => add_16(cpu, memory, DE),
        0x1A => load_8(cpu, memory, A, AddrDE),
        0x1B => dec_16(cpu, memory, DE),
        0x1C => inc_8(cpu, memory, E),
        0x1D => dec_8(cpu, memory, E),
        0x1E => load_8(cpu, memory, E, Imm8),
        0x1F => rr(cpu, memory, A),
        0x20 => jr_n(cpu, memory, Check::NZ),
        0x21 => load_16(cpu, memory, HL, Imm16),
        0x22 => ldi(cpu, memory, AddrHL, A),
        0x23 => inc_16(cpu, memory, HL),
        0x24 => inc_8(cpu, memory, H),
        0x25 => dec_8(cpu, memory, H),
        0x26 => load_8(cpu, memory, H, Imm8),
        0x27 => daa(cpu, memory),
        0x28 => jr_n(cpu, memory, Check::Z),
        0x29 => add_16(cpu, memory, HL),
        0x2A => ldi(cpu, memory, A, AddrHL),
        0x2B => dec_16(cpu, memory, HL),
        0x2C => inc_8(cpu, memory, L),
        0x2D => dec_8(cpu, memory, L),
        0x2E => load_8(cpu, memory, L, Imm8),
        0x2F => cpl(cpu),
        0x30 => jr_n(cpu, memory, Check::NC),
        0x31 => load_16(cpu, memory, SP, Imm16),
        0x32 => ldd(cpu, memory, AddrHL, A),
        0x33 => inc_16(cpu, memory, SP),
        0x34 => inc_8(cpu, memory, AddrHL),
        0x35 => dec_8(cpu, memory, AddrHL),
        0x36 => load_8(cpu, memory, AddrHL, Imm8),
        0x37 => scf(cpu),
        0x38 => jr_n(cpu, memory, Check::C),
        0x39 => add_16(cpu, memory, SP),
        0x3A => ldd(cpu, memory, A, AddrHL),
        0x3B => dec_16(cpu, memory, SP),
        0x3C => inc_8(cpu, memory, A),
        0x3D => dec_8(cpu, memory, A),
        0x3E => load_8(cpu, memory, A, Imm8),
        0x3F => ccf(cpu),
        0x40 => load_8(cpu, memory, B, B),
        0x41 => load_8(cpu, memory, B, C),
        0x42 => load_8(cpu, memory, B, D),
        0x43 => load_8(cpu, memory, B, E),
        0x44 => load_8(cpu, memory, B, H),
        0x45 => load_8(cpu, memory, B, L),
        0x46 => load_8(cpu, memory, B, AddrHL),
        0x47 => load_8(cpu, memory, B, A),
        0x48 => load_8(cpu, memory, C, B),
        0x49 => load_8(cpu, memory, C, C),
        0x4A => load_8(cpu, memory, C, D),
        0x4B => load_8(cpu, memory, C, E),
        0x4C => load_8(cpu, memory, C, H),
        0x4D => load_8(cpu, memory, C, L),
        0x4E => load_8(cpu, memory, C, AddrHL),
        0x4F => load_8(cpu, memory, C, A),
        0x50 => load_8(cpu, memory, D, B),
        0x51 => load_8(cpu, memory, D, C),
        0x52 => load_8(cpu, memory, D, D),
        0x53 => load_8(cpu, memory, D, E),
        0x54 => load_8(cpu, memory, D, H),
        0x55 => load_8(cpu, memory, D, L),
        0x56 => load_8(cpu, memory, D, AddrHL),
        0x57 => load_8(cpu, memory, D, A),
        0x58 => load_8(cpu, memory, E, B),
        0x59 => load_8(cpu, memory, E, C),
        0x5A => load_8(cpu, memory, E, D),
        0x5B => load_8(cpu, memory, E, E),
        0x5C => load_8(cpu, memory, E, H),
        0x5D => load_8(cpu, memory, E, L),
        0x5E => load_8(cpu, memory, E, AddrHL),
        0x5F => load_8(cpu, memory, E, A),
        0x60 => load_8(cpu, memory, H, B),
        0x61 => load_8(cpu, memory, H, C),
        0x62 => load_8(cpu, memory, H, D),
        0x63 => load_8(cpu, memory, H, E),
        0x64 => load_8(cpu, memory, H, H),
        0x65 => load_8(cpu, memory, H, L),
        0x66 => load_8(cpu, memory, H, AddrHL),
        0x67 => load_8(cpu, memory, H, A),
        0x68 => load_8(cpu, memory, L, B),
        0x69 => load_8(cpu, memory, L, C),
        0x6A => load_8(cpu, memory, L, D),
        0x6B => load_8(cpu, memory, L, E),
        0x6C => load_8(cpu, memory, L, H),
        0x6D => load_8(cpu, memory, L, L),
        0x6E => load_8(cpu, memory, L, AddrHL),
        0x6F => load_8(cpu, memory, L, A),
        0x70 => load_8(cpu, memory, AddrHL, B),
        0x71 => load_8(cpu, memory, AddrHL, C),
        0x72 => load_8(cpu, memory, AddrHL, D),
        0x73 => load_8(cpu, memory, AddrHL, E),
        0x74 => load_8(cpu, memory, AddrHL, H),
        0x75 => load_8(cpu, memory, AddrHL, L),
        0x76 => { /* TODO: HALT */ }
        0x77 => load_8(cpu, memory, AddrHL, A),
        0x78 => load_8(cpu, memory, A, B),
        0x79 => load_8(cpu, memory, A, C),
        0x7A => load_8(cpu, memory, A, D),
        0x7B => load_8(cpu, memory, A, E),
        0x7C => load_8(cpu, memory, A, H),
        0x7D => load_8(cpu, memory, A, L),
        0x7E => load_8(cpu, memory, A, AddrHL),
        0x7F => load_8(cpu, memory, A, A),
        0x80 => add_8(cpu, memory, B),
        0x81 => add_8(cpu, memory, C),
        0x82 => add_8(cpu, memory, D),
        0x83 => add_8(cpu, memory, E),
        0x84 => add_8(cpu, memory, H),
        0x85 => add_8(cpu, memory, L),
        0x86 => add_8(cpu, memory, AddrHL),
        0x87 => add_8(cpu, memory, A),
        0x88 => adc(cpu, memory, B),
        0x89 => adc(cpu, memory, C),
        0x8A => adc(cpu, memory, D),
        0x8B => adc(cpu, memory, E),
        0x8C => adc(cpu, memory, H),
        0x8D => adc(cpu, memory, L),
        0x8E => adc(cpu, memory, AddrHL),
        0x8F => adc(cpu, memory, A),
        0x90 => sub(cpu, memory, B),
        0x91 => sub(cpu, memory, C),
        0x92 => sub(cpu, memory, D),
        0x93 => sub(cpu, memory, E),
        0x94 => sub(cpu, memory, H),
        0x95 => sub(cpu, memory, L),
        0x96 => sub(cpu, memory, AddrHL),
        0x97 => sub(cpu, memory, A),
        0x98 => sbc(cpu, memory, B),
        0x99 => sbc(cpu, memory, C),
        0x9A => sbc(cpu, memory, D),
        0x9B => sbc(cpu, memory, E),
        0x9C => sbc(cpu, memory, H),
        0x9D => sbc(cpu, memory, L),
        0x9E => sbc(cpu, memory, AddrHL),
        0x9F => sbc(cpu, memory, A),
        0xA0 => and(cpu, memory, B),
        0xA1 => and(cpu, memory, C),
        0xA2 => and(cpu, memory, D),
        0xA3 => and(cpu, memory, E),
        0xA4 => and(cpu, memory, H),
        0xA5 => and(cpu, memory, L),
        0xA6 => and(cpu, memory, AddrHL),
        0xA7 => and(cpu, memory, A),
        0xA8 => xor(cpu, memory, B),
        0xA9 => xor(cpu, memory, C),
        0xAA => xor(cpu, memory, D),
        0xAB => xor(cpu, memory, E),
        0xAC => xor(cpu, memory, H),
        0xAD => xor(cpu, memory, L),
        0xAE => xor(cpu, memory, AddrHL),
        0xAF => xor(cpu, memory, A),
        0xB0 => or(cpu, memory, B),
        0xB1 => or(cpu, memory, C),
        0xB2 => or(cpu, memory, D),
        0xB3 => or(cpu, memory, E),
        0xB4 => or(cpu, memory, H),
        0xB5 => or(cpu, memory, L),
        0xB6 => or(cpu, memory, AddrHL),
        0xB7 => or(cpu, memory, A),
        0xB8 => cp(cpu, memory, B),
        0xB9 => cp(cpu, memory, C),
        0xBA => cp(cpu, memory, D),
        0xBB => cp(cpu, memory, E),
        0xBC => cp(cpu, memory, H),
        0xBD => cp(cpu, memory, L),
        0xBE => cp(cpu, memory, AddrHL),
        0xBF => cp(cpu, memory, A),
        0xC0 => ret(cpu, memory, Check::NZ),
        0xC1 => pop(cpu, memory, BC),
        0xC2 => jp_n(cpu, memory, Check::NZ),
        0xC3 => jp_n(cpu, memory, Check::True),
        0xC4 => call(cpu, memory, Check::NZ),
        0xC5 => push(cpu, memory, BC),
        0xC6 => add_8(cpu, memory, Imm8),
        0xC7 => rst(cpu, memory, 0x00),
        0xC8 => ret(cpu, memory, Check::Z),
        0xC9 => ret(cpu, memory, Check::True),
        0xCA => jp_n(cpu, memory, Check::Z),
        0xCC => call(cpu, memory, Check::Z),
        0xCD => call(cpu, memory, Check::True),
        0xCE => adc(cpu, memory, Imm8),
        0xCF => rst(cpu, memory, 0x08),
        0xD0 => ret(cpu, memory, Check::NC),
        0xD1 => pop(cpu, memory, DE),
        0xD2 => jp_n(cpu, memory, Check::NC),
        0xD4 => call(cpu, memory, Check::NC),
        0xD5 => push(cpu, memory, DE),
        0xD6 => sub(cpu, memory, Imm8),
        0xD7 => rst(cpu, memory, 0x10),
        0xD8 => ret(cpu, memory, Check::C),
        0xD9 => reti(cpu, memory),
        0xDA => jp_n(cpu, memory, Check::C),
        0xDC => call(cpu, memory, Check::C),
        0xDE => sbc(cpu, memory, Imm8),
        0xDF => rst(cpu, memory, 0x18),
        0xE0 => ldh_n(cpu, memory),
        0xE1 => pop(cpu, memory, HL),
        0xE2 => load_8(cpu, memory, AddrC, A),
        0xE5 => push(cpu, memory, HL),
        0xE6 => and(cpu, memory, Imm8),
        0xE7 => rst(cpu, memory, 0x20),
        0xE8 => add_sp(cpu, memory, SP, Imm8),
        0xE9 => jp(cpu, memory, HL),
        0xEA => load_8(cpu, memory, AddrImm16, A),
        0xEE => xor(cpu, memory, Imm8),
        0xEF => rst(cpu, memory, 0x28),
        0xF0 => ldh_a(cpu, memory),
        0xF1 => pop(cpu, memory, AF),
        0xF2 => load_8(cpu, memory, A, AddrC),
        0xF3 => cpu.reset_ime(),
        0xF5 => push(cpu, memory, AF),
        0xF6 => or(cpu, memory, Imm8),
        0xF7 => rst(cpu, memory, 0x30),
        0xF8 => ldhl(cpu, memory),
        0xF9 => load_16(cpu, memory, SP, HL),
        0xFA => load_8(cpu, memory, A, AddrImm16),
        0xFB => cpu.set_ime_delayed(),
        0xFE => cp(cpu, memory, Imm8),
        0xFF => rst(cpu, memory, 0x38),
        _ => {
            println!("{:?}", cpu.registers);
            panic!(format!("Unknown operation 0x{:X}", op));
        }
    }
}

/// Execute commands in the extended instruction space
fn execute_extended(op: u8, cpu: &mut CPU, memory: &mut MemoryBus) {
    use self::Loc16::*;
    use self::Loc8::*;

    match op {
        0x00 => rlc(cpu, memory, B),
        0x01 => rlc(cpu, memory, C),
        0x02 => rlc(cpu, memory, D),
        0x03 => rlc(cpu, memory, E),
        0x04 => rlc(cpu, memory, H),
        0x05 => rlc(cpu, memory, L),
        0x06 => rlc(cpu, memory, AddrHL),
        0x07 => rlc(cpu, memory, A),
        0x08 => rrc(cpu, memory, B),
        0x09 => rrc(cpu, memory, C),
        0x0A => rrc(cpu, memory, D),
        0x0B => rrc(cpu, memory, E),
        0x0C => rrc(cpu, memory, H),
        0x0D => rrc(cpu, memory, L),
        0x0E => rrc(cpu, memory, AddrHL),
        0x0F => rrc(cpu, memory, A),
        0x10 => rl(cpu, memory, B),
        0x11 => rl(cpu, memory, C),
        0x12 => rl(cpu, memory, D),
        0x13 => rl(cpu, memory, E),
        0x14 => rl(cpu, memory, H),
        0x15 => rl(cpu, memory, L),
        0x16 => rl(cpu, memory, AddrHL),
        0x17 => rl(cpu, memory, A),
        0x18 => rr(cpu, memory, B),
        0x19 => rr(cpu, memory, C),
        0x1A => rr(cpu, memory, D),
        0x1B => rr(cpu, memory, E),
        0x1C => rr(cpu, memory, H),
        0x1D => rr(cpu, memory, L),
        0x1E => rr(cpu, memory, AddrHL),
        0x1F => rr(cpu, memory, A),
        0x20 => sla(cpu, memory, B),
        0x21 => sla(cpu, memory, C),
        0x22 => sla(cpu, memory, D),
        0x23 => sla(cpu, memory, E),
        0x24 => sla(cpu, memory, H),
        0x25 => sla(cpu, memory, L),
        0x26 => sla(cpu, memory, AddrHL),
        0x27 => sla(cpu, memory, A),
        0x28 => sra(cpu, memory, B),
        0x29 => sra(cpu, memory, C),
        0x2A => sra(cpu, memory, D),
        0x2B => sra(cpu, memory, E),
        0x2C => sra(cpu, memory, H),
        0x2D => sra(cpu, memory, L),
        0x2E => sra(cpu, memory, AddrHL),
        0x2F => sra(cpu, memory, A),
        0x30 => swap(cpu, memory, B),
        0x31 => swap(cpu, memory, C),
        0x32 => swap(cpu, memory, D),
        0x33 => swap(cpu, memory, E),
        0x34 => swap(cpu, memory, H),
        0x35 => swap(cpu, memory, L),
        0x36 => swap(cpu, memory, AddrHL),
        0x37 => swap(cpu, memory, A),
        0x38 => srl(cpu, memory, B),
        0x39 => srl(cpu, memory, C),
        0x3A => srl(cpu, memory, D),
        0x3B => srl(cpu, memory, E),
        0x3C => srl(cpu, memory, H),
        0x3D => srl(cpu, memory, L),
        0x3E => srl(cpu, memory, AddrHL),
        0x3F => srl(cpu, memory, A),
        0x40 => bit(cpu, memory, B, 0),
        0x41 => bit(cpu, memory, C, 1),
        0x42 => bit(cpu, memory, D, 2),
        0x43 => bit(cpu, memory, E, 3),
        0x44 => bit(cpu, memory, H, 4),
        0x45 => bit(cpu, memory, L, 5),
        0x46 => bit(cpu, memory, AddrHL, 6),
        0x47 => bit(cpu, memory, A, 7),
        0x48 => bit(cpu, memory, B, 0),
        0x49 => bit(cpu, memory, C, 1),
        0x4A => bit(cpu, memory, D, 2),
        0x4B => bit(cpu, memory, E, 3),
        0x4C => bit(cpu, memory, H, 4),
        0x4D => bit(cpu, memory, L, 5),
        0x4E => bit(cpu, memory, AddrHL, 6),
        0x4F => bit(cpu, memory, A, 7),
        0x50 => bit(cpu, memory, B, 0),
        0x51 => bit(cpu, memory, C, 1),
        0x52 => bit(cpu, memory, D, 2),
        0x53 => bit(cpu, memory, E, 3),
        0x54 => bit(cpu, memory, H, 4),
        0x55 => bit(cpu, memory, L, 5),
        0x56 => bit(cpu, memory, AddrHL, 6),
        0x57 => bit(cpu, memory, A, 7),
        0x58 => bit(cpu, memory, B, 0),
        0x59 => bit(cpu, memory, C, 1),
        0x5A => bit(cpu, memory, D, 2),
        0x5B => bit(cpu, memory, E, 3),
        0x5C => bit(cpu, memory, H, 4),
        0x5D => bit(cpu, memory, L, 5),
        0x5E => bit(cpu, memory, AddrHL, 6),
        0x5F => bit(cpu, memory, A, 7),
        0x60 => bit(cpu, memory, B, 0),
        0x61 => bit(cpu, memory, C, 1),
        0x62 => bit(cpu, memory, D, 2),
        0x63 => bit(cpu, memory, E, 3),
        0x64 => bit(cpu, memory, H, 4),
        0x65 => bit(cpu, memory, L, 5),
        0x66 => bit(cpu, memory, AddrHL, 6),
        0x67 => bit(cpu, memory, A, 7),
        0x68 => bit(cpu, memory, B, 0),
        0x69 => bit(cpu, memory, C, 1),
        0x6A => bit(cpu, memory, D, 2),
        0x6B => bit(cpu, memory, E, 3),
        0x6C => bit(cpu, memory, H, 4),
        0x6D => bit(cpu, memory, L, 5),
        0x6E => bit(cpu, memory, AddrHL, 6),
        0x6F => bit(cpu, memory, A, 7),
        0x70 => bit(cpu, memory, B, 0),
        0x71 => bit(cpu, memory, C, 1),
        0x72 => bit(cpu, memory, D, 2),
        0x73 => bit(cpu, memory, E, 3),
        0x74 => bit(cpu, memory, H, 4),
        0x75 => bit(cpu, memory, L, 5),
        0x76 => bit(cpu, memory, AddrHL, 6),
        0x77 => bit(cpu, memory, A, 7),
        0x78 => bit(cpu, memory, B, 0),
        0x79 => bit(cpu, memory, C, 1),
        0x7A => bit(cpu, memory, D, 2),
        0x7B => bit(cpu, memory, E, 3),
        0x7C => bit(cpu, memory, H, 4),
        0x7D => bit(cpu, memory, L, 5),
        0x7E => bit(cpu, memory, AddrHL, 6),
        0x7F => bit(cpu, memory, A, 7),
        0x80 => res(cpu, memory, B, 0),
        0x81 => res(cpu, memory, C, 1),
        0x82 => res(cpu, memory, D, 2),
        0x83 => res(cpu, memory, E, 3),
        0x84 => res(cpu, memory, H, 4),
        0x85 => res(cpu, memory, L, 5),
        0x86 => res(cpu, memory, AddrHL, 6),
        0x87 => res(cpu, memory, A, 7),
        0x88 => res(cpu, memory, B, 0),
        0x89 => res(cpu, memory, C, 1),
        0x8A => res(cpu, memory, D, 2),
        0x8B => res(cpu, memory, E, 3),
        0x8C => res(cpu, memory, H, 4),
        0x8D => res(cpu, memory, L, 5),
        0x8E => res(cpu, memory, AddrHL, 6),
        0x8F => res(cpu, memory, A, 7),
        0x90 => res(cpu, memory, B, 0),
        0x91 => res(cpu, memory, C, 1),
        0x92 => res(cpu, memory, D, 2),
        0x93 => res(cpu, memory, E, 3),
        0x94 => res(cpu, memory, H, 4),
        0x95 => res(cpu, memory, L, 5),
        0x96 => res(cpu, memory, AddrHL, 6),
        0x97 => res(cpu, memory, A, 7),
        0x98 => res(cpu, memory, B, 0),
        0x99 => res(cpu, memory, C, 1),
        0x9A => res(cpu, memory, D, 2),
        0x9B => res(cpu, memory, E, 3),
        0x9C => res(cpu, memory, H, 4),
        0x9D => res(cpu, memory, L, 5),
        0x9E => res(cpu, memory, AddrHL, 6),
        0x9F => res(cpu, memory, A, 7),
        0xA0 => res(cpu, memory, B, 0),
        0xA1 => res(cpu, memory, C, 1),
        0xA2 => res(cpu, memory, D, 2),
        0xA3 => res(cpu, memory, E, 3),
        0xA4 => res(cpu, memory, H, 4),
        0xA5 => res(cpu, memory, L, 5),
        0xA6 => res(cpu, memory, AddrHL, 6),
        0xA7 => res(cpu, memory, A, 7),
        0xA8 => res(cpu, memory, B, 0),
        0xA9 => res(cpu, memory, C, 1),
        0xAA => res(cpu, memory, D, 2),
        0xAB => res(cpu, memory, E, 3),
        0xAC => res(cpu, memory, H, 4),
        0xAD => res(cpu, memory, L, 5),
        0xAE => res(cpu, memory, AddrHL, 6),
        0xAF => res(cpu, memory, A, 7),
        0xB0 => res(cpu, memory, B, 0),
        0xB1 => res(cpu, memory, C, 1),
        0xB2 => res(cpu, memory, D, 2),
        0xB3 => res(cpu, memory, E, 3),
        0xB4 => res(cpu, memory, H, 4),
        0xB5 => res(cpu, memory, L, 5),
        0xB6 => res(cpu, memory, AddrHL, 6),
        0xB7 => res(cpu, memory, A, 7),
        0xB8 => res(cpu, memory, B, 0),
        0xB9 => res(cpu, memory, C, 1),
        0xBA => res(cpu, memory, D, 2),
        0xBB => res(cpu, memory, E, 3),
        0xBC => res(cpu, memory, H, 4),
        0xBD => res(cpu, memory, L, 5),
        0xBE => res(cpu, memory, AddrHL, 6),
        0xBF => res(cpu, memory, A, 7),
        0xC0 => set(cpu, memory, B, 0),
        0xC1 => set(cpu, memory, C, 1),
        0xC2 => set(cpu, memory, D, 2),
        0xC3 => set(cpu, memory, E, 3),
        0xC4 => set(cpu, memory, H, 4),
        0xC5 => set(cpu, memory, L, 5),
        0xC6 => set(cpu, memory, AddrHL, 6),
        0xC7 => set(cpu, memory, A, 7),
        0xC8 => set(cpu, memory, B, 0),
        0xC9 => set(cpu, memory, C, 1),
        0xCA => set(cpu, memory, D, 2),
        0xCB => set(cpu, memory, E, 3),
        0xCC => set(cpu, memory, H, 4),
        0xCD => set(cpu, memory, L, 5),
        0xCE => set(cpu, memory, AddrHL, 6),
        0xCF => set(cpu, memory, A, 7),
        0xD0 => set(cpu, memory, B, 0),
        0xD1 => set(cpu, memory, C, 1),
        0xD2 => set(cpu, memory, D, 2),
        0xD3 => set(cpu, memory, E, 3),
        0xD4 => set(cpu, memory, H, 4),
        0xD5 => set(cpu, memory, L, 5),
        0xD6 => set(cpu, memory, AddrHL, 6),
        0xD7 => set(cpu, memory, A, 7),
        0xD8 => set(cpu, memory, B, 0),
        0xD9 => set(cpu, memory, C, 1),
        0xDA => set(cpu, memory, D, 2),
        0xDB => set(cpu, memory, E, 3),
        0xDC => set(cpu, memory, H, 4),
        0xDD => set(cpu, memory, L, 5),
        0xDE => set(cpu, memory, AddrHL, 6),
        0xDF => set(cpu, memory, A, 7),
        0xE0 => set(cpu, memory, B, 0),
        0xE1 => set(cpu, memory, C, 1),
        0xE2 => set(cpu, memory, D, 2),
        0xE3 => set(cpu, memory, E, 3),
        0xE4 => set(cpu, memory, H, 4),
        0xE5 => set(cpu, memory, L, 5),
        0xE6 => set(cpu, memory, AddrHL, 6),
        0xE7 => set(cpu, memory, A, 7),
        0xE8 => set(cpu, memory, B, 0),
        0xE9 => set(cpu, memory, C, 1),
        0xEA => set(cpu, memory, D, 2),
        0xEB => set(cpu, memory, E, 3),
        0xEC => set(cpu, memory, H, 4),
        0xED => set(cpu, memory, L, 5),
        0xEE => set(cpu, memory, AddrHL, 6),
        0xEF => set(cpu, memory, A, 7),
        0xF0 => set(cpu, memory, B, 0),
        0xF1 => set(cpu, memory, C, 1),
        0xF2 => set(cpu, memory, D, 2),
        0xF3 => set(cpu, memory, E, 3),
        0xF4 => set(cpu, memory, H, 4),
        0xF5 => set(cpu, memory, L, 5),
        0xF6 => set(cpu, memory, AddrHL, 6),
        0xF7 => set(cpu, memory, A, 7),
        0xF8 => set(cpu, memory, B, 0),
        0xF9 => set(cpu, memory, C, 1),
        0xFA => set(cpu, memory, D, 2),
        0xFB => set(cpu, memory, E, 3),
        0xFC => set(cpu, memory, H, 4),
        0xFD => set(cpu, memory, L, 5),
        0xFE => set(cpu, memory, AddrHL, 6),
        0xFF => set(cpu, memory, A, 7),
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

fn load_8(cpu: &mut CPU, memory: &mut MemoryBus, dest: Loc8, src: Loc8) {
    let value = src.read(cpu, memory);
    dest.write(cpu, memory, value);
}

fn load_16(cpu: &mut CPU, memory: &mut MemoryBus, dest: Loc16, src: Loc16) {
    let value = src.read(cpu, memory);
    dest.write(cpu, memory, value);
}

fn inc_8(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8) {
    let value = loc.read(cpu, memory);
    let result = value.wrapping_add(1);
    loc.write(cpu, memory, result);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = (value & 0x0F) == 0x0F;
}

fn inc_16(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc16) {
    let value = loc.read(cpu, memory);
    let result = value.wrapping_add(1);
    loc.write(cpu, memory, result);
}

fn dec_8(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8) {
    let value = loc.read(cpu, memory);
    let result = value.wrapping_sub(1);
    loc.write(cpu, memory, result);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = true;
    cpu.registers.f.half_carry = (value & 0x0F) == 0;
}

fn dec_16(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc16) {
    let value = loc.read(cpu, memory);
    let result = value.wrapping_sub(1);
    loc.write(cpu, memory, result);
}

fn scf(cpu: &mut CPU) {
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = true;
}

fn add_8(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8) {
    let left = cpu.registers.a;
    let right = loc.read(cpu, memory);

    let (result, carry) = left.overflowing_add(right);
    let half_carry = (left & 0x0F) + (right & 0x0F) > 0x0F;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = half_carry;
    cpu.registers.f.carry = carry;
    cpu.registers.a = result;
}

fn add_16(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc16) {
    let left = cpu.registers.get_hl();
    let right = loc.read(cpu, memory);

    let (result, carry) = left.overflowing_add(right);
    let half_carry = (left & 0x0FFF) + (right & 0x0FFF) > 0x0FFF;

    cpu.registers.f.subtract = false;
    cpu.registers.f.carry = carry;
    cpu.registers.f.half_carry = half_carry;
    cpu.registers.set_hl(result);
}

fn add_sp(cpu: &mut CPU, memory: &mut MemoryBus, dest: Loc16, src: Loc8) {
    // TODO: go over this implementation
    let sp = cpu.registers.sp;
    let byte = src.read(cpu, memory);

    let value = byte as i8 as i16 as u16;
    let result = sp.wrapping_add(value);

    let carry = (sp & 0xFF) + (value & 0xFF) > 0xFF;
    let half_carry = (sp & 0x0F) + (value & 0x0F) > 0x0F;

    cpu.registers.f.zero = false;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = half_carry;
    cpu.registers.f.carry = carry;
    dest.write(cpu, memory, result);
}

fn adc(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8) {
    let left = cpu.registers.a;
    let right = loc.read(cpu, memory);

    let carry_value = bits::from_bool(cpu.registers.f.carry);
    let result = left.wrapping_add(right).wrapping_add(carry_value);

    let carry = (left as u16 + right as u16 + carry_value as u16) > 0xFF;
    let half_carry = ((left & 0x0F) + (right & 0x0F) + carry_value) > 0x0F;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = half_carry;
    cpu.registers.f.carry = carry;
    cpu.registers.a = result;
}

fn sub(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8) {
    let left = cpu.registers.a;
    let right = loc.read(cpu, memory);
    let result = left.wrapping_sub(right);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = true;
    cpu.registers.f.half_carry = (right & 0x0F) > (left & 0x0F);
    cpu.registers.f.carry = right > left;
    cpu.registers.a = result;
}

fn sbc(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8) {
    let left = cpu.registers.a;
    let right = loc.read(cpu, memory);

    let carry_value = bits::from_bool(cpu.registers.f.carry);
    let result = left.wrapping_sub(right).wrapping_sub(carry_value);

    let carry = (right as u16 + carry_value as u16) > left as u16;
    let half_carry = ((right & 0x0F) + carry_value) > (left & 0x0F);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = true;
    cpu.registers.f.half_carry = half_carry;
    cpu.registers.f.carry = carry;
    cpu.registers.a = result;
}

fn cp(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8) {
    let left = cpu.registers.a;
    let right = loc.read(cpu, memory);
    let result = left.wrapping_sub(right);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = true;
    cpu.registers.f.half_carry = (right & 0x0F) > (left & 0x0F);
    cpu.registers.f.carry = right > left;
}

// See: http://gbdev.gg8.se/wiki/articles/DAA
fn daa(cpu: &mut CPU, memory: &mut MemoryBus) {
    let n = cpu.registers.f.subtract;
    let c = cpu.registers.f.carry;
    let h = cpu.registers.f.half_carry;
    let mut result = cpu.registers.a;

    if !n {
        if h || (result & 0x0F) > 0x09 {
            result = result.wrapping_add(0x06);
        }

        if c || result > 0x9F {
            result = result.wrapping_add(0x60);
            cpu.registers.f.carry = true;
        }
    } else {
        if h {
            result = result.wrapping_sub(0x06);
        }

        if c {
            result = result.wrapping_sub(0x60);
        }
    }

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.half_carry = false;
    cpu.registers.a = result;
}

fn and(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8) {
    let left = cpu.registers.a;
    let right = loc.read(cpu, memory);
    let result = left & right;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = true;
    cpu.registers.f.carry = false;
    cpu.registers.a = result;
}

fn or(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8) {
    let left = cpu.registers.a;
    let right = loc.read(cpu, memory);
    let result = left | right;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
    cpu.registers.a = result;
}

fn xor(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8) {
    let left = cpu.registers.a;
    let right = loc.read(cpu, memory);
    let result = left ^ right;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
    cpu.registers.a = result;
}

fn cpl(cpu: &mut CPU) {
    let value = cpu.registers.a;
    let result = value ^ 0xFF;

    cpu.registers.f.subtract = true;
    cpu.registers.f.half_carry = true;
    cpu.registers.a = result;
}

fn ccf(cpu: &mut CPU) {
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = !cpu.registers.f.carry;
}

fn sla(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8) {
    let value = loc.read(cpu, memory);
    let result = value << 1;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = bits::to_bool(value & 0x80);
    loc.write(cpu, memory, result);
}

fn sra(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8) {
    let value = loc.read(cpu, memory);
    let result = value >> 1;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = bits::to_bool(value & 0x1);
    loc.write(cpu, memory, result);
}

fn srl(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8) {
    let value = loc.read(cpu, memory);
    let result = value >> 1;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = bits::to_bool(value & 0x1);
    loc.write(cpu, memory, result);
}

fn rrc(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8) {
    let value = loc.read(cpu, memory);
    let result = value.rotate_right(1);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = bits::to_bool(value & 0x1);
    loc.write(cpu, memory, result);
}

fn rr(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8) {
    let value = loc.read(cpu, memory);
    let carry = bits::from_bool(cpu.registers.f.carry);
    let result = (carry << 7) | (value >> 1);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = bits::to_bool(value & 0x1);
    loc.write(cpu, memory, result);
}

fn rlc(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8) {
    let value = loc.read(cpu, memory);
    let result = value.rotate_left(1);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = bits::to_bool(value >> 7);
    loc.write(cpu, memory, result);
}

fn rl(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8) {
    let value = loc.read(cpu, memory);
    let carry = bits::from_bool(cpu.registers.f.carry);
    let result = (value << 1) | carry;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = bits::to_bool(value >> 7);
    loc.write(cpu, memory, result);
}

fn bit(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8, index: u8) {
    let value = loc.read(cpu, memory);
    let result = value & (1 << index);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = true;
}

fn res(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8, index: u8) {
    let value = loc.read(cpu, memory);
    loc.write(cpu, memory, bits::reset(value, index));
}

fn set(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8, index: u8) {
    let value = loc.read(cpu, memory);
    loc.write(cpu, memory, bits::set(value, index));
}

fn swap(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc8) {
    let value = loc.read(cpu, memory);
    let high_nibble = (value & 0xF0) >> 4;
    let low_nibble = (value & 0x0F) << 4;
    let result = low_nibble | high_nibble;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
    loc.write(cpu, memory, value);
}

fn ldi(cpu: &mut CPU, memory: &mut MemoryBus, dest: Loc8, src: Loc8) {
    let value = src.read(cpu, memory);
    dest.write(cpu, memory, value);
    inc_16(cpu, memory, Loc16::HL);
}

fn ldd(cpu: &mut CPU, memory: &mut MemoryBus, dest: Loc8, src: Loc8) {
    let value = src.read(cpu, memory);
    dest.write(cpu, memory, value);
    dec_16(cpu, memory, Loc16::HL);
}

fn ldh_n(cpu: &mut CPU, memory: &mut MemoryBus) {
    let offset = cpu.get_byte(memory);
    let address = bits::to_word(0xFF, offset);
    memory.set_byte(address, cpu.registers.a);
}

fn ldh_a(cpu: &mut CPU, memory: &mut MemoryBus) {
    let offset = cpu.get_byte(memory);
    let address = bits::to_word(0xFF, offset);
    cpu.registers.a = memory.get_byte(address);
}

fn ldhl(cpu: &mut CPU, memory: &mut MemoryBus) {
    add_sp(cpu, memory, Loc16::HL, Loc8::Imm8);
}

fn push(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc16) {
    cpu.registers.decrement_sp();
    cpu.registers.decrement_sp();

    let address = loc.read(cpu, memory);
    memory.set_word(cpu.registers.sp, address);
}

fn pop(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc16) {
    let result = memory.get_word(cpu.registers.sp);
    loc.write(cpu, memory, result);

    cpu.registers.increment_sp();
    cpu.registers.increment_sp();
}

fn rst(cpu: &mut CPU, memory: &mut MemoryBus, new_pc: u16) {
    push(cpu, memory, Loc16::PC);
    cpu.registers.pc = new_pc;
}

fn call(cpu: &mut CPU, memory: &mut MemoryBus, check: Check) {
    // Must get before checking to advance cycles
    let address = cpu.get_word(memory);

    if check.evaluate(cpu) {
        push(cpu, memory, Loc16::PC);
        cpu.registers.pc = address;
    }
}

fn ret(cpu: &mut CPU, memory: &mut MemoryBus, check: Check) {
    if check.evaluate(cpu) {
        pop(cpu, memory, Loc16::PC);
    }
}

fn reti(cpu: &mut CPU, memory: &mut MemoryBus) {
    cpu.set_ime();
    ret(cpu, memory, Check::True);
}

fn jp(cpu: &mut CPU, memory: &mut MemoryBus, loc: Loc16) {
    cpu.registers.pc = loc.read(cpu, memory);
}

fn jp_n(cpu: &mut CPU, memory: &mut MemoryBus, check: Check) {
    // Must get before checking to advance cycles
    let address = cpu.get_word(memory);

    if check.evaluate(cpu) {
        cpu.registers.pc = address;
    }
}

fn jr_n(cpu: &mut CPU, memory: &mut MemoryBus, check: Check) {
    // Must get before checking to advance cycles
    let offset = cpu.get_byte(memory) as i8;

    if check.evaluate(cpu) {
        // Effectively subtracts because of wrap
        // eg. 0xeb (i8) becomes 0xffeb (u16)
        cpu.registers.add_pc(offset as u16);
    }
}

fn debug(label: &str) {
    // println!("{}", label)
}
