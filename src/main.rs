mod bits;
mod gameboy;

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

    let display = Box::new(NoDisplay::new());
    GameBoy::load(&data, display).run();
}
