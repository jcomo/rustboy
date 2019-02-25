mod bits;
mod display;
mod gameboy;

use crate::display::SDLDisplay;
use crate::gameboy::GameBoy;
use crate::gameboy::NoDisplay;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: rustboy [rom]");
        process::exit(1);
    }

    let filename = &args[1];
    let data = fs::read(filename).unwrap_or_else(|e| {
        println!("error: {}", e);
        process::exit(1);
    });

    let mut display = Box::new(SDLDisplay::new(2));
    let mut gameboy = GameBoy::new(&data, display);

    println!("[start] RustBoy");
    loop {
        gameboy.step();
    }
}
