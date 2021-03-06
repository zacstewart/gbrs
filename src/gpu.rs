use memory_map::{ReadByte, WriteByte};

const BASE: u16 = 0xff40;
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const OAM_SIZE: usize = SCREEN_WIDTH;

#[derive(Debug)]
enum LineMode {
    HBlank = 0,
    VBlank = 1,
    OAMRead = 2,
    VRAMRead = 3
}

#[derive(Debug, Copy, Clone)]
enum Shade {
    White,
    LightGray,
    DarkGray,
    Black
}

impl Shade {
    fn from_u8(value: u8) -> Shade {
        match value {
            0 => Shade::White,
            1 => Shade::LightGray,
            2 => Shade::DarkGray,
            3 => Shade::Black,
            _ => { panic!("Invalid shade: {}", value); }
        }
    }

    fn to_u8(&self) -> u8 {
        match *self {
            Shade::White => 0,
            Shade::LightGray => 1,
            Shade::DarkGray => 2,
            Shade::Black => 3
        }
    }
}

pub struct GPU {
    scroll_y: u8,
    scroll_x: u8,
    current_line: u8,
    memory: [u8; 0xbf],
    clock: u16,
    vram: [u8; 8192],
    pub oam: [u8; OAM_SIZE],

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
    lyc: u8,
    line_mode: LineMode,

    window_position_y: u8,
    window_position_x: u8,

    bg_palette: (Shade, Shade, Shade, Shade),
    obj_0_palette: (Shade, Shade, Shade, Shade),
    obj_1_palette: (Shade, Shade, Shade, Shade)
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            scroll_y: 0,
            scroll_x: 0,
            current_line: 0,
            memory: [0; 0xbf],
            clock: 0,
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
            lyc: 0,
            line_mode: LineMode::OAMRead,
            window_position_y: 0,
            window_position_x: 0,
            bg_palette: (Shade::White, Shade::White, Shade::White, Shade::White),
            obj_0_palette: (Shade::White, Shade::White, Shade::White, Shade::White),
            obj_1_palette: (Shade::White, Shade::White, Shade::White, Shade::White)
        }
    }

    pub fn step(&mut self, cycles: u8) {
        self.clock = self.clock + (cycles as u16);

        match self.line_mode {
            LineMode::HBlank => {
                if self.clock >= 51 {
                    if self.current_line == 143 {
                        self.line_mode = LineMode::VBlank;
                        self.render_screen();
                    } else {
                        self.line_mode = LineMode::OAMRead;
                    }
                    self.current_line = self.current_line + 1;
                    //self.current_scan += 640;
                    self.clock = 0;
                }
            }
            LineMode::VBlank => {
                if self.clock >= 114 {
                    self.clock = 0;
                    self.current_line = self.current_line + 1;
                    if self.current_line > 153 {
                        self.current_line = 0;
                        self.line_mode = LineMode::OAMRead
                    }
                }
            }
            LineMode::OAMRead => {
                if self.clock >= 20 {
                    self.clock = 0;
                    self.line_mode = LineMode::VRAMRead;
                }
            }
            LineMode::VRAMRead => {
                if self.clock >= 43 {
                    self.clock = 0;
                    self.line_mode = LineMode::HBlank;
                    self.render_scanline();
                }
            }
        }
    }

    fn render_scanline(&mut self) {
        if !self.lcd_on { return; }

        if self.bg_display_enable {
            println!("Render scanline background");
        }

        if self.obj_display_enable {
            println!("Render scanline object");
        }
    }

    fn render_screen(&mut self) {
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
                //TODO bits 5,6
                value |= (self.bg_tile_select & 1) << 4;
                value |= (self.bg_map_select & 1) << 3;
                value |= (self.obj_size & 1) << 2;
                if self.obj_display_enable { value |= 0b00000010; }
                if self.bg_display_enable { value |= 0b00000001; }
                value
            }
            0xff41 => {
                let mut value = 0;
                value |= (self.coincidence_interrupt & 1) << 6;
                value |= (self.oam_interrupt & 1) << 5;
                value |= (self.v_blank_interrupt & 1) << 4;
                value |= (self.h_blank_interrupt & 1) << 3;
                if self.current_line == self.lyc { value |= 0b0000_0100; }
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
            0xff44 => { self.current_line } // LY
            0xff45 => { self.lyc }
            0xff47 => {
                self.bg_palette.0.to_u8() |
                    self.bg_palette.1.to_u8() << 2 |
                    self.bg_palette.2.to_u8() << 4 |
                    self.bg_palette.3.to_u8() << 6
            }
            0xff48 => {
                self.obj_0_palette.0.to_u8() |
                    self.obj_0_palette.1.to_u8() << 2 |
                    self.obj_0_palette.2.to_u8() << 4 |
                    self.obj_0_palette.3.to_u8() << 6
            }
            0xff49 => {
                self.obj_1_palette.0.to_u8() |
                    self.obj_0_palette.1.to_u8() << 2 |
                    self.obj_0_palette.2.to_u8() << 4 |
                    self.obj_0_palette.3.to_u8() << 6
            }
            0xff4a => { self.window_position_y }
            0xff4b => { self.window_position_x }
            _ => { println!("Read GPU: {:04x}", address); self.memory[(address - BASE) as usize] }
        }
    }
}

impl WriteByte for GPU {
    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x8000...0x97ff => { self.vram[(address & 0x1fff) as usize] = value }
            0x9800...0x9fff => { self.vram[(address & 0x1fff) as usize] = value }
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
            }
            0xff42 => { self.scroll_y = value; }
            0xff43 => { self.scroll_x = value; }
            0xff45 => { self.lyc = value; }
            0xff47 => {
                self.bg_palette = (
                    Shade::from_u8(value & 0b0000_0011),
                    Shade::from_u8((value & 0b0000_1100) >> 2),
                    Shade::from_u8((value & 0b0011_0000) >> 4),
                    Shade::from_u8((value & 0b1100_0000) >> 6)
                    );
            }
            0xff48 => {
                self.obj_0_palette = (
                    Shade::from_u8(value & 0b0000_0011),
                    Shade::from_u8((value & 0b0000_1100) >> 2),
                    Shade::from_u8((value & 0b0011_0000) >> 4),
                    Shade::from_u8((value & 0b1100_0000) >> 6)
                    );
            }
            0xff49 => {
                self.obj_1_palette = (
                    Shade::from_u8(value & 0b0000_0011),
                    Shade::from_u8((value & 0b0000_1100) >> 2),
                    Shade::from_u8((value & 0b0011_0000) >> 4),
                    Shade::from_u8((value & 0b1100_0000) >> 6)
                    );
            }
            0xff4a => { self.window_position_y = value; }
            0xff4b => { self.window_position_x = value; }
            _ => { println!("Write GPU: {:04x} = {:02x}", address, value); self.memory[(address - BASE) as usize] = value; }
        }
    }
}
