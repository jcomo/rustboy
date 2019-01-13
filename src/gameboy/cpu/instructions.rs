use super::MemoryBus;
use super::CPU;

pub fn execute(op: u8, cpu: &mut CPU, memory: &mut MemoryBus) {
    match op {
        0x31 => {
            debug("LD SP, nn");
            let word = cpu.get_word(memory);
            cpu.registers.sp = word;
        }
        0xAF => {
            debug("XOR A, A");
            let result = xor(cpu, cpu.registers.a, cpu.registers.a);
            cpu.registers.a = result;
        }
        0x21 => {
            debug("SLA C");
        }
        _ => {
            panic!(format!("Unknown operation 0x{:X}", op));
        }
    }
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

fn debug(label: &'static str) {
    println!("{}", label)
}
