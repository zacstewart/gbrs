use memory_map::{ReadByte, WriteByte};

pub struct Joypad {
    select_button_keys: bool,
    select_directional_keys: bool,
    down: bool,
    up: bool,
    left: bool,
    right: bool,
    start: bool,
    select: bool,
    b: bool,
    a: bool
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            select_button_keys: false,
            select_directional_keys: false,
            down: false,
            up: false,
            left: false,
            right: false,
            start: false,
            select: false,
            b: false,
            a: false
        }
    }
}

impl ReadByte for Joypad {
    fn read_byte(&self, _address: u16) -> u8 {
        let mut value = 0;
        if self.select_button_keys { value |= 0b0010_0000; }
        if self.select_directional_keys { value |= 0b0001_000; }
        if self.down || self.start { value |= 0b0000_1000; }
        if self.up || self.select { value |= 0b0000_0100; }
        if self.left || self.b { value |= 0b0000_0010; }
        if self.right || self.b { value |= 0b0000_0001; }
        !value
    }
}

impl WriteByte for Joypad {
    fn write_byte(&mut self, _address: u16, value: u8) {
        let value = !value;
        self.select_button_keys = value & 0b0010_0000 == 0b0010_0000;
        self.select_directional_keys = value & 0b0001_0000 == 0b0001_0000;
    }
}
