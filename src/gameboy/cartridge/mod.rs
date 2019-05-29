trait MBC {
    fn read_rom_bank1(&self, rom: &[u8], address: u16) -> u8;
    fn read_ram(&self, ram: &[u8], address: u16) -> u8;
    fn write_registers(&mut self, rom: &[u8], address: u16, byte: u8);
    fn write_ram(&mut self, ram: &mut [u8], address: u16, byte: u8);
}

struct NoMBC {}

impl NoMBC {
    fn new() -> NoMBC {
        NoMBC {}
    }
}

impl MBC for NoMBC {
    fn read_rom_bank1(&self, rom: &[u8], address: u16) -> u8 {
        rom[address as usize]
    }

    fn read_ram(&self, ram: &[u8], address: u16) -> u8 {
        0xff
    }

    fn write_registers(&mut self, rom: &[u8], address: u16, byte: u8) {}

    fn write_ram(&mut self, ram: &mut [u8], address: u16, byte: u8) {}
}

enum BankingMode {
    ROM,
    RAM,
}

struct MBC1 {
    rom_bank_lower_bits: u8,
    rom_bank_upper_bits: u8,
    ram_bank: u8,
    ram_enabled: bool,
    banking_mode: BankingMode,
}

impl MBC1 {
    fn new() -> MBC1 {
        MBC1 {
            rom_bank_lower_bits: 1,
            rom_bank_upper_bits: 0,
            ram_bank: 0,
            ram_enabled: false,
            banking_mode: BankingMode::ROM,
        }
    }

    fn get_rom_bank(&self) -> u16 {
        let upper_bits = self.rom_bank_upper_bits & 0b11;
        let lower_bits = self.rom_bank_lower_bits & 0x1F;
        ((upper_bits << 5) | lower_bits) as u16
    }

    fn get_ram_bank(&self) -> u16 {
        (self.ram_bank & 0b11) as u16
    }
}

impl MBC for MBC1 {
    fn read_rom_bank1(&self, rom: &[u8], address: u16) -> u8 {
        let relative_address = address - 0x4000;
        let index = relative_address + (0x4000 * self.get_rom_bank());
        rom[index as usize]
    }

    fn read_ram(&self, ram: &[u8], address: u16) -> u8 {
        if self.ram_enabled {
            let index = address + (0x2000 * self.get_ram_bank());
            ram[index as usize]
        } else {
            0xff
        }
    }

    fn write_registers(&mut self, rom: &[u8], address: u16, byte: u8) {
        match address >> 8 {
            0x00...0x1F => {
                self.ram_enabled = byte & 0x0F == 0x0A;
            }
            0x20...0x3F => {
                self.rom_bank_lower_bits = match byte {
                    0x00 => 0x01,
                    0x20 => 0x21,
                    0x40 => 0x41,
                    0x60 => 0x61,
                    _ => byte,
                };
            }
            0x40...0x5F => match self.banking_mode {
                BankingMode::ROM => self.rom_bank_upper_bits = byte,
                BankingMode::RAM => self.ram_bank = byte,
            },
            0x60...0x7F => match byte & 0x01 {
                0x00 => self.banking_mode = BankingMode::ROM,
                0x01 => self.banking_mode = BankingMode::RAM,
                _ => unreachable!(),
            },
            _ => unreachable!("Invalid register address: 0x{:x}", address),
        }
    }

    fn write_ram(&mut self, ram: &mut [u8], address: u16, byte: u8) {
        if self.ram_enabled {
            let index = address + (0x2000 * self.get_ram_bank());
            ram[index as usize] = byte;
        }
    }
}

pub struct Cartridge {
    mbc: Box<dyn MBC>,
    rom: Box<[u8]>,
    ram: Box<[u8]>,
}

impl Cartridge {
    pub fn new(data: Vec<u8>) -> Cartridge {
        let empty_ram = vec![0; 0x8000];
        Cartridge {
            mbc: Cartridge::mbc_from_byte(data[0x147]),
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
        self.mbc.write_registers(&self.rom, address, byte)
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        self.mbc.read_ram(&self.ram, address)
    }

    pub fn write_ram(&mut self, address: u16, byte: u8) {
        self.mbc.write_ram(&mut self.ram, address, byte)
    }

    fn mbc_from_byte(byte: u8) -> Box<dyn MBC> {
        match byte {
            0x00 => Box::new(NoMBC::new()),
            0x01...0x03 => Box::new(MBC1::new()),
            _ => panic!("unimplemented MBC: 0x{:x}", byte),
        }
    }
}
