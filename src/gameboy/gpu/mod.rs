use crate::bits;
use crate::gameboy::display::NoDisplay;
use crate::gameboy::display::VideoDisplay;
use crate::gameboy::irq::Interrupt;
use crate::gameboy::irq::IRQ;
use crate::gameboy::Color;

// LCDC CPU cycle lengths
const OAM_CYCLES: i32 = 21;
const PIXEL_TRANSFER_CYCLES: i32 = 43;
const HBLANK_CYCLES: i32 = 50;
const VBLANK_CYCLES: i32 = 114;

// LCDC Stat mode flags
const MODE_FLAG_HBLANK: u8 = 0b00;
const MODE_FLAG_VBLANK: u8 = 0b01;
const MODE_FLAG_ACCESS_OAM: u8 = 0b10;
const MODE_FLAG_PIXEL_TRANSFER: u8 = 0b11;
const STAT_UNUSED: u8 = 0b1000_0000;

const V_SCANLINE_MAX: u8 = 160;
const H_SCANLINE_MAX: u8 = 144;
const H_SCANLINE_VBLANK_MAX: u8 = 153;

// (0x9800 - 0x8000 = 6kB) / 16 bytes per tile
const NUM_TILES: usize = 384;
const BYTES_PER_TILE: usize = 16;
const TILE_MAP_SIZE: usize = 0x400;

const NUM_SPRITES: usize = 40;

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

    fn map(&self, color: Color) -> Color {
        match color {
            Color::White => self.white.clone(),
            Color::Light => self.light.clone(),
            Color::Dark => self.dark.clone(),
            Color::Black => self.black.clone(),
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
        bits::from_bool(control.lcd_on) << 7
            | bits::from_bool(control.window_map) << 6
            | bits::from_bool(control.window_on) << 5
            | bits::from_bool(control.bg_data) << 4
            | bits::from_bool(control.bg_map) << 3
            | bits::from_bool(control.obj_size) << 2
            | bits::from_bool(control.obj_on) << 1
            | bits::from_bool(control.bg_on)
    }
}

#[derive(Debug, PartialEq)]
struct Stat {
    line_compare_interrupt: bool,
    hblank_interrupt: bool,
    vblank_interrupt: bool,
    access_oam_interrupt: bool,
    line_compare: bool,
    mode_flag: u8,
}

impl Stat {
    fn new() -> Stat {
        Stat {
            line_compare_interrupt: false,
            hblank_interrupt: false,
            vblank_interrupt: false,
            access_oam_interrupt: false,
            line_compare: false,
            mode_flag: MODE_FLAG_ACCESS_OAM,
        }
    }
}

impl From<u8> for Stat {
    fn from(byte: u8) -> Stat {
        Stat {
            line_compare_interrupt: bits::is_set(byte, 6),
            hblank_interrupt: bits::is_set(byte, 5),
            vblank_interrupt: bits::is_set(byte, 4),
            access_oam_interrupt: bits::is_set(byte, 3),
            line_compare: bits::is_set(byte, 2),
            mode_flag: byte & 0b11,
        }
    }
}

impl From<&Stat> for u8 {
    fn from(stat: &Stat) -> u8 {
        STAT_UNUSED
            | bits::from_bool(stat.line_compare_interrupt) << 6
            | bits::from_bool(stat.hblank_interrupt) << 5
            | bits::from_bool(stat.vblank_interrupt) << 4
            | bits::from_bool(stat.access_oam_interrupt) << 3
            | bits::from_bool(stat.line_compare) << 2
            | (stat.mode_flag & 0b11)
    }
}

#[derive(Debug, PartialEq)]
enum Mode {
    OAM,
    PixelTransfer,
    HBlank,
    VBlank,
}

impl Mode {
    fn flag_bits(&self) -> u8 {
        use self::Mode::*;

        match self {
            OAM => MODE_FLAG_ACCESS_OAM,
            PixelTransfer => MODE_FLAG_PIXEL_TRANSFER,
            HBlank => MODE_FLAG_HBLANK,
            VBlank => MODE_FLAG_VBLANK,
        }
    }

    fn cycles(&self) -> i32 {
        use self::Mode::*;

        match self {
            OAM => OAM_CYCLES,
            PixelTransfer => PIXEL_TRANSFER_CYCLES,
            HBlank => HBLANK_CYCLES,
            VBlank => VBLANK_CYCLES,
        }
    }
}

