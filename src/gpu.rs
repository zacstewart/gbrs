use memory_map::{ReadByte, WriteByte};

pub struct GPU {
    current_line: u8
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            current_line: 0
        }
    }
}

impl ReadByte for GPU {
    fn read_byte(&self, address: u16) -> u8 {
        0
    }
}

impl WriteByte for GPU {
    fn write_byte(&mut self, address: u16, value: u8) {
        println!("Write to GPU: {} = {}", address, value);
    }
}
