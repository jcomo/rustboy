mod mbc1;
mod none;

use self::mbc1::MBC1;
use self::none::NoMBC;

pub trait MBC {
    fn read_rom_bank1(&self, rom: &[u8], address: u16) -> u8;
    fn read_ram(&self, ram: &[u8], address: u16) -> u8;
    fn write_registers(&mut self, address: u16, byte: u8);
    fn write_ram(&mut self, ram: &mut [u8], address: u16, byte: u8);
}

pub fn mbc_from_byte(byte: u8) -> Box<dyn MBC> {
    match byte {
        0x00 => Box::new(NoMBC::new()),
        0x01...0x03 => Box::new(MBC1::new()),
        _ => panic!("unimplemented MBC: 0x{:x}", byte),
    }
}
