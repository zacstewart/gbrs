use mmu::MMU;

enum Data {
  Byte(u8),
  Word(u16),
}

trait AddressingMode {
  fn load(&self, cpu: &mut CPU) -> Data;
  fn store(&self, cpu: &mut CPU, value: Data);
}

struct ImmediateAddressingMode;
impl AddressingMode for ImmediateAddressingMode {
  fn load(&self, cpu: &mut CPU) -> Data {
    Byte(cpu.take_byte())
  }
  fn store(&self, _: &mut CPU, _: Data) {
    fail!("Can't write to ROM!")
  }
}

struct ImmediateWordAddressingMode;
impl AddressingMode for ImmediateWordAddressingMode {
  fn load(&self, cpu: &mut CPU) -> Data {
    Word(cpu.take_byte() as u16 + cpu.take_byte() as u16)
  }
  fn store(&self, _: &mut CPU, _: Data) {
    fail!("Can't write to ROM!")
  }
}

struct MemoryAddressingMode {
  address: u16
}

impl AddressingMode for MemoryAddressingMode {
  fn load(&self, cpu: &mut CPU) -> Data {
    Byte(cpu.mmu.read_byte(self.address))
  }

  fn store(&self, cpu: &mut CPU, value: Data) {
    match value {
      Byte(b) => cpu.mmu.write_byte(self.address, b),
      _ => {}
    }
  }
}

#[deriving(Show)]
pub struct Clock {
  m: u16,
  t: u16
}

impl Clock {
  fn new() -> Clock {
    Clock {
      m: 0,
      t: 0
    }
  }
}

#[deriving(Show)]
pub struct Flags {
  z: bool,
  n: bool,
  h: bool,
  c: bool
}

impl Flags {
  fn new() -> Flags {
    Flags {
      z: false,
      n: false,
      h: false,
      c: false
    }
  }
}

#[deriving(Show)]
pub struct CPU {
  mmu: MMU,
  clock: Clock,

  pc: u16, // Program Counter
  sp: u16, // Stack pointer

  // Registers
  a: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  h: u8,
  l: u8,

  // Clock
  m: u16,
  t: u16,

  flags: Flags
}

impl CPU {
  pub fn new(mmu: MMU) -> CPU {
    let clock = Clock::new();
    let flags = Flags::new();
    CPU {
      mmu: mmu,
      clock: clock,
      pc: 0,
      sp: 0,
      a: 0,
      b: 0,
      c: 0,
      d: 0,
      e: 0,
      h: 0,
      l: 0,
      m: 0,
      t: 0,
      flags: flags
    }
  }

  pub fn execute(&mut self, op: u8) {
    match op {
      0x00 => self.nop(),
      0x01 => { let v = self.immediate_word(); self.ld_bc(v); },
      0x02 => { let am = self.address_bc(); self.ld_mem_a(am); },
      0x03 => self.inc_bc(),
      0x04 => self.inc_b(),
      0x05 => self.dec_b(),
      0x06 => { let v = self.immediate(); self.ld_b(v); },
      0x07 => self.rlca(),
      0x08 => { let am = self.immediate_word(); self.ld_mem_sp(am); },
      0x09 => self.add_hl_bc(),
      0x0a => { let am = self.address_bc(); self.ld_a(am); },
      0x0b => self.dec_bc(),
      0x0c => self.inc_c(),
      0x0d => self.dec_c(),
      0x0e => { let am = self.immediate(); self.ld_c(am); },
      0x0f => self.rrca(),
      0x10 => self.stop(),
      0x11 => { let am = self.immediate_word(); self.ld_de(am) },
      0x12 => { let am = self.address_de(); self.ld_mem_a(am); },
      0x13 => self.inc_de(),
      0x14 => self.inc_d(),
      0x15 => self.dec_d(),
      0x16 => { let am = self.immediate(); self.ld_d(am); },
      0x17 => self.rla(),
      0x18 => { let v = self.immediate(); self.jr(v) },
      0x19 => self.add_hl_de(),
      0x1a => { let am = self.address_de(); self.ld_a(am); },
      0x1b => self.dec_de(),
      0x1c => self.inc_e(),
      0x1d => self.dec_e(),
      0x1e => { let am = self.immediate(); self.ld_e(am); },
      0x1f => self.rra(),
      0x20 => { let v = self.immediate(); self.jr_nz(v); },
      0x21 => { let am = self.immediate_word(); self.ld_hl(am); },
      0x23 => self.inc_hl(),
      0x24 => self.inc_h(),
      0x26 => { let am = self.immediate(); self.ld_h(am); },
      0x28 => { let v = self.immediate(); self.jr_z(v); },
      0x29 => self.add_hl_hl(),
      0x2a => { let am = self.address_hli(); self.ld_a(am); },
      0x2c => self.inc_l(),
      0x2d => self.dec_l(),
      0x2e => { let am = self.immediate(); self.ld_l(am); },
      0x30 => { let v = self.immediate(); self.jr_nc(v); },
      0x31 => { let am = self.immediate_word(); self.ld_sp(am); },
      0x33 => self.inc_sp(),
      0x38 => { let v = self.immediate(); self.jr_c(v); },
      0x39 => self.add_hl_sp(),
      0x3a => { let am = self.address_hld(); self.ld_a(am); },
      0x3c => self.inc_a(),
      0x3d => self.dec_a(),
      0x3e => { let am = self.immediate(); self.ld_a(am); },
      0x76 => self.halt(),
      _ => println!("{}", self)
    }
    self.clock.m += self.m;
  }

