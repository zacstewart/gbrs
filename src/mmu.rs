use std::fs::File;
use std::io::Read;
use std::fmt;

pub struct MMU {
  pub program: Box<[u8]>,
  pub working_ram: [u8; 0x2000]
}

impl MMU {
  pub fn new() -> MMU {
    MMU {
      program: Box::new([]),
      working_ram: [0; 0x2000]
    }
  }

  pub fn load_rom(&mut self, filename: &str) {
    let mut data = vec!();
    match File::open(filename).unwrap().read_to_end(&mut data) {
      Ok(length) => {
        self.program = data.into_boxed_slice();
      },
      _ => panic!("Failed to read ROM.")
    }
  }

  pub fn read_byte(&self, address: u16) -> u8 {
    let address = address as usize;
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
      _ => { panic!("Read memory out of bounds: {}", address) }
    }
  }

  pub fn read_word(&self, address: u16) -> u16 {
    let ls = self.read_byte(address) as u16;
    let ms = (self.read_byte(address + 1) as u16) << 8;
    ms | ls
  }

  pub fn write_byte(&mut self, address: u16, value: u8) {
    println!("Writing {:x} = {:x}", address, value);
    let address = address as usize;
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
      _ => { panic!("Wrote memory out of bounds: {}", address) }
    }
  }

  pub fn write_word(&mut self, address: u16, value: u16) {
    let upper = ((value & 0xff00) >> 8) as u8;
    let lower = (value & 0xff) as u8;
    self.write_byte(address, upper);
    self.write_byte(address, lower);
  }
}

impl fmt::Debug for MMU {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "<MMU>")
  }
}
