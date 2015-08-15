use memory_map::{ReadByte, WriteByte};

const BASE: u16 = 0xff40;

pub struct GPU {
    memory: [u8; 0xbf],
    vram: [u8; 8192],
    oam: [u8; 160],
    lcd_on: bool,
    bg_tile_select: u8,
    bg_map_select: u8,
    obj_size: u8,
    obj_display_enable: bool,
    bg_display_enable: bool
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            memory: [0; 0xbf],
            vram: [0; 8192],
            oam: [0; 160],
            lcd_on: true,
            bg_tile_select: 1,
            bg_map_select: 0,
            obj_size: 0,
            obj_display_enable: false,
            bg_display_enable: true
        }
    }
}

impl ReadByte for GPU {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x8000...0x9fff => { self.vram[(address & 0x1fff) as usize] }
            0xfe00...0xfe9f => { self.oam[(address & 0xff) as usize] }
            0xff40 => {
                let mut value: u8 = 0;
                if self.lcd_on { value |= 0b10000000; }
                value |= ((self.bg_tile_select & 1) << 4);
                value |= ((self.bg_map_select & 1) << 3);
                value |= ((self.obj_size & 1) << 2);
                if self.obj_display_enable { value |= 0b00000010; }
                if self.bg_display_enable { value |= 0b00000001; }
                value
            }
            _ => { self.memory[(address - BASE) as usize] }
        }
    }
}

impl WriteByte for GPU {
    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x8000...0x9fff => { self.vram[(address & 0x1fff) as usize] = value }
            0xfe00...0xfe9f => { self.oam[(address & 0xff) as usize] = value }
            0xff40 => {
                self.lcd_on = (value & 0b10000000) == 0b10000000;
                self.bg_tile_select = (value & 0b00010000) >> 4;
                self.bg_map_select = (value & 0b00001000) >> 3;
                self.obj_size = (value & 0b00000100) >> 2;
                self.obj_display_enable = (value & 0b00000010) == 0b000000010;
                self.bg_display_enable = (value & 0b00000001) == 0b00000001;
            }
        }
    }
}
