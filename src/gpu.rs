use memory_map::{ReadByte, WriteByte};

const BASE: u16 = 0xff40;

enum LineMode {
    HBlank = 0,
    VBlank = 1,
    OAMRead = 2,
    VRAMRead = 3
}

pub struct GPU {
    scroll_y: u8,
    scroll_x: u8,
    current_line: u8,
    memory: [u8; 0xbf],
    vram: [u8; 8192],
    oam: [u8; 160],

    lcd_on: bool,               // LCDC
    bg_tile_select: u8,
    bg_map_select: u8,
    obj_size: u8,
    obj_display_enable: bool,
    bg_display_enable: bool,

    coincidence_interrupt: u8,  // STAT
    oam_interrupt: u8,
    v_blank_interrupt: u8,
    h_blank_interrupt: u8,
    coincidence_flag: u8,
    line_mode: LineMode
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            scroll_y: 0,
            scroll_x: 0,
            current_line: 0,
            memory: [0; 0xbf],
            vram: [0; 8192],
            oam: [0; 160],
            lcd_on: true,
            bg_tile_select: 1,
            bg_map_select: 0,
            obj_size: 0,
            obj_display_enable: false,
            bg_display_enable: true,
            coincidence_interrupt: 0,
            oam_interrupt: 0,
            v_blank_interrupt: 0,
            h_blank_interrupt: 0,
            coincidence_flag: 0,
            line_mode: LineMode::OAMRead,
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
            0xff41 => {
                let mut value = 0;
                value |= ((self.coincidence_interrupt & 1) << 6);
                value |= ((self.oam_interrupt & 1) << 5);
                value |= ((self.v_blank_interrupt & 1) << 4);
                value |= ((self.h_blank_interrupt & 1) << 3);
                value |= ((self.coincidence_flag & 1) << 2);
                value |= match self.line_mode {
                    LineMode::HBlank => 0,
                    LineMode::VBlank => 1,
                    LineMode::OAMRead => 2,
                    LineMode::VRAMRead => 3
                };
                value
            }
            0xff42 => { self.scroll_y }
            0xff43 => { self.scroll_x }
            0xff44 => { self.current_line }
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
            0xff41 => {
                self.coincidence_interrupt = (value & 0b0100_0000) >> 6;
                self.oam_interrupt = (value & 0b0010_0000) >> 5;
                self.v_blank_interrupt = (value & 0b0001_0000) >> 4;
                self.h_blank_interrupt = (value & 0b0000_1000) >> 3;
                self.coincidence_flag = (value & 0b0000_0100) >> 2;
                self.line_mode = match (value & 0b0000_0011) {
                    0 => LineMode::HBlank,
                    1 => LineMode::VBlank,
                    2 => LineMode::OAMRead,
                    3 => LineMode::VRAMRead,
                    _ => panic!("Invalid line mode: {:2x}", value)
                };
            }
            0xff42 => { self.scroll_y = value; }
            0xff43 => { self.scroll_x = value; }
        }
    }
}
