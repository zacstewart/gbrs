use cartridge::Cartridge;
use std::fmt;
use gpu;
use memory_map::{ReadByte, WriteByte};
use joypad;
use timer;

pub struct MMU {
    pub cartridge: Cartridge,
    pub working_ram: [u8; 0x2000],
    pub hram: [u8; 127],
    pub gpu: gpu::GPU,
    pub joypad: joypad::Joypad,
    pub timer: timer::Timer,
    pub ie: u8,
    pub interrupt_flag: u8
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            cartridge: Cartridge::new(Box::new([])),
            working_ram: [0; 0x2000],
            hram: [0; 127],
            gpu: gpu::GPU::new(),
            joypad: joypad::Joypad::new(),
            timer: timer::Timer::new(),
            ie: 0,
            interrupt_flag: 0
        }
    }

    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = cartridge;
    }

    pub fn step(&mut self, clock: u8) {
        self.gpu.step(clock);
        self.timer.step(clock);
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

    pub fn leave_bios(&mut self) {
        self.cartridge.leave_bios();
    }
}

impl ReadByte for MMU {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000...0x7fff => { self.cartridge.read_byte(address) }              // ROM bank 0 & switchable [Cartridge]
            0x8000...0x9fff => { self.gpu.read_byte(address) }                    // VRAM [GPU]
            0xa000...0xbfff => { self.cartridge.read_byte(address) }              // External RAM [Cartridge]
            0xc000...0xdfff => { self.working_ram[(address & 0x1fff) as usize] }  // Working ram (WRAM)
            0xe000...0xfdff => { self.working_ram[(address & 0x1fff) as usize] }  // Shadow RAM (ECHO)
            0xfe00...0xfe9f => { self.gpu.read_byte(address) }                    // Sprite attribute table (OAM) [GPU]
            0xfea0...0xfeff => { 0 }                                              // Unusable
            0xff00          => { self.joypad.read_byte(address) }                 // P1
            0xff01...0xff02 => { println!("Serial: {:04x}", address); 0 }         // Serial data transfer
            0xff03          => { println!("Unknown: {:04x}", address); 0 }
            0xff04...0xff07 => { self.timer.read_byte(address) }                  // Timer and divider
            0xff08...0xff0e => { println!("Reading I/O: {:2x}", address); 0}      // Memory-mapped I/O
            0xff0f => { self.interrupt_flag }
            0xff10...0xff3f => { 0 } // Sound
            0xff40...0xff4b => { self.gpu.read_byte(address) }                    // GPU
            0xff4c...0xff7f => { 0 }                                              // Unusable
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
            0x0000...0x7fff => { self.cartridge.write_byte(address, value); }                 // ROM Bank 0 & switchable [Cartridge]
            0x8000...0x9fff => { self.gpu.write_byte(address, value); }                       // VRAM [GPU]
            0xa000...0xbfff => { self.cartridge.write_byte(address, value); }                 // External RAM [Cartridge]
            0xc000...0xdfff => { self.working_ram[(address & 0x1fff) as usize] = value; }     // Working RAM (WRAM)
            0xe000...0xfdff => { self.working_ram[(address & 0x1fff) as usize] = value; }     // Shadow RAM (ECHO)
            0xfe00...0xfe9f => { self.gpu.write_byte(address, value); }                       // Sprite info
            0xfea0...0xfeff => { }                                                            // Unusable
            0xff00          => { self.joypad.write_byte(address, value); }                    // P1
            0xff01...0xff02 => { println!("Serial: {:04x} = {:02x}", address, value); }       // Serial data transfer
            0xff03          => { println!("Unknown: {:04x} = {:02x}", address, value); }
            0xff04...0xff07 => { self.timer.write_byte(address, value); }                     // Timer and divider
            0xff08...0xff0e => { println!("Writing I/O: {:2x} = {:2x}", address, value); }    // Memory-mapped I/O
            0xff0f => { self.interrupt_flag = value; }
            0xff10...0xff3f => { } // Sound
            0xff46 => { // DMA
                // I'd prefer this be in the GPU implementation
                // but it's difficult (impossible?) to let the GPU have a
                // ref to the MMU.
                let start_address = (value as u16) << 8;
                for i in 0..gpu::OAM_SIZE {
                    let value = self.read_byte(start_address + (i as u16));
                    self.gpu.oam[i] = value;
                }
            }
            0xff40...0xff4b => { self.gpu.write_byte(address, value); }                       // GPU
            0xff4c...0xff7f => { }                                                            // Unusable
            0xff80...0xfffe => { self.hram[(address & 0x7f) as usize] = value; }              // Zero-page RAM (High RAM, HRAM)
            0xffff => { self.ie = value; }                                                    // Interrupt enable register
            _ => { panic!("Wrote memory out of bounds: {:2x}", address); }
        }
    }

}

impl fmt::Debug for MMU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<MMU>")
    }
}
