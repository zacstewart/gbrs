use memory_map::{ReadByte, WriteByte};

enum ClockFrequency {
    Hz4096,
    Hz262144,
    Hz65536,
    Hz16384
}

pub struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
    stop: bool,
    input_clock: ClockFrequency,
    internal_clock: u8,
    sub: u8
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            stop: false,
            input_clock: ClockFrequency::Hz4096,
            internal_clock: 0,
            sub: 0
        }
    }

    pub fn step(&mut self, clock: u8) {
        self.sub = self.sub + clock;
        if self.sub > 15 {
            self.internal_clock = self.internal_clock + 1;
            self.sub = self.sub - 16;
        }

        if !self.stop {
            match self.input_clock {
                ClockFrequency::Hz4096 => {
                    if self.internal_clock >= 64 {
                        self.inc();
                    }
                }
                ClockFrequency::Hz262144 => {
                    if self.internal_clock >= 1 {
                        self.inc();
                    }
                }
                ClockFrequency::Hz65536 => {
                    if self.internal_clock >= 4 {
                        self.inc();
                    }
                }
                ClockFrequency::Hz16384 => {
                    if self.internal_clock >= 16 {
                        self.inc();
                    }
                }
            }
        }
    }

    fn inc(&mut self) {
        self.tima += 1;
        self.internal_clock = 0;
        if self.tima == 255 {
            self.tima = self.tma;
            // return INTERRUPT
            return
        }
        self.tima += 1;
    }
}

impl ReadByte for Timer {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xff04 => { self.div }
            0xff05 => { self.tima }
            0xff06 => { self.tma }
            0xff07 => {
                let mut value = 0;
                if !self.stop { value |= 0b0100; }
                value |= match self.input_clock {
                    ClockFrequency::Hz4096 => 0b00,
                    ClockFrequency::Hz262144 => 0b01,
                    ClockFrequency::Hz65536 => 0b10,
                    ClockFrequency::Hz16384 => 0b11
                };
                value
            }
            _ => { panic!("Invalid timer address: {:04x}", address); }
        }

    }
}

impl WriteByte for Timer {
    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xff04 => { self.div = 0; }
            0xff05 => { self.tima = value; }
            0xff06 => { self.tma = value; }
            0xff07 => {
                self.stop = (value & 0b0100) == 0;
                self.input_clock = match value & 0b0011 {
                    0b00 => ClockFrequency::Hz4096,
                    0b01 => ClockFrequency::Hz262144,
                    0b10 => ClockFrequency::Hz65536,
                    0b11 => ClockFrequency::Hz16384,
                    _ => { panic!("Invalid click frequency: {:04x}", value & 0b0011); }
                }
            }
            _ => { panic!("Invalid timer address: {:04x}", address); }
        }
    }
}
