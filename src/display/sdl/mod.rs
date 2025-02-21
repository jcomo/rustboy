extern crate sdl2;

use std::process;
use std::time::Duration;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color as SDLColor;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;

use crate::gameboy::clock::WallClock;
use crate::gameboy::display::VideoDisplay;
use crate::gameboy::Button;
use crate::gameboy::Color;
use crate::gameboy::GameBoy;

const SCREEN_WIDTH: u32 = 160;
const SCREEN_HEIGHT: u32 = 144;

pub struct SDLFrontend {}

impl SDLFrontend {
    pub fn run(cartridge: &Vec<u8>, display_scale: u32) {
        let sdl_context = sdl2::init().unwrap();
        let mut controller = SDLController::new(&sdl_context);

        let clock = WallClock::z80();
        let display = SDLDisplay::new(display_scale, &sdl_context);
        let mut gameboy = GameBoy::new(&cartridge, Box::new(clock), Box::new(display));

        loop {
            controller.process_input(&mut gameboy);
            gameboy.step();
        }
    }
}

struct SDLController {
    event_pump: EventPump,
    last_processed: Instant,
    processing_delay: Duration,
}

impl SDLController {
    fn new(sdl_context: &Sdl) -> SDLController {
        SDLController {
            event_pump: sdl_context.event_pump().unwrap(),
            last_processed: Instant::now(),
            processing_delay: Duration::from_millis(3),
        }
    }

    fn process_input(&mut self, gameboy: &mut GameBoy) {
        if self.ready_to_process() {
            self.pump_events(gameboy);
            self.reset();
        }
    }

    fn ready_to_process(&self) -> bool {
        self.last_processed.elapsed() > self.processing_delay
    }

    fn reset(&mut self) {
        self.last_processed = Instant::now();
    }

    fn pump_events(&mut self, gameboy: &mut GameBoy) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => process::exit(0),
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    SDLController::key_to_button(key).map(|btn| {
                        gameboy.button_down(btn);
                    });
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    SDLController::key_to_button(key).map(|btn| {
                        gameboy.button_up(btn);
                    });
                }
                _ => (),
            }
        }
    }

    fn key_to_button(key: Keycode) -> Option<Button> {
        match key {
            Keycode::W => Some(Button::Up),
            Keycode::A => Some(Button::Left),
            Keycode::S => Some(Button::Down),
            Keycode::D => Some(Button::Right),
            Keycode::X => Some(Button::Start),
            Keycode::Z => Some(Button::Select),
            Keycode::O => Some(Button::A),
            Keycode::J => Some(Button::B),
            _ => None,
        }
    }
}

struct SDLDisplay {
    scale: u32,
    canvas: Canvas<Window>,
}

impl SDLDisplay {
    fn new(scale: u32, sdl_context: &Sdl) -> SDLDisplay {
        let width = SCREEN_WIDTH * scale;
        let height = SCREEN_HEIGHT * scale;

        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("RustBoy", width, height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        SDLDisplay {
            scale: scale,
            canvas: canvas,
        }
    }
}

impl VideoDisplay for SDLDisplay {
    fn set_pixel(&mut self, x: u8, y: u8, color: Color) {
        let x_pos = x as i32 * self.scale as i32;
        let y_pos = y as i32 * self.scale as i32;

        let pixel = Rect::new(x_pos, y_pos, self.scale, self.scale);
        let sdl_color = match color {
            Color::White => SDLColor::RGB(0x9b, 0xbc, 0x0f),
            Color::Light => SDLColor::RGB(0x8b, 0xac, 0x0f),
            Color::Dark => SDLColor::RGB(0x30, 0x62, 0x30),
            Color::Black => SDLColor::RGB(0x0f, 0x38, 0x0f),
        };

        self.canvas.set_draw_color(sdl_color);
        self.canvas.fill_rect(pixel).unwrap();
    }

    fn vsync(&mut self) {
        self.canvas.present();
    }
}
