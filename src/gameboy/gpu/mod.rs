use crate::bits;

#[derive(Debug, PartialEq)]
enum Color {
    White = 0b00,
    Light = 0b01,
    Dark = 0b10,
    Black = 0b11,
}

impl From<u8> for Color {
    fn from(byte: u8) -> Color {
        match byte {
            0b00 => Color::White,
            0b01 => Color::Light,
            0b10 => Color::Dark,
            0b11 => Color::Black,
            _ => panic!("Unknown color 0x{:x}", byte),
        }
    }
}

impl From<&Color> for u8 {
    fn from(color: &Color) -> u8 {
        match color {
            Color::White => 0b00,
            Color::Light => 0b01,
            Color::Dark => 0b10,
            Color::Black => 0b11,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Palette {
    white: Color,
    light: Color,
    dark: Color,
    black: Color,
}

impl Palette {
    fn new() -> Palette {
        Palette {
            white: Color::White,
            light: Color::White,
            dark: Color::White,
            black: Color::White,
        }
    }
}

impl From<u8> for Palette {
    fn from(byte: u8) -> Palette {
        Palette {
            white: Color::from(byte & 0b11),
            light: Color::from((byte >> 2) & 0b11),
            dark: Color::from((byte >> 4) & 0b11),
            black: Color::from((byte >> 6) & 0b11),
        }
    }
}

impl From<&Palette> for u8 {
    fn from(palette: &Palette) -> u8 {
        u8::from(&palette.white)
            | u8::from(&palette.light) << 2
            | u8::from(&palette.dark) << 4
            | u8::from(&palette.black) << 6
    }
}

#[derive(Debug, PartialEq)]
struct Control {
    lcd_on: bool,
    window_map: bool,
    window_on: bool,
    bg_data: bool,
    bg_map: bool,
    obj_size: bool,
    obj_on: bool,
    bg_on: bool,
}

impl Control {
    fn new() -> Control {
        Control {
            lcd_on: false,
            window_map: false,
            window_on: false,
            bg_data: false,
            bg_map: false,
            obj_size: false,
            obj_on: false,
            bg_on: false,
        }
    }
}

impl From<u8> for Control {
    fn from(byte: u8) -> Control {
        Control {
            lcd_on: bits::is_set(byte, 7),
            window_map: bits::is_set(byte, 6),
            window_on: bits::is_set(byte, 5),
            bg_data: bits::is_set(byte, 4),
            bg_map: bits::is_set(byte, 3),
            obj_size: bits::is_set(byte, 2),
            obj_on: bits::is_set(byte, 1),
            bg_on: bits::is_set(byte, 0),
        }
    }
}

impl From<&Control> for u8 {
    fn from(control: &Control) -> u8 {
        bits::set(7, control.lcd_on)
            | bits::set(6, control.window_map)
            | bits::set(5, control.window_on)
            | bits::set(4, control.bg_data)
            | bits::set(3, control.bg_map)
            | bits::set(2, control.obj_size)
            | bits::set(1, control.obj_on)
            | bits::set(0, control.bg_on)
    }
}

pub struct GPU {
    current_line: u8,
    scroll_x: u8,
    scroll_y: u8,
    control: Control,
    bg_palette: Palette,
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            current_line: 0,
            scroll_x: 0,
            scroll_y: 0,
            control: Control::new(),
            bg_palette: Palette::new(),
        }
    }

    pub fn get_control(&self) -> u8 {
        u8::from(&self.control)
    }

    pub fn set_control(&mut self, value: u8) {
        self.control = Control::from(value)
    }

    pub fn get_current_line(&self) -> u8 {
        self.current_line
    }

    pub fn reset_current_line(&mut self) {
        self.current_line = 0
    }

    pub fn get_compare_line(&self) -> u8 {
        panic!("get_compare_line()")
    }

    pub fn set_compare_line(&mut self, value: u8) {
        panic!("set_compare_line(0x{:X})", value)
    }

    pub fn get_scroll_x(&self) -> u8 {
        self.scroll_x
    }

    pub fn set_scroll_x(&mut self, value: u8) {
        self.scroll_x = value
    }

    pub fn get_scroll_y(&self) -> u8 {
        self.scroll_y
    }

    pub fn set_scroll_y(&mut self, value: u8) {
        self.scroll_y = value
    }

    pub fn get_window_x(&self) -> u8 {
        panic!("get_window_x()")
    }

    pub fn set_window_x(&mut self, value: u8) {
        panic!("set_window_x(0x{:X})", value)
    }

    pub fn get_window_y(&self) -> u8 {
        panic!("get_window_y()")
    }

    pub fn set_window_y(&mut self, value: u8) {
        panic!("set_window_y(0x{:X})", value)
    }

    pub fn get_bg_palette(&self) -> u8 {
        u8::from(&self.bg_palette)
    }

    pub fn set_bg_palette(&mut self, value: u8) {
        self.bg_palette = Palette::from(value)
    }

    pub fn get_object_palette_0(&self) -> u8 {
        panic!("get_object_palette_0()")
    }

    pub fn set_object_palette_0(&mut self, value: u8) {
        panic!("set_object_palette_0(0x{:X})", value)
    }

    pub fn get_object_palette_1(&self) -> u8 {
        panic!("get_object_palette_1()")
    }

    pub fn set_object_palette_1(&mut self, value: u8) {
        panic!("set_object_palette_1(0x{:X})", value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn palette_from_u8() {
        let palette = Palette::from(0b11_10_01_00);

        assert_eq!(
            palette,
            Palette {
                white: Color::White,
                light: Color::Light,
                dark: Color::Dark,
                black: Color::Black,
            }
        )
    }

    #[test]
    fn u8_from_palette() {
        let palette = Palette::from(0b11_10_01_00);

        assert_eq!(u8::from(&palette), 0b11_10_01_00);
    }

    #[test]
    fn control_from_u8() {
        let control = Control::from(0b10101010);

        assert_eq!(
            control,
            Control {
                lcd_on: true,
                window_map: false,
                window_on: true,
                bg_data: false,
                bg_map: true,
                obj_size: false,
                obj_on: true,
                bg_on: false,
            }
        )
    }

    #[test]
    fn u8_from_control() {
        let control = Control::from(0b01010101);

        assert_eq!(u8::from(&control), 0b01010101);
    }

    #[test]
    fn gpu_bg_palette() {
        let mut gpu = GPU::new();
        let palette = Palette::from(0xFF);

        gpu.set_bg_palette(0xFF);

        assert_eq!(gpu.get_bg_palette(), 0xFF);
        assert_eq!(gpu.bg_palette, palette);
    }
}