#[derive(Copy, Clone, Default)]
struct Tile {
    bytes: [u8; BYTES_PER_TILE],
}

impl Tile {
    fn new() -> Tile {
        Tile::default()
    }

    /// Returns the color data for the given (x, y) pixel in the tile
    fn get_color(&self, row: u8, col: u8) -> Color {
        let arr_offset = (row * 2) as usize;
        let top = self.bytes[arr_offset + 1];
        let bottom = self.bytes[arr_offset];

        let shift = 7 - col;
        let msb = (top >> shift) & 0x1;
        let lsb = (bottom >> shift) & 0x1;
        let value = (msb << 1) | lsb;
        Color::from(value)
    }
}

#[derive(Copy, Clone, Default)]
struct Sprite {
    y: u8,
    x: u8,
    tile_num: u8,
    flags: SpriteFlags,
}

impl Sprite {
    fn new() -> Sprite {
        Sprite::default()
    }

    fn get_x(&self) -> u8 {
        self.x.wrapping_sub(8)
    }

    fn get_y(&self) -> u8 {
        self.y.wrapping_sub(16)
    }
}

#[derive(Copy, Clone, Default)]
struct SpriteFlags {
    priority: bool,
    flip_y: bool,
    flip_x: bool,
    palette: bool,
}

impl From<u8> for SpriteFlags {
    fn from(byte: u8) -> SpriteFlags {
        SpriteFlags {
            priority: bits::is_set(byte, 7),
            flip_y: bits::is_set(byte, 6),
            flip_x: bits::is_set(byte, 5),
            palette: bits::is_set(byte, 4),
        }
    }
}

impl From<&SpriteFlags> for u8 {
    fn from(flags: &SpriteFlags) -> u8 {
        bits::from_bool(flags.priority) << 7
            | bits::from_bool(flags.flip_y) << 6
            | bits::from_bool(flags.flip_x) << 5
            | bits::from_bool(flags.palette) << 4
    }
}

pub struct GPU {
    current_line: u8,
    compare_line: u8,
    current_mode: Mode,
    remaining_cycles: i32,
    scroll_x: u8,
    scroll_y: u8,
    window_x: u8,
    window_y: u8,
    control: Control,
    stat: Stat,
    bg_palette: Palette,
    obj_palette_0: Palette,
    obj_palette_1: Palette,
    tile_map_0: [u8; TILE_MAP_SIZE],
    tile_map_1: [u8; TILE_MAP_SIZE],
    tile_data: [Tile; NUM_TILES],
    sprites: [Sprite; NUM_SPRITES],
    display: Box<dyn VideoDisplay>,
}

impl GPU {
    pub fn new(display: Box<dyn VideoDisplay>) -> GPU {
        GPU {
            current_line: 0,
            compare_line: 0,
            current_mode: Mode::OAM,
            remaining_cycles: Mode::OAM.cycles(),
            scroll_x: 0,
            scroll_y: 0,
            window_x: 0,
            window_y: 0,
            control: Control::new(),
            stat: Stat::new(),
            bg_palette: Palette::new(),
            obj_palette_0: Palette::new(),
            obj_palette_1: Palette::new(),
            tile_map_0: [0; TILE_MAP_SIZE],
            tile_map_1: [0; TILE_MAP_SIZE],
            tile_data: [Tile::new(); NUM_TILES],
            sprites: [Sprite::new(); NUM_SPRITES],
            display: display,
        }
    }

    pub fn get_control(&self) -> u8 {
        u8::from(&self.control)
    }

    pub fn set_control(&mut self, value: u8) {
        self.control = Control::from(value)
    }

    pub fn get_stat(&self) -> u8 {
        if self.control.lcd_on {
            u8::from(&self.stat)
        } else {
            STAT_UNUSED
        }
    }

    pub fn set_stat(&mut self, value: u8) {
        self.stat = Stat::from(value)
    }

    pub fn get_current_line(&self) -> u8 {
        self.current_line
    }

    pub fn reset_current_line(&mut self) {
        self.current_line = 0
    }

    pub fn get_compare_line(&self) -> u8 {
        self.compare_line
    }

    pub fn set_compare_line(&mut self, value: u8) {
        self.compare_line = value
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
        self.window_x
    }

    pub fn set_window_x(&mut self, value: u8) {
        self.window_x = value
    }

