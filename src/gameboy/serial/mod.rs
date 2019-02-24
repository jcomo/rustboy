/*
 * Serial data transfer emulation. This implementation completely fakes the transfer
 * since there are no plans to support linking within this emulator. If/when a real
 * implementation is required, this page has good information about expected behavior.
 *   Reference: http://gbdev.gg8.se/wiki/articles/Serial_Data_Transfer_(Link_Cable)
 */

use crate::bits;

#[derive(PartialEq, Default, Debug)]
struct Control {
    transfer: bool,
    internal_clock: bool,
}

impl From<&Control> for u8 {
    fn from(control: &Control) -> u8 {
        bits::from_bool(control.transfer) << 7 | bits::from_bool(control.internal_clock)
    }
}

impl From<u8> for Control {
    fn from(byte: u8) -> Control {
        Control {
            transfer: bits::is_set(byte, 7),
            internal_clock: bits::is_set(byte, 0),
        }
    }
}

#[derive(Default)]
pub struct Serial {
    data: u8,
    control: Control,
}

impl Serial {
    pub fn new() -> Serial {
        Serial::default()
    }

    pub fn get_data(&self) -> u8 {
        self.data
    }

    pub fn set_data(&mut self, byte: u8) {
        self.data = byte;
    }

    pub fn get_control(&self) -> u8 {
        u8::from(&self.control)
    }

    pub fn set_control(&mut self, byte: u8) {
        self.control = Control::from(byte);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn u8_from_control() {
        let control = Control {
            transfer: true,
            internal_clock: true,
        };

        assert_eq!(u8::from(&control), 0x81);
    }

    #[test]
    fn control_from_u8() {
        let control = Control::from(0x81);
        let expected = Control {
            transfer: true,
            internal_clock: true,
        };

        assert_eq!(control, expected);
    }

    #[test]
    fn serial_control() {
        let mut serial = Serial::new();
        let control = Control::from(0x81);

        serial.set_control(0x81);

        assert_eq!(serial.control, control);
        assert_eq!(serial.get_control(), 0x81);
    }

    #[test]
    fn serial_data() {
        let mut serial = Serial::new();

        serial.set_data(0xAA);

        assert_eq!(serial.get_data(), 0xAA);
    }
}
