use super::MBC;

#[derive(Debug, PartialEq)]
enum BankingMode {
    ROM,
    RAM,
}

pub struct MBC1 {
    rom_bank_lower_bits: u8,
    rom_bank_upper_bits: u8,
    ram_bank: u8,
    ram_enabled: bool,
    banking_mode: BankingMode,
}

impl MBC1 {
    pub fn new() -> MBC1 {
        MBC1 {
            rom_bank_lower_bits: 1,
            rom_bank_upper_bits: 0,
            ram_bank: 0,
            ram_enabled: false,
            banking_mode: BankingMode::ROM,
        }
    }

    fn get_rom_bank(&self) -> usize {
        let upper_bits = self.rom_bank_upper_bits & 0b11;
        let lower_bits = self.rom_bank_lower_bits & 0x1F;
        ((upper_bits << 5) | lower_bits) as usize
    }

    fn get_ram_bank(&self) -> usize {
        (self.ram_bank & 0b11) as usize
    }
}

impl MBC for MBC1 {
    fn read_rom_bank1(&self, rom: &[u8], address: u16) -> u8 {
        let relative_address = (address - 0x4000) as usize;
        let offset = 0x4000 * self.get_rom_bank();
        rom[relative_address + offset]
    }

    fn read_ram(&self, ram: &[u8], address: u16) -> u8 {
        if self.ram_enabled {
            let relative_address = (address - 0xA000) as usize;
            let offset = 0x2000 * self.get_ram_bank();
            ram[relative_address + offset]
        } else {
            0xff
        }
    }

    fn write_registers(&mut self, address: u16, byte: u8) {
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
            let relative_address = (address - 0xA000) as usize;
            let offset = 0x2000 * self.get_ram_bank();
            ram[relative_address + offset] = byte;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_ram_bank() {
        let mut mbc = MBC1::new();

        mbc.ram_bank = 0x01;
        assert_eq!(0x01, mbc.get_ram_bank());

        mbc.ram_bank = 0x0F;
        assert_eq!(0x03, mbc.get_ram_bank());
    }

    #[test]
    fn get_rom_bank() {
        let mut mbc = MBC1::new();

        mbc.rom_bank_lower_bits = 0x1F;
        mbc.rom_bank_upper_bits = 0x03;

        assert_eq!(0x7F, mbc.get_rom_bank());
    }

    #[test]
    fn read_rom_bank1() {
        let mut mbc = MBC1::new();
        let mut rom = [0; 0xF000];

        rom[0x4001] = 1;

        assert_eq!(1, mbc.read_rom_bank1(&rom, 0x4001));

        rom[0x8001] = 2;
        mbc.rom_bank_lower_bits = 0x02;

        assert_eq!(2, mbc.read_rom_bank1(&rom, 0x4001));
    }

    #[test]
    fn read_rom_bank1_overflow_u16() {
        let mut mbc = MBC1::new();
        let rom = [0; 0x1FFFF];

        mbc.rom_bank_lower_bits = 0x05;
        assert_eq!(0, mbc.read_rom_bank1(&rom, 0x4001));
    }

    #[test]
    fn read_ram() {
        let mut mbc = MBC1::new();
        let ram = [1; 0x8000];

        mbc.ram_enabled = false;
        assert_eq!(0xFF, mbc.read_ram(&ram, 0xA040));

        mbc.ram_enabled = true;
        assert_eq!(1, mbc.read_ram(&ram, 0xA040));
    }

    #[test]
    fn write_registers_ram_enabled() {
        let mut mbc = MBC1::new();

        mbc.write_registers(0x00, 0xAA);
        assert_eq!(true, mbc.ram_enabled);

        mbc.write_registers(0x00, 0x0B);
        assert_eq!(false, mbc.ram_enabled);

        mbc.write_registers(0x00, 0x00);
        assert_eq!(false, mbc.ram_enabled);
    }

    #[test]
    fn write_registers_rom_bank() {
        let mut mbc = MBC1::new();

        mbc.write_registers(0x2000, 0x00);
        assert_eq!(0x01, mbc.rom_bank_lower_bits);

        mbc.write_registers(0x2000, 0x20);
        assert_eq!(0x21, mbc.rom_bank_lower_bits);

        mbc.write_registers(0x2000, 0x40);
        assert_eq!(0x41, mbc.rom_bank_lower_bits);

        mbc.write_registers(0x2000, 0x60);
        assert_eq!(0x61, mbc.rom_bank_lower_bits);

        mbc.write_registers(0x2000, 0x05);
        assert_eq!(0x05, mbc.rom_bank_lower_bits);
    }

    #[test]
    fn write_registers_rom_or_ram_bank() {
        let mut mbc = MBC1::new();

        mbc.banking_mode = BankingMode::ROM;
        mbc.write_registers(0x4000, 0b11);

        assert_eq!(0b11, mbc.rom_bank_upper_bits);
        assert_eq!(0b00, mbc.ram_bank);

        mbc.banking_mode = BankingMode::RAM;
        mbc.write_registers(0x4000, 0b10);

        assert_eq!(0b11, mbc.rom_bank_upper_bits);
        assert_eq!(0b10, mbc.ram_bank);
    }

    #[test]
    fn write_registers_banking_mode() {
        let mut mbc = MBC1::new();

        mbc.write_registers(0x6000, 0b11);
        assert_eq!(BankingMode::RAM, mbc.banking_mode);

        mbc.write_registers(0x6000, 0b01);
        assert_eq!(BankingMode::RAM, mbc.banking_mode);

        mbc.write_registers(0x6000, 0b00);
        assert_eq!(BankingMode::ROM, mbc.banking_mode);
    }

    #[test]
    fn write_ram() {
        let mut mbc = MBC1::new();
        let mut ram = [1; 0x8000];

        mbc.ram_enabled = false;
        mbc.write_ram(&mut ram, 0xB000, 0);

        assert_eq!(1, ram[0x1000]);

        mbc.ram_enabled = true;
        mbc.write_ram(&mut ram, 0xB000, 0);

        assert_eq!(0, ram[0x1000]);

        mbc.ram_bank = 0b10;
        mbc.write_ram(&mut ram, 0xB000, 5);

        assert_eq!(5, ram[0x5000]);
    }
}
