mod cpu;

use std::env;
use std::io::Read;
use std::fs::File;

fn usage() {
    println!("Usage: chip8 <program>");
}

fn open_program(path: std::string::String) -> Vec<u8> {
    let mut program_file = File::open(path).unwrap();
    let mut program_data: Vec<u8> = Vec::new();
    match program_file.read_to_end(&mut program_data) {
        Ok(bytes) => println!("Read {} bytes from program file.", bytes),
        Err(_) => panic!("Problem reading from program file.")
    };
    program_data
}

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        usage();
        return;
    }

    let program_path = args.nth(1).unwrap();
    println!("Loading program file: {}", program_path);
    let data = open_program(program_path);


    let mut cpu = cpu::CPU::new();
    cpu.load_program(data);
    loop {
        cpu.cycle();
    }
}