  pub fn take_byte(&mut self) -> u8 {
    let immediate = self.mmu.read_byte(self.pc);
    self.pc += 1;
    return immediate;
  }

  // Addressing

  fn immediate(&mut self) -> ImmediateAddressingMode {
    ImmediateAddressingMode
  }

  fn immediate_word(&mut self) -> ImmediateWordAddressingMode {
    ImmediateWordAddressingMode
  }

  fn address(&mut self, address: u16) -> MemoryAddressingMode {
    MemoryAddressingMode { address: address }
  }

  fn address_bc(&mut self) -> MemoryAddressingMode {
    let address = (self.b as u16 << 8) + self.c as u16;
    self.address(address)
  }

  fn address_de(&mut self) -> MemoryAddressingMode {
    let address = (self.d as u16 << 8) + self.e as u16;
    self.address(address)
  }

  fn address_hl(&mut self) -> MemoryAddressingMode {
    let address = (self.h as u16 << 8) + self.l as u16;
    self.address(address)
  }

  fn address_hli(&mut self) -> MemoryAddressingMode {
    let address = (self.h as u16 << 8) + self.l as u16;
    self.inc_hl();
    self.address(address)
  }

  fn address_hld(&mut self) -> MemoryAddressingMode {
    let address = (self.h as u16 << 8) + self.l as u16;
    self.dec_hl();
    self.address(address)
  }

  // Loads

