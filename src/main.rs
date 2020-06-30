use std::fs;
use std::time::Instant;

use simple_logger;
use sdl2::event::Event;
use clap::{App, Arg};

mod chip8;
mod display;
mod instructions;
mod main_memory;
mod registers;
mod stack;
mod interface;

use display::Display;
use interface::AVInterface;

pub fn main() {
    let matches = App::new("yac8")
                            .version("0.1.0")
                            .author("halfhorst")
                            .about("Yet another CHIP-8 emulator")
                            .arg(Arg::with_name("program_file")
                                    .value_name("PROGRAM_FILE")
                                    .help("A CHIP-8 ROM filepath.")
                                    .takes_value(true)
                                    .required(true))
                            .arg(Arg::with_name("scan")
                                    .short("s")
                                    .long("scan")
                                    .value_name("SCAN")
                                    .help("Scan the program only, printing raw bytes and instructions.")
                                    .takes_value(false)
                                    .required(false))
                            .arg(Arg::with_name("verbose")
                                    .short("v")
                                    .long("verbose")
                                    .value_name("VERBOSE")
                                    .help("Run the VM with verbose logging to the terminal.")
                                    .takes_value(false)
                                    .required(false))
                            .arg(Arg::with_name("clock_speed")
                                    .short("c")
                                    .long("clock")
                                    .help("The clock speed to run the CPU at in hz. Defaults to 700hz.")
                                    .value_name("clock_speed")
                                    .takes_value(true)
                                    .required(false))
                            .get_matches();

    let program_file = matches.value_of("program_file").unwrap();
    let scan = matches.is_present("scan");
    let verbose = matches.is_present("verbose");
    let clock_speed = match matches.value_of("clock_speed") {
        Some(s) => {
            match s.parse::<f64>() {
                Ok(n) => n,
                Err(_) => panic!("Failed to parse clock_speed")
            }
        },
        None => 700.0
    };

    if verbose {
        simple_logger::init().unwrap();
    }

    println!("=> Booting ROM [ {} ].", program_file);
    let rom_bytes = fs::read(program_file).expect("Cannot open or read ROM file.");
    let mut machine = chip8::Chip8::new(rom_bytes, clock_speed);

    if scan {
        machine.scan_program();
        std::process::exit(0);
    }

    let mut av_interface = AVInterface::new(Display::WIDTH as u32, Display::HEIGHT as u32);

    let mut timer = Instant::now();
    loop {
        machine.cycle(timer.elapsed());
        timer = Instant::now();

        // make this a reference, no editing necessary
        av_interface.draw(&machine.display.buffer);

        av_interface.canvas.present();

        let event = av_interface.event_pump.poll_event();
        match event {
            Some(e) => {
                match e {
                    Event::KeyDown {scancode, ..} => {
                        machine.update_key(scancode.unwrap().to_string(), true)
                    },
                    Event::KeyUp {scancode, ..} => {
                        machine.update_key(scancode.unwrap().to_string(), false)
                    }
                    Event::Quit {..} => {
                        break;
                    },
                    _ => {}
                }
            }
            None => {}
        }
    }
}
