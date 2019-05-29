mod mbc;

use self::mbc::mbc_from_byte;
use self::mbc::MBC;

pub struct Cartridge {
    mbc: Box<dyn MBC>,
    rom: Box<[u8]>,
    ram: Box<[u8]>,
}

impl Cartridge {
    pub fn new(data: Vec<u8>) -> Cartridge {
        let empty_ram = vec![0; 0x8000];

        Cartridge {
            mbc: mbc_from_byte(data[0x147]),
            rom: data.into_boxed_slice(),
            ram: empty_ram.into_boxed_slice(),
        }
    }

    pub fn read_rom_bank0(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    pub fn read_rom_bank1(&self, address: u16) -> u8 {
        self.mbc.read_rom_bank1(&self.rom, address)
    }

    pub fn write_registers(&mut self, address: u16, byte: u8) {
        self.mbc.write_registers(address, byte)
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        self.mbc.read_ram(&self.ram, address)
    }

    pub fn write_ram(&mut self, address: u16, byte: u8) {
        self.mbc.write_ram(&mut self.ram, address, byte)
    }
}
