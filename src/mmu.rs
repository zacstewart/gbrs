use std::num::Num;
use std::fmt;

static MAX_ROM_SIZE: uint = 64000;

pub struct MMU {
  pub program: [u8, ..MAX_ROM_SIZE],
  pub working_ram: [u8, ..0x2000]
}

impl MMU {
  pub fn new() -> MMU {
    MMU {
      program: [0, ..MAX_ROM_SIZE],
      working_ram: [0, ..0x2000]
    }
  }

  pub fn load_rom(&mut self, rom: &[u8]) {
    for (i, b) in rom.iter().enumerate() {
      self.program[i] = b.clone()
    }
  }

  pub fn read_byte(&self, address: u16) -> u8 {
    let address = address as uint;
    match address {
      0x0000...0x3fff => self.program[address], // ROM Bank 0
      0x4000...0x7fff => self.program[address], // ROM Bank 1
      0x8000...0x9fff => 0, // GPU vram
      0xa000...0xbfff => 0, // External RAM
      0xc000...0xdfff => self.working_ram[address & 0x1fff],
      0xe000...0xfdff => self.working_ram[address & 0x1fff], // Shadow RAM
      0xfe00...0xfe9f => 0, // Sprite info
      0xfea0...0xfeff => 0,
      0xff00...0xff7f => 0, // Memory-mapped I/O
      0xff80...0xffff => 0, // Zero-page RAM
      _ => { fail!("Read memory out of bounds: {}", address) }
    }
  }

  pub fn read_word(&self, address: u16) -> u16 {
    self.read_byte(address) as u16 + self.read_byte(address + 1) as u16
  }

  pub fn write_byte(&mut self, address: u16, value: u8) {
    println!("Writing {} = {}", address, value);
    let address = address as uint;
    match address {
      0x0000...0x3fff => {}, // ROM Bank 0
      0x4000...0x7fff => {}, // ROM Bank 1
      0x8000...0x9fff => {}, // GPU vram
      0xa000...0xbfff => {}, // External RAM
      0xc000...0xdfff => self.working_ram[address & 0x1fff] = value,
      0xe000...0xfdff => self.working_ram[address & 0x1fff] = value, // Shadow RAM
      0xfe00...0xfe9f => {}, // Sprite info
      0xff00...0xff7f => {}, // Memory-mapped I/O
      0xff80...0xffff => {}, // Zero-page RAM
      _ => { fail!("Wrote memory out of bounds: {}", address) }
    }
  }
}

impl fmt::Show for MMU {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "<MMU>")
  }
}