    pub fn get_window_y(&self) -> u8 {
        self.window_y
    }

    pub fn set_window_y(&mut self, value: u8) {
        self.window_y = value
    }

    pub fn get_bg_palette(&self) -> u8 {
        u8::from(&self.bg_palette)
    }

    pub fn set_bg_palette(&mut self, value: u8) {
        self.bg_palette = Palette::from(value)
    }

    pub fn get_obj_palette_0(&self) -> u8 {
        u8::from(&self.obj_palette_0)
    }

    pub fn set_obj_palette_0(&mut self, value: u8) {
        self.obj_palette_0 = Palette::from(value)
    }

    pub fn get_obj_palette_1(&self) -> u8 {
        u8::from(&self.obj_palette_1)
    }

    pub fn set_obj_palette_1(&mut self, value: u8) {
        self.obj_palette_1 = Palette::from(value)
    }

    pub fn get_tile_map_0(&self, address: u16) -> u8 {
        self.tile_map_0[address as usize]
    }

    pub fn set_tile_map_0(&mut self, address: u16, byte: u8) {
        self.tile_map_0[address as usize] = byte
    }

    pub fn get_tile_map_1(&self, address: u16) -> u8 {
        self.tile_map_1[address as usize]
    }

    pub fn set_tile_map_1(&mut self, address: u16, byte: u8) {
        self.tile_map_1[address as usize] = byte
    }

    pub fn get_tile_row(&self, address: u16) -> u8 {
        let tile = self.tile_data[(address / 16) as usize];
        tile.bytes[(address % 16) as usize]
    }

    pub fn set_tile_row(&mut self, address: u16, byte: u8) {
        let mut tile = &mut self.tile_data[(address / 16) as usize];
        tile.bytes[(address % 16) as usize] = byte;
    }

    pub fn read_oam(&self, address: u8) -> u8 {
        let sprite = self.sprites[(address / 4) as usize];
        match address % 4 {
            0 => sprite.y,
            1 => sprite.x,
            2 => sprite.tile_num,
            _ => u8::from(&sprite.flags),
        }
    }

    pub fn write_oam(&mut self, address: u8, byte: u8) {
        let mut sprite = &mut self.sprites[(address / 4) as usize];
        match address % 4 {
            0 => sprite.y = byte,
            1 => sprite.x = byte,
            2 => sprite.tile_num = byte,
            _ => sprite.flags = SpriteFlags::from(byte),
        }
    }

    pub fn emulate(&mut self, irq: &mut IRQ) {
        if !self.control.lcd_on {
            return;
        }

        self.remaining_cycles -= 1;
        if self.remaining_cycles > 0 {
            return;
        }

        match self.current_mode {
            Mode::OAM => self.switch_mode(Mode::PixelTransfer, irq),
            Mode::PixelTransfer => {
                self.draw_scanline();
                self.switch_mode(Mode::HBlank, irq);
            }
            Mode::HBlank => {
                self.current_line += 1;
                self.check_compare_line(irq);

                if self.current_line < H_SCANLINE_MAX {
                    self.switch_mode(Mode::OAM, irq);
                } else {
                    self.display.vsync();
                    self.switch_mode(Mode::VBlank, irq);
                }
            }
            Mode::VBlank => {
                self.current_line += 1;
                self.check_compare_line(irq);

                if self.current_line < H_SCANLINE_VBLANK_MAX {
                    // Reset cycles to be able to continue incrementing scanline
                    // but do not actually switch mode (no interrupts)
                    self.remaining_cycles = Mode::VBlank.cycles();
                } else {
                    self.current_line = 0;
                    self.switch_mode(Mode::OAM, irq);
                }
            }
        }
    }

    fn switch_mode(&mut self, mode: Mode, irq: &mut IRQ) {
        self.stat.mode_flag = mode.flag_bits();
        self.remaining_cycles = mode.cycles();
        self.current_mode = mode;

        match self.current_mode {
            Mode::OAM => {
                if self.stat.access_oam_interrupt {
                    irq.set_interrupt(&Interrupt::LCDC);
                }
            }
            Mode::HBlank => {
                if self.stat.hblank_interrupt {
                    irq.set_interrupt(&Interrupt::LCDC);
                }
            }
            Mode::VBlank => {
                irq.set_interrupt(&Interrupt::VBlank);
                if self.stat.vblank_interrupt {
                    irq.set_interrupt(&Interrupt::LCDC);
                }
            }
            _ => {}
        };
    }

