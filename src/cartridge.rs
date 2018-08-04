use std::fs::File;
use std::io::Read;
use memory_map::{ReadByte, WriteByte};

pub struct Cartridge {
    rom: Box<[u8]>,
    ram: Box<[u8]>,
    ram_enabled: bool
}

impl Cartridge {
    pub fn new(rom: Box<[u8]>) -> Cartridge {
        Cartridge {
            rom: rom,
            ram: Box::new([0; 0x1fff]),
            ram_enabled: false
        }
    }

    pub fn load(filename: &str) -> Cartridge {
        let mut data = vec!();
        match File::open(filename).unwrap().read_to_end(&mut data) {
            Ok(length) => {
                Cartridge::new(data.into_boxed_slice())
            },
            _ => panic!("Failed to read ROM.")
        }
    }

    pub fn size(&self) -> usize {
        self.rom.len()
    }
}

impl ReadByte for Cartridge {
    fn read_byte(&self, address: u16) -> u8 {
        //println!("Reading cart: {:2x}", address);
        match address {
            0x0000...0xff => {
                match self.rom.get(address as usize) {
                    Some(&value) => value,
                    None => 0
                }
            }
            0xa000...0xbfff => {
                if self.ram_enabled {
                    self.ram[(address - 0xa000) as usize]
                } else {
                    0
                }
            }
            _ => {
                match self.rom.get(address as usize) {
                    Some(&value) => value,
                    None => 0
                }
            }
        }
    }
}

impl WriteByte for Cartridge {
    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000...0x1fff => { self.ram_enabled = (value & 0xff) == 0x0a; }
            0xa000...0xbfff => {
                if self.ram_enabled {
                    self.ram[(address - 0xa000) as usize] = value;
                }
            }
            _ => { println!("Write to Cart: {:2x} = {:2x}", address, value); }
        }
    }
}
