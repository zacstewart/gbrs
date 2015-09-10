use cartridge::Cartridge;
use std::fmt;
use gpu::GPU;
use memory_map::{ReadByte, WriteByte};

pub struct MMU {
  pub cartridge: Cartridge,
  pub working_ram: [u8; 0x2000],
  pub hram: [u8; 127],
  pub gpu: GPU
  pub ie: u8
}

impl MMU {
  pub fn new() -> MMU {
    MMU {
      cartridge: Cartridge::new(Box::new([])),
      working_ram: [0; 0x2000],
      hram: [0; 127],
      gpu: GPU::new()
      ie: 0
    }
  }

  pub fn load_cartridge(&mut self, cartridge: Cartridge) {
      self.cartridge = cartridge;
  }

  pub fn step(&mut self, clock: u16) {
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
      0x0000...0x7fff => { self.cartridge.read_byte(address) }              // ROM bank 0 & switchable [Cartridge]
      0x8000...0x9fff => { self.gpu.read_byte(address) },                   // VRAM [GPU]
      0xa000...0xbfff => { self.cartridge.read_byte(address) }              // External RAM [Cartridge]
      0xc000...0xdfff => { self.working_ram[(address & 0x1fff) as usize] }  // Working ram (WRAM)
      0xe000...0xfdff => { self.working_ram[(address & 0x1fff) as usize] }  // Shadow RAM (ECHO)
      0xfe00...0xfe9f => { self.gpu.read_byte(address) }                    // Sprite attribute table (OAM) [GPU]
      0xfea0...0xfeff => 0,                                                 // Unusable
      0xff00...0xff3f => { println!("Reading I/O: {:2x}", address); 0}      // Memory-mapped I/O
      0xff40...0xff7f => { self.gpu.read_byte(address) }                    // GPU
      0xff80...0xfffe => { self.hram[(address & 0x7f) as usize] }           // Zero-page RAM (High RAM, HRAM)
      0xffff => { self.ie }                                                 // Interrupt enable register
      _ => { panic!("Read memory out of bounds: {}", address) }
    }
  }
}

impl WriteByte for MMU {
  fn write_byte(&mut self, address: u16, value: u8) {
    //println!("Writing {:x} = {:x}", address, value);
    match address {
      0x0000...0x7fff => { self.cartridge.write_byte(address, value); },                // ROM Bank 0 & switchable [Cartridge]
      0x8000...0x9fff => { self.gpu.write_byte(address, value); }                       // VRAM [GPU]
      0xa000...0xbfff => { self.cartridge.write_byte(address, value); }                 // External RAM [Cartridge]
      0xc000...0xdfff => { self.working_ram[(address & 0x1fff) as usize] = value }      // Working RAM (WRAM)
      0xe000...0xfdff => { self.working_ram[(address & 0x1fff) as usize] = value }      // Shadow RAM (ECHO)
      0xfe00...0xfe9f => { self.gpu.write_byte(address, value) }                        // Sprite info
      0xfea0...0xfeff => { }                                                            // Unusable
      0xff00...0xff3f => { println!("Writing I/O: {:2x} = {:2x}", address, value); }    // Memory-mapped I/O
      0xff40...0xff7f => { self.gpu.write_byte(address, value) }                        // GPU
      0xff80...0xfffe => { self.hram[(address & 0x7f) as usize] = value }               // Zero-page RAM (High RAM, HRAM)
      0xffff => { self.ie = value }                                                     // Interrupt enable register
      _ => { panic!("Wrote memory out of bounds: {:2x}", address) }
    }
  }

}

impl fmt::Debug for MMU {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "<MMU>")
  }
}