  fn ld_b<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => self.b = byte,
      _ => {}
    }
  }

  fn ld_c<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => self.c = byte,
      _ => {}
    }
  }

  fn ld_d<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => self.d = byte,
      _ => {}
    }
  }

  fn ld_e<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => self.e = byte,
      _ => {}
    }
  }

  fn ld_h<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => self.h = byte,
      _ => {}
    }
  }

  fn ld_l<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => self.l = byte,
      _ => {}
    }
  }

  fn ld_a<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => self.a = byte,
      _ => {}
    }
  }

  fn ld_sp<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Word(word) => self.sp = word,
      _ => {}
    }
  }

  fn ld_de<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Word(word) => {
        self.e = get_upper_bytes(word);
        self.d = get_lower_bytes(word);
      },
      _ => fail!("Unexpected addressing mode")
    }
  }

  // Stores

  fn st_a<AM:AddressingMode>(&mut self, am: AM) {
    let a = Byte(self.a);
    am.store(self, a);
  }

  fn st_sp<AM:AddressingMode>(&mut self, am: AM) {
    let value = Word(self.sp);
    am.store(self, value);
  }

  // Arithmetic

  fn add_hl(&mut self, value: u16) {
    let mut hl = (self.h << 8) as u16 + self.l as u16;

    if hl + value < hl {
      self.flags.c = true;
    } else {
      self.flags.c = false;
    }

    self.flags.n = false;

    hl += value;

    self.h = (hl >> 8) as u8;
    self.l = hl as u8;
    self.m = 3;
  }

  fn inc<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => {
        am.store(self, Byte(byte + 1));
      },
      _ => fail!()
    }
  }

  fn dec<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => {
        am.store(self, Byte(byte - 1));
      },
      _ => fail!()
    }
  }

  // Ops

  fn nop(&mut self) {
    self.m = 1;
    self.t = 4;
  }

  fn ld_bc<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Word(word) => {
        self.b = get_upper_bytes(word);
        self.c = get_lower_bytes(word);
      },
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn ld_hl<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Word(word) => {
        self.h = get_upper_bytes(word);
        self.l = get_lower_bytes(word);
      },
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn ld_mem_a<AM:AddressingMode>(&mut self, am: AM) {
    let data = Byte(self.a);
    am.store(self, data);
  }

  fn inc_bc(&mut self) {
    if self.c == 255 {
      self.b += 1;
    }
    self.c += 1;
    self.m = 1;
  }

  fn inc_de(&mut self) {
    if self.e == 255 {
      self.d += 1;
    }
    self.e += 1;
    self.m = 1;
  }

  fn inc_hl(&mut self) {
    if self.l == 255 {
      self.h += 1;
    }
    self.l += 1;
    self.m = 1;
  }

  fn inc_sp(&mut self) {
    self.sp += 1;
    self.m = 1;
  }

  fn inc_b(&mut self) {
    self.b += 1;
    if self.b == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn inc_c(&mut self) {
    self.c += 1;
    if self.c == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn inc_d(&mut self) {
    self.d += 1;
    if self.d == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn inc_e(&mut self) {
    self.e += 1;
    if self.e == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn inc_h(&mut self) {
    self.h += 1;
    if self.h == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn inc_l(&mut self) {
    self.l += 1;
    if self.l == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn inc_a(&mut self) {
    self.a += 1;
    if self.a == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn dec_bc(&mut self) {
    if self.c == 0 {
      self.b -= 1
    }
    self.c -= 1;
    self.m = 8;
  }

  fn dec_de(&mut self) {
    if self.e == 0 {
      self.d -= 1
    }
    self.e -= 1;
    self.m = 8;
  }

  fn dec_hl(&mut self) {
    if self.l == 0 {
      self.h -= 1
    }
    self.l -= 1;
    self.m = 8;
  }

  fn dec_sp(&mut self) {
    self.sp -= 1;
    self.m = 8;
  }

  fn dec_b(&mut self) {
    self.b -= 1;
    if self.b == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn dec_c(&mut self) {
    self.c -= 1;
    if self.c == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn dec_d(&mut self) {
    self.d -= 1;
    if self.d  == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn dec_e(&mut self) {
    self.e -= 1;
    if self.e  == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn dec_l(&mut self) {
    self.l -= 1;
    if self.l  == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn dec_a(&mut self) {
    self.a -= 1;
    if self.a  == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn rlca(&mut self) {
    // put bit 7 of a in carry flag
    if (self.a & 0x80) == 0x80 {
      self.flags.c = true
    } else {
      self.flags.c = false
    }

    self.a = (self.a << 1) | (self.a >> 7); // rotate a
    self.m = 4;
  }

  fn rla(&mut self) {
    let old_f: u8;
    if self.flags.c {
      old_f = 1;
    } else {
      old_f = 0;
    }

    if (self.a & 0x80) == 0x80 {
      self.flags.c = true;
    } else {
      self.flags.c = false;
    }

    self.a = (self.a << 1) | old_f; // rotate a left, move f to end of a
    self.m = 4;
  }

  fn rrca(&mut self) {
    if (self.a & 1) == 1 {
      self.flags.c = true;
    } else {
      self.flags.c = false;
    }
    self.a = (self.a >> 1) | (self.a << 7);
    self.m = 4;
  }

  fn rra(&mut self) {
    let old_f: u8;
    if self.flags.c {
      old_f = 0x80;
    } else {
      old_f = 0;
    }
    if (self.a & 1) == 1 {
      self.flags.c = true;
    } else {
      self.flags.c = false;
    }

    self.a = (self.a >> 1) | old_f;
  }

  // Jumps

  fn jr<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => {
        self.pc += byte as u16;
      },
      _ => fail!()
    }
    self.m = 8;
  }

  fn jr_nz<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => {
        if !self.flags.z {
          self.pc += byte as u16
        }
      },
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn jr_z<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => {
        if self.flags.z {
          self.pc += byte as u16
        }
      },
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn jr_nc<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => {
        if !self.flags.c {
          self.pc += byte as u16
        }
      },
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn jr_c<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => {
        if self.flags.c {
          self.pc += byte as u16
        }
      },
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn ld_mem_sp<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Word(word) => {
        let am = self.address(word);
        self.st_sp(am)
      },
      _ => {}
    }
    self.m = 3;
  }

  fn add_hl_bc(&mut self) {
    let bc = (self.b << 8) as u16 + self.c as u16;
    self.add_hl(bc);
  }

  fn add_hl_de(&mut self) {
    let de = (self.d << 8) as u16 + self.e as u16;
    self.add_hl(de);
  }

  fn add_hl_hl(&mut self) {
    let hl = (self.h << 8) as u16 + self.l as u16;
    self.add_hl(hl);
  }

  fn add_hl_sp(&mut self) {
    let sp = self.sp;
    self.add_hl(sp);
  }

  fn halt(&mut self) {
    //fail!("HALT");
  }

  fn stop(&mut self) {
    self.pc += 1;
  }
}

fn get_lower_bytes(word: u16) -> u8 {
  (word & 0xff) as u8
}
fn get_upper_bytes(word: u16) -> u8 {
  ((word & 0xff00) >> 8) as u8
}
