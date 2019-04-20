enum MBC {
    None,
    MBC1,
}

pub struct Cartridge {
    mode: MBC,
    rom: Box<[u8]>,
    rom_bank: u8,
    ram: Box<[u8]>,
    ram_bank: u8,
    ram_enabled: bool,
}

impl Cartridge {
    pub fn new(data: Vec<u8>) -> Cartridge {
        let empty_ram = vec![0; 0x8000];
        Cartridge {
            mode: MBC::None,
            rom: data.into_boxed_slice(),
            rom_bank: 0,
            ram: empty_ram.into_boxed_slice(),
            ram_bank: 0,
            ram_enabled: false,
        }
    }

    pub fn read_rom_bank0(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    pub fn read_rom_bank1(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    pub fn write_registers(&mut self, address: u16, byte: u8) {}

    pub fn read_ram(&self, address: u16) -> u8 {
        0xff // TODO
    }

    pub fn write_ram(&mut self, address: u16, byte: u8) {}
}
