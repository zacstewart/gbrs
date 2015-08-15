use memory_map::{ReadByte, WriteByte};

pub struct GPU {
    memory: [u8; 0xbf],
    vram: [u8; 8192],
    oam: [u8; 160],
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            memory: [0; 0xbf],
            vram: [0; 8192],
            oam: [0; 160],
        }
    }
}

impl ReadByte for GPU {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x8000...0x9fff => { self.vram[(address & 0x1fff) as usize] }
            0xfe00...0xfe9f => { self.oam[(address & 0xff) as usize] }
            _ => { self.memory[(address - BASE) as usize] }
        }
    }
}

impl WriteByte for GPU {
    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x8000...0x9fff => { self.vram[(address & 0x1fff) as usize] = value }
            0xfe00...0xfe9f => { self.oam[(address & 0xff) as usize] = value }
        }
    }
}
