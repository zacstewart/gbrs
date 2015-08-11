pub trait ReadByte {
    fn read_byte(&self, address: u16) -> u8;
}

pub trait WriteByte {
    fn write_byte(&mut self, address: u16, value: u8);
}
