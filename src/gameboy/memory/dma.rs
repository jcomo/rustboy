const DMA_CYCLES: u8 = 160;

pub struct DMA {
    source: u8,
    offset: u8,
    running: bool,
}

impl DMA {
    pub fn new() -> DMA {
        DMA {
            source: 0x0,
            offset: 0,
            running: false,
        }
    }

    pub fn initialize(&mut self, addr_high: u8) {
        if self.running {
            println!("DMA already running (0x{:x})", addr_high);
            return;
        }

        self.source = addr_high;
        self.offset = 0;
        self.running = true;
    }

    pub fn emulate(&mut self) -> Option<u16> {
        if self.running {
            Some(self.get_next_address())
        } else {
            None
        }
    }

    pub fn get_source(&self) -> u8 {
        self.source
    }

    fn get_next_address(&mut self) -> u16 {
        let address = (self.source as u16) << 8;
        let address = address.wrapping_add(self.offset as u16);
        self.offset = self.offset.wrapping_add(1);
        if self.is_complete() {
            self.running = false;
        }

        address
    }

    fn is_complete(&self) -> bool {
        self.offset >= DMA_CYCLES
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initialize() {
        let mut dma = DMA::new();

        dma.initialize(0xC1);

        assert_eq!(dma.source, 0xC1);
        assert_eq!(dma.offset, 0);
        assert_eq!(dma.running, true);

        dma.initialize(0xA1);

        assert_eq!(dma.source, 0xC1);
    }

    #[test]
    fn emulate() {
        let mut dma = DMA::new();

        dma.initialize(0xC1);

        let address = dma.emulate();

        assert_eq!(address, Some(0xC100));
        assert_eq!(dma.running, true);

        let address = dma.emulate();

        assert_eq!(address, Some(0xC101));
        assert_eq!(dma.running, true);
    }

    #[test]
    fn emulate_stop_after_complete() {
        let mut dma = DMA::new();

        dma.initialize(0xC1);

        for _ in 0..DMA_CYCLES {
            let address = dma.emulate();

            assert_eq!(address.is_some(), true);
        }

        let address = dma.emulate();

        assert_eq!(address, None);
        assert_eq!(dma.running, false);
    }
}