    fn check_compare_line(&mut self, irq: &mut IRQ) {
        self.stat.line_compare = self.current_line == self.compare_line;
        if self.stat.line_compare && self.stat.line_compare_interrupt {
            irq.set_interrupt(&Interrupt::LCDC);
        }
    }

    fn draw_scanline(&mut self) {
        if self.control.bg_on {
            self.draw_tiles();
        }

        if self.control.obj_on {
            self.draw_sprites();
        }
    }

    // TODO: handle window drawing
    fn draw_tiles(&mut self) {
        let y_pos = self.current_line.wrapping_add(self.scroll_y);
        let tile_row = y_pos / 8;

        for col in 0..V_SCANLINE_MAX {
            let x_pos = col.wrapping_add(self.scroll_x);
            let tile_col = x_pos / 8;
            let tile = self.get_tile(tile_row, tile_col);

            let color = tile.get_color(y_pos % 8, x_pos % 8);
            let color = self.bg_palette.map(color);
            self.display.set_pixel(col, self.current_line, color);
        }
    }

    fn draw_sprites(&mut self) {
        let y_size: u8 = if self.control.obj_size { 16 } else { 8 };

        for &sprite in self.sprites.iter() {
            if !self.should_draw_sprite(&sprite, y_size) {
                continue;
            }

            let palette = if sprite.flags.palette {
                &self.obj_palette_1
            } else {
                &self.obj_palette_0
            };

            let sprite_y = self.current_line.wrapping_sub(sprite.get_y());
            let mut y_pos = if sprite.flags.flip_y {
                y_size - sprite_y - 1
            } else {
                sprite_y
            };

            let mut tile_num = sprite.tile_num;
            if y_pos >= 8 {
                // Go to the next tile, and normalize the y pos
                tile_num += 1;
                y_pos -= 8;
            }

            // Sprite tiles start at 0x8000, which is where tile_data is based
            let tile = self.tile_data[tile_num as usize];
            for x_offset in 0..8 {
                let x_pos = if sprite.flags.flip_x {
                    7 - x_offset
                } else {
                    x_offset
                };

                let color = tile.get_color(y_pos, x_pos);
                if color == Color::White {
                    // White is transparent for sprite drawing
                    continue;
                }

                let col = sprite.get_x() + x_offset;
                if col >= V_SCANLINE_MAX {
                    // Don't draw sprites off screen
                    continue;
                }

                let color = palette.map(color);
                self.display.set_pixel(col, self.current_line, color);
            }
        }
    }

    /// Sprites should only be drawn when they intersect with the scanline
    fn should_draw_sprite(&self, sprite: &Sprite, y_size: u8) -> bool {
        let y_pos = sprite.get_y();
        let line = self.current_line;
        return y_pos <= line && line < (y_pos + y_size);
    }

