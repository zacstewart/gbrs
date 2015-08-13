use cartridge::Cartridge;
use std::fmt;
use gpu::GPU;
use memory_map::{ReadByte, WriteByte};

pub struct MMU {
  pub cartridge: Cartridge,
  pub working_ram: [u8; 0x2000],
  pub hram: [u8; 127],
  pub gpu: GPU
}

impl MMU {
  pub fn new() -> MMU {
    MMU {
      cartridge: Cartridge::new(Box::new([])),
      working_ram: [0; 0x2000],
      hram: [0; 127],
      gpu: GPU::new()
    }
  }

  pub fn load_cartridge(&mut self, cartridge: Cartridge) {
      self.cartridge = cartridge;
  }

  pub fn read_word(&self, address: u16) -> u16 {
    let ls = self.read_byte(address) as u16;
    let ms = (self.read_byte(address + 1) as u16) << 8;
    ms | ls
  }

  pub fn write_word(&mut self, address: u16, value: u16) {
    let upper = ((value & 0xff00) >> 8) as u8;
    let lower = (value & 0xff) as u8;
    self.write_byte(address, upper);
    self.write_byte(address, lower);
  }
}

impl ReadByte for MMU {
  fn read_byte(&self, address: u16) -> u8 {
    match address {
      0xa000...0xbfff => 0, // External RAM
      0xc000...0xdfff => self.working_ram[(address & 0x1fff) as usize],
      0xe000...0xfdff => self.working_ram[(address & 0x1fff) as usize], // Shadow RAM
      0xfe00...0xfe9f => 0, // Sprite info
      0xfea0...0xfeff => 0,
      0xff00...0xff3f => { println!("Reading I/O: {}", address); 0} // Memory-mapped I/O
      0xff40...0xff7f => { self.gpu.read_byte(address) } // GPU
      0x0000...0x7fff => { self.cartridge.read_byte(address) }              // ROM bank 0 & switchable [Cartridge]
      0x8000...0x9fff => { self.gpu.read_byte(address) },                   // VRAM [GPU]
      0xff80...0xfffe => { self.hram[(address & 0x7f) as usize] }           // Zero-page RAM (High RAM, HRAM)
      0xffff => { 0 }                                                       // Interrupt enable register
      _ => { panic!("Read memory out of bounds: {}", address) }
    }
  }
}

impl WriteByte for MMU {
  fn write_byte(&mut self, address: u16, value: u8) {
    //println!("Writing {:x} = {:x}", address, value);
    match address {
      0xa000...0xbfff => {}, // External RAM
      0xc000...0xdfff => self.working_ram[(address & 0x1fff) as usize] = value,
      0xe000...0xfdff => self.working_ram[(address & 0x1fff) as usize] = value, // Shadow RAM
      0xfe00...0xfe9f => {}, // Sprite info
      0xff00...0xff3f => { println!("Writing I/O: {} = {}", address, value) } // Memory-mapped I/O
      0xff40...0xff7f => { self.gpu.write_byte(address, value) } // GPU
      _ => { panic!("Wrote memory out of bounds: {}", address) }
      0x0000...0x7fff => { self.cartridge.write_byte(address, value); },                // ROM Bank 0 & switchable [Cartridge]
      0x8000...0x9fff => { self.gpu.write_byte(address, value); }                       // VRAM [GPU]
      0xff80...0xfffe => { self.hram[(address & 0x7f) as usize] = value }               // Zero-page RAM (High RAM, HRAM)
      0xffff => { println!("Write to interrupt enable register: {:2x} = {:2x}", address, value) } // Interrupt enable register
    }
  }

}

impl fmt::Debug for MMU {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "<MMU>")
  }
}
