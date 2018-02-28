#[allow(dead_code)]

mod cpu;
mod memory;
mod timer;
mod io;

use std::io::{Error, Read};
use std::fs::File;
use std::path::Path;

use cpu::Cpu;
use memory::Memory;
use io::lcd::Lcd;

fn main() {
    let rom = read_bytes("rom/logo.ch8").expect("Unable to load ROM");
    let mut lcd = Lcd::new();
    let mut memory = Memory::new(&rom);
    let mut cpu = Cpu::new(&mut memory, &mut lcd);

    loop {
        if cpu.exit { break; }
        cpu.step();
    }
}

fn read_bytes(filename: &str) -> Result<Vec<u8>, Error> {
    let path = Path::new(filename);
    let mut file = File::open(path).expect("Unable to open file");
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(e)
    }
}
