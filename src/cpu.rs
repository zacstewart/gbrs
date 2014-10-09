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
  fn store(&self, cpu: &mut CPU, value: Data) {
    fail!("Can't write to ROM!")
  }
}

struct ImmediateWordAddressingMode;
impl AddressingMode for ImmediateWordAddressingMode {
  fn load(&self, cpu: &mut CPU) -> Data {
    Word(cpu.take_byte() as u16 + cpu.take_byte() as u16)
  }
  fn store(&self, cpu: &mut CPU, value: Data) {
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

  pub pc: u16, // Program Counter
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
      0x00 => self.NOP(),
      0x01 => self.LD_bc_d16(),
      0x02 => self.LD_mbc_a(),
      0x03 => self.INC_bc(),
      0x04 => self.INC_b(),
      0x05 => self.DEC_b(),
      0x06 => self.LD_b_d8(),
      0x07 => self.RLCA(),
      0x08 => self.LD_a16_sp(),
      0x09 => self.ADD_hl_bc(),
      0x0a => self.LD_a_mbc(),
      0x0b => self.DEC_bc(),
      0x0c => self.INC_c(),
      0x0d => self.DEC_c(),
      0x0e => self.LD_c_d8(),
      0x0f => self.RRCA(),
      0x10 => self.STOP(),
      0x11 => self.LD_de_d16(),
      0x12 => self.LD_mde_a(),
      0x13 => self.INC_de(),
      0x14 => self.INC_d(),
      0x15 => self.DEC_d(),
      0x16 => self.LD_d_d8(),
      0x17 => self.RLA(),
      0x18 => self.JR_r8(),
      0x19 => self.ADD_hl_de(),
      0x1a => self.LD_a_mde(),
      0x1b => self.DEC_de(),
      0x1c => self.INC_e(),
      0x1d => self.DEC_e(),
      0x1e => self.LD_e_d8(),
      0x1f => self.RRA(),
      0x21 => self.LD_hl_d16(),
      0x23 => self.INC_hl(),
      0x24 => self.INC_h(),
      0x26 => self.LD_h_d8(),
      0x29 => self.ADD_hl_hl(),
      0x2a => self.LD_a_mhli(),
      0x2c => self.INC_l(),
      0x2d => self.DEC_l(),
      0x2e => self.LD_l_d8(),
      0x31 => self.LD_sp_d16(),
      0x33 => self.INC_sp(),
      0x39 => self.ADD_hl_sp(),
      0x3a => self.LD_a_mhld(),
      0x3c => self.INC_a(),
      0x3d => self.DEC_a(),
      0x3e => self.LD_a_d8(),
      0x76 => self.HALT(),
      e => println!("{}", self)
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

  // Stores

  fn st_a<AM:AddressingMode>(&mut self, am: AM) {
    let a = Byte(self.a);
    am.store(self, a);
  }

  // Arithmetic

  fn add_hl(&mut self, value: u16) {
    let mut hl = (self.h << 8) as u16 + self.l as u16;

    if (hl + value < hl) {
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

  fn NOP(&mut self) {
    self.m = 1;
    self.t = 4;
  }

  fn LD_bc_d16(&mut self) {
    let v = self.immediate();
    self.ld_c(v);
    let v = self.immediate();
    self.ld_b(v);
    self.m = 3;
  }

  fn LD_de_d16(&mut self) {
    let v = self.immediate();
    self.ld_e(v);
    let v = self.immediate();
    self.ld_d(v);
    self.m = 3;
  }

  fn LD_hl_d16(&mut self) {
    let v = self.immediate();
    self.ld_l(v);
    let v = self.immediate();
    self.ld_h(v);
    self.m = 3;
  }

  fn LD_sp_d16(&mut self) {
    let v = self.immediate_word();
    match v.load(self) {
      Word(word) => self.sp = word,
      _ => {}
    }
    self.m = 3;
  }

  fn LD_mbc_a(&mut self) {
    let am = self.address_bc();
    self.st_a(am);
    self.m = 2;
  }

  fn LD_mde_a(&mut self) {
    let am = self.address_de();
    self.st_a(am);
    self.m = 2;
  }

  fn LD_a_mbc(&mut self) {
    let am = self.address_bc();
    self.ld_a(am);
    self.m = 2;
  }

  fn LD_a_mde(&mut self) {
    let am = self.address_de();
    self.ld_a(am);
    self.m = 2;
  }

  fn LD_a_mhli(&mut self) {
    let am = self.address_de();
    self.ld_a(am);
    self.inc(am)
  }

  fn LD_a_mhld(&mut self) {
    let am = self.address_de();
    self.ld_a(am);
    self.dec(am)
  }

  fn INC_bc(&mut self) {
    if self.c == 255 {
      self.b += 1;
    }
    self.c += 1;
    self.m = 1;
  }

  fn INC_de(&mut self) {
    if self.e == 255 {
      self.d += 1;
    }
    self.e += 1;
    self.m = 1;
  }

  fn INC_hl(&mut self) {
    if self.l == 255 {
      self.h += 1;
    }
    self.l += 1;
    self.m = 1;
  }

  fn INC_sp(&mut self) {
    self.sp += 1;
    self.m = 1;
  }

  fn INC_b(&mut self) {
    self.b += 1;
    if self.b == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn INC_c(&mut self) {
    self.c += 1;
    if self.c == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn INC_d(&mut self) {
    self.d += 1;
    if self.d == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn INC_e(&mut self) {
    self.e += 1;
    if self.e == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn INC_h(&mut self) {
    self.h += 1;
    if self.h == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn INC_l(&mut self) {
    self.l += 1;
    if self.l == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn INC_a(&mut self) {
    self.a += 1;
    if self.a == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn DEC_bc(&mut self) {
    if self.c == 0 {
      self.b -= 1
    }
    self.c -= 1;
    self.m = 8;
  }

  fn DEC_de(&mut self) {
    if self.e == 0 {
      self.d -= 1
    }
    self.e -= 1;
    self.m = 8;
  }

  fn DEC_hl(&mut self) {
    if self.l == 0 {
      self.h -= 1
    }
    self.l -= 1;
    self.m = 8;
  }

  fn DEC_sp(&mut self) {
    self.sp -= 1;
    self.m = 8;
  }

  fn DEC_b(&mut self) {
    self.b -= 1;
    if self.b == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn DEC_c(&mut self) {
    self.c -= 1;
    if self.c == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn DEC_d(&mut self) {
    self.d -= 1;
    if self.d  == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn DEC_e(&mut self) {
    self.e -= 1;
    if self.e  == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn DEC_l(&mut self) {
    self.l -= 1;
    if self.l  == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn DEC_a(&mut self) {
    self.a -= 1;
    if self.a  == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn LD_b_d8(&mut self) {
    let v = self.immediate();
    self.ld_b(v);
    self.m = 2;
  }

  fn LD_c_d8(&mut self) {
    let v = self.immediate();
    self.ld_c(v);
    self.m = 2;
  }

  fn LD_d_d8(&mut self) {
    let v = self.immediate();
    self.ld_d(v);
    self.m = 2;
  }

  fn LD_e_d8(&mut self) {
    let v = self.immediate();
    self.ld_e(v);
    self.m = 2;
  }

  fn LD_h_d8(&mut self) {
    let v = self.immediate();
    self.ld_h(v);
    self.m = 2;
  }

  fn LD_l_d8(&mut self) {
    let v = self.immediate();
    self.ld_l(v);
    self.m = 2;
  }

  fn LD_a_d8(&mut self) {
    let v = self.immediate();
    self.ld_a(v);
    self.m = 2;
  }

  fn RLCA(&mut self) {
    // put bit 7 of a in carry flag
    if (self.a & 0x80) == 0x80 {
      self.flags.c = true
    } else {
      self.flags.c = false
    }

    self.a = (self.a << 1) | (self.a >> 7); // rotate a
    self.m = 4;
  }

  fn RLA(&mut self) {
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

  fn RRCA(&mut self) {
    if (self.a & 1) == 1 {
      self.flags.c = true;
    } else {
      self.flags.c = false;
    }
    self.a = (self.a >> 1) | (self.a << 7);
    self.m = 4;
  }

  fn RRA(&mut self) {
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

  fn JR_r8(&mut self) {
    match self.immediate().load(self) {
      Byte(byte) => {
        self.pc += byte as u16;
      },
      _ => fail!()
    }
    self.m = 8;
  }

  fn LD_a16_sp(&mut self) {
    match self.immediate_word().load(self) {
      Word(word) => {
        let am = self.address(word);
        self.ld_sp(am)
      },
      _ => {}
    }
    self.m = 3;
  }

  fn ADD_hl_bc(&mut self) {
    let bc = (self.b << 8) as u16 + self.c as u16;
    self.add_hl(bc);
  }

  fn ADD_hl_de(&mut self) {
    let de = (self.d << 8) as u16 + self.e as u16;
    self.add_hl(de);
  }

  fn ADD_hl_hl(&mut self) {
    let hl = (self.h << 8) as u16 + self.l as u16;
    self.add_hl(hl);
  }

  fn ADD_hl_sp(&mut self) {
    let sp = self.sp;
    self.add_hl(sp);
  }

  fn HALT(&mut self) {
    //fail!("HALT");
  }

  fn STOP(&mut self) {
    self.pc += 1;
  }
}
