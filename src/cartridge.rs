use std::fs::File;
use std::io::Read;
use memory_map::ReadByte;

pub struct Cartridge {
    rom: Box<[u8]>
}

impl Cartridge {
    pub fn new(rom: Box<[u8]>) -> Cartridge {
        Cartridge {
            rom: rom
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
        self.rom[address as usize]
    }
}
