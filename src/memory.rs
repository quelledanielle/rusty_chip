const MAX_SIZE: usize = 0x1000;
const FONT_RANGE: Range<Address> = 0x0..0x200;
const ROM_RANGE: Range<Address> = 0x200..0xEA0;
const STACK_RANGE: Range<Address> = 0xEA0..0xF00;
const DISPLAY_RANGE: Range<Address> = 0xF00..MAX_SIZE;

use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::{Index, IndexMut, Range};
use io::font;

type Address = usize;
type Byte = u8;

pub struct Memory {
    mem: [Byte; MAX_SIZE]
}

impl Memory {
    pub const ROM_RANGE: Range<Address> = ROM_RANGE;
    pub const STACK_RANGE: Range<Address> = STACK_RANGE;

    pub fn new(rom: &Vec<Byte>) -> Memory {
        Memory {
            mem: Memory::init_mem(&rom)
        }
    }

    fn init_mem(rom: &Vec<Byte>) -> [Byte; MAX_SIZE] {
        let mut mem = [0x0; MAX_SIZE];

        Memory::load_font(&mut mem);
        Memory::load_rom(&mut mem, &rom);

        mem
    }

    fn load_font(mem: &mut [Byte]) {
        for i in 0..font::FONT_SET.len() {
            for j in 0..font::SPRITE_LEN {
                mem[i * font::SPRITE_LEN + FONT_RANGE.start + j] = font::FONT_SET[i][j];
            }
        }
    }

    fn load_rom(mem: &mut [Byte], rom: &Vec<Byte>) {
        for i in 0..rom.len() {
            mem[i + ROM_RANGE.start] = rom[i];
        }
    }
}

impl Index<Address> for Memory {
    type Output = Byte;

    fn index(&self, addr: Address) -> &Self::Output {
        &self.mem[addr]
    }
}

impl IndexMut<Address> for Memory {
    fn index_mut(&mut self, addr: Address) -> &mut Byte {
        &mut self.mem[addr]
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let hex_bytes = self.mem.iter()
            .map(|byte| format!("{:02x}", &byte));

        let lines = hex_bytes.enumerate()
            .fold(String::new(), |mut acc, (i, hex_byte)| {
                if i != 0 {
                    if i % 2 == 0 { acc.push_str(" "); }
                    if i % 16 == 0 { acc.push_str("\n"); }
                }

                acc.push_str(&hex_byte);
                acc
            });

        write!(f, "{}", lines)
    }
}

impl Debug for Memory {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self)
    }
}
