use std::fs::File;
use std::io::Read;

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
          Ok(_) => {
              Cartridge::new(data.into_boxed_slice())
          },
          _ => panic!("Failed to read ROM.")
        }
    }

    pub fn read(&self, address: usize) -> u8 {
        self.rom[address]
    }
}
