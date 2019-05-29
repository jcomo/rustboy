use super::MBC;

pub struct NoMBC {}

impl NoMBC {
    pub fn new() -> NoMBC {
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

    fn write_registers(&mut self, address: u16, byte: u8) {}

    fn write_ram(&mut self, ram: &mut [u8], address: u16, byte: u8) {}
}

#[cfg(test)]
mod test {
    use super::*;

}