    /// Given a tile row and col, returns the tile via the proper semantics
    /// by doing a lookup for the number and then the data using LCDC register
    fn get_tile(&self, row: u8, col: u8) -> &Tile {
        // TODO: add option for choosing window tiles
        let tile_map = if self.control.bg_map {
            &self.tile_map_1
        } else {
            &self.tile_map_0
        };

        // First, look up the tile number in the mapping
        let offset = (row as usize) * 32 + (col as usize);
        let tile_num = tile_map[offset];

        // Next, use the tile number to find the corresponding data
        if self.control.bg_data {
            &self.tile_data[tile_num as usize]
        } else {
            let tile_num = tile_num as i8 as u16; // Extend the sign
            let addr = tile_num.wrapping_add(0x80) as usize;

            // (0x8800 - 0x8000) / 0x10 = 0x80
            &self.tile_data[0x80 + addr]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl GPU {
        fn test() -> GPU {
            let display = NoDisplay::new();
            GPU::new(Box::new(display))
        }
    }

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
    fn tile_get_color() {
        let mut tile = Tile::new();

        tile.bytes[3] = 0b10101110;
        tile.bytes[2] = 0b00110101;

        assert_eq!(tile.get_color(1, 0), Color::Dark);
        assert_eq!(tile.get_color(1, 1), Color::White);
        assert_eq!(tile.get_color(1, 2), Color::Black);
        assert_eq!(tile.get_color(1, 3), Color::Light);
        assert_eq!(tile.get_color(1, 4), Color::Dark);
        assert_eq!(tile.get_color(1, 5), Color::Black);
        assert_eq!(tile.get_color(1, 6), Color::Dark);
        assert_eq!(tile.get_color(1, 7), Color::Light);
    }

    #[test]
    fn gpu_bg_palette() {
        let mut gpu = GPU::test();
        let palette = Palette::from(0xFF);

        gpu.set_bg_palette(0xFF);

        assert_eq!(gpu.get_bg_palette(), 0xFF);
        assert_eq!(gpu.bg_palette, palette);
    }

    #[test]
    fn gpu_obj_palette_0() {
        let mut gpu = GPU::test();
        let palette = Palette::from(0xFF);

        gpu.set_obj_palette_0(0xFF);

        assert_eq!(gpu.get_obj_palette_0(), 0xFF);
        assert_eq!(gpu.obj_palette_0, palette);
    }

    #[test]
    fn gpu_obj_palette_1() {
        let mut gpu = GPU::test();
        let palette = Palette::from(0xFF);

        gpu.set_obj_palette_1(0xFF);

        assert_eq!(gpu.get_obj_palette_1(), 0xFF);
        assert_eq!(gpu.obj_palette_1, palette);
    }

    #[test]
    fn gpu_tile_data() {
        let mut gpu = GPU::test();

        assert_eq!(gpu.get_tile_row(0x0), 0x0);
        assert_eq!(gpu.get_tile_row(0x1), 0x0);

        gpu.set_tile_row(0x0, 0xFF);
        gpu.set_tile_row(0x1, 0xAA);

        assert_eq!(gpu.get_tile_row(0x0), 0xFF);
        assert_eq!(gpu.get_tile_row(0x1), 0xAA);

        assert_eq!(gpu.tile_data[0x0].bytes[0x0], 0xFF);
        assert_eq!(gpu.tile_data[0x0].bytes[0x1], 0xAA);
    }

    #[test]
    fn gpu_control() {
        let mut gpu = GPU::test();
        let control = Control::from(0xFF);

        gpu.set_control(0xFF);

        assert_eq!(gpu.get_control(), 0xFF);
        assert_eq!(gpu.control, control);
    }

    #[test]
    fn gpu_stat() {
        let mut gpu = GPU::test();
        let stat = Stat::from(0xFF);

        gpu.set_stat(0xFF);

        gpu.control.lcd_on = false;
        assert_eq!(gpu.get_stat(), 0x80);

        gpu.control.lcd_on = true;
        assert_eq!(gpu.get_stat(), 0xFF);
    }

    #[test]
    fn gpu_scroll() {
        let mut gpu = GPU::test();

        gpu.set_scroll_x(0x6);
        assert_eq!(gpu.get_scroll_x(), 0x6);

        gpu.set_scroll_y(0x5);
        assert_eq!(gpu.get_scroll_y(), 0x5);
    }

    #[test]
    fn gpu_current_line() {
        let mut gpu = GPU::test();

        gpu.current_line = 0x4;
        assert_eq!(gpu.get_current_line(), 0x4);

        gpu.reset_current_line();
        assert_eq!(gpu.get_current_line(), 0x0);
    }

    #[test]
    fn gpu_compare_line() {
        let mut gpu = GPU::test();

        gpu.set_compare_line(0x4);
        assert_eq!(gpu.get_compare_line(), 0x4);
    }

    #[test]
    fn gpu_emulate_state_transitions() {
        let mut gpu = GPU::test();
        let mut irq = IRQ::enabled();
        gpu.control.lcd_on = true;

        gpu.remaining_cycles = 1;
        gpu.emulate(&mut irq);

        assert_eq!(gpu.current_mode, Mode::PixelTransfer);
        assert_eq!(gpu.remaining_cycles, Mode::PixelTransfer.cycles());
        assert_eq!(gpu.stat.mode_flag, MODE_FLAG_PIXEL_TRANSFER);
        assert_eq!(irq.ack_interrupt(), None);

        gpu.remaining_cycles = 1;
        gpu.emulate(&mut irq);

        assert_eq!(gpu.current_mode, Mode::HBlank);
        assert_eq!(gpu.remaining_cycles, Mode::HBlank.cycles());
        assert_eq!(gpu.stat.mode_flag, MODE_FLAG_HBLANK);
        assert_eq!(irq.ack_interrupt(), None);

        gpu.remaining_cycles = 1;
        gpu.emulate(&mut irq);

        assert_eq!(gpu.current_mode, Mode::OAM);
        assert_eq!(gpu.remaining_cycles, Mode::OAM.cycles());
        assert_eq!(gpu.stat.mode_flag, MODE_FLAG_ACCESS_OAM);
        assert_eq!(irq.ack_interrupt(), None);

        assert_eq!(gpu.current_line, 1);
    }

    #[test]
    fn gpu_emulate_interrupts() {
        let mut gpu = GPU::test();
        let mut irq = IRQ::enabled();

        gpu.control.lcd_on = true;
        gpu.stat.access_oam_interrupt = true;
        gpu.stat.hblank_interrupt = true;
        gpu.stat.vblank_interrupt = true;

        gpu.remaining_cycles = 1;
        gpu.emulate(&mut irq);

        assert_eq!(gpu.current_mode, Mode::PixelTransfer);
        assert_eq!(irq.ack_interrupt(), None);

        gpu.remaining_cycles = 1;
        gpu.emulate(&mut irq);

        assert_eq!(gpu.current_mode, Mode::HBlank);
        assert_eq!(irq.ack_interrupt(), Some(Interrupt::LCDC.get_addr()));

        gpu.current_line = H_SCANLINE_MAX - 1;
        gpu.remaining_cycles = 1;
        gpu.emulate(&mut irq);

        assert_eq!(gpu.current_mode, Mode::VBlank);
        assert_eq!(irq.ack_interrupt(), Some(Interrupt::VBlank.get_addr()));
        assert_eq!(irq.ack_interrupt(), Some(Interrupt::LCDC.get_addr()));

        gpu.current_line = V_SCANLINE_MAX - 1;
        gpu.remaining_cycles = 1;
        gpu.emulate(&mut irq);

        assert_eq!(gpu.current_mode, Mode::OAM);
        assert_eq!(irq.ack_interrupt(), Some(Interrupt::LCDC.get_addr()));
    }

    #[test]
    fn gpu_emulate_vblank() {
        let mut gpu = GPU::test();
        let mut irq = IRQ::enabled();

        gpu.control.lcd_on = true;
        gpu.switch_mode(Mode::HBlank, &mut irq);

        gpu.current_line = H_SCANLINE_MAX - 1;
        gpu.remaining_cycles = 1;
        gpu.emulate(&mut irq);

        assert_eq!(gpu.current_mode, Mode::VBlank);
        assert_eq!(gpu.current_line, H_SCANLINE_MAX);
        assert_eq!(irq.ack_interrupt(), Some(Interrupt::VBlank.get_addr()));

        gpu.remaining_cycles = 1;
        gpu.emulate(&mut irq);

        assert_eq!(gpu.current_mode, Mode::VBlank);
        assert_eq!(gpu.current_line, H_SCANLINE_MAX + 1);
        assert_eq!(irq.ack_interrupt(), None);

        gpu.current_line = V_SCANLINE_MAX - 1;
        gpu.remaining_cycles = 1;
        gpu.emulate(&mut irq);

        assert_eq!(gpu.current_mode, Mode::OAM);
        assert_eq!(gpu.current_line, 0);
        assert_eq!(irq.ack_interrupt(), None);
    }

    #[test]
    fn gpu_emulate_compare_line() {
        let mut gpu = GPU::test();
        let mut irq = IRQ::enabled();

        gpu.control.lcd_on = true;
        gpu.compare_line = 1;
        gpu.stat.line_compare_interrupt = true;

        gpu.switch_mode(Mode::HBlank, &mut irq);
        gpu.remaining_cycles = 1;
        gpu.emulate(&mut irq);

        assert_eq!(gpu.current_line, 1);
        assert_eq!(gpu.stat.line_compare, true);
        assert_eq!(irq.ack_interrupt(), Some(Interrupt::LCDC.get_addr()));

        gpu.switch_mode(Mode::HBlank, &mut irq);
        gpu.remaining_cycles = 1;
        gpu.emulate(&mut irq);

        assert_eq!(gpu.current_line, 2);
        assert_eq!(gpu.stat.line_compare, false);
        assert_eq!(irq.ack_interrupt(), None);
    }
}
