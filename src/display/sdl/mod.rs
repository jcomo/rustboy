extern crate sdl2;

use std::process;
use std::thread::sleep;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color as SDLColor;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::gameboy::Color;
use crate::gameboy::VideoDisplay;

const SCREEN_WIDTH: u32 = 160;
const SCREEN_HEIGHT: u32 = 144;

pub struct SDLDisplay {
    scale: u32,
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

impl SDLDisplay {
    pub fn new(scale: u32) -> SDLDisplay {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let width = SCREEN_WIDTH * scale;
        let height = SCREEN_HEIGHT * scale;
        let window = video_subsystem
            .window("RustBoy", width, height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        SDLDisplay {
            scale: scale,
            canvas: canvas,
            event_pump: event_pump,
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
        self.canvas.fill_rect(pixel);
    }

    fn vsync(&mut self) {
        self.canvas.present();
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => process::exit(0),
                _ => println!("{:?}", event),
            }
        }
    }
}
