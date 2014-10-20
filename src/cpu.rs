use mmu::MMU;

enum Data {
  Byte(u8),
  SignedByte(i8),
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

struct ImmediateSignedAddressingMode;
impl AddressingMode for ImmediateSignedAddressingMode {
  fn load(&self, cpu: &mut CPU) -> Data {
    SignedByte(cpu.take_byte() as i8)
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

struct RegisterAdressingMode {
  value: u8
}

impl AddressingMode for RegisterAdressingMode {
  fn load(&self, _: &mut CPU) -> Data {
    Byte(self.value)
  }

  fn store(&self, cpu: &mut CPU, value: Data) {
    fail!("Can't write registers yet");
  }
}

struct SixteenBitRegisterAdressingMode {
  value: u16
}

impl AddressingMode for SixteenBitRegisterAdressingMode {
  fn load(&self, _: &mut CPU) -> Data {
    Word(self.value)
  }

  fn store(&self, cpu: &mut CPU, value: Data) {
    fail!("Can't write registers yet");
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

  flags: Flags,
  interrups: bool
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
      flags: flags,
      interrups: true
    }
  }

  pub fn execute(&mut self) {
    loop {
      println!("{}", self);
      self.step();
    }
  }

  fn step(&mut self) {
    let instruction = self.take_byte();
    decode_op!(instruction, self);
    self.clock.m += self.m;
  }

  // Fetch from program

  fn take_byte(&mut self) -> u8 {
    let immediate = self.mmu.read_byte(self.pc);
    self.pc += 1;
    return immediate;
  }

  fn take_word(&mut self) -> u16 {
    let immediate = self.mmu.read_word(self.pc);
    self.pc += 2;
    return immediate;
  }

  // Pop off stack

  fn pop_byte(&mut self) -> u8 {
    let value = self.mmu.read_byte(self.sp);
    self.sp += 1;
    return value;
  }

  fn pop_word(&mut self) -> u16 {
    let value = self.mmu.read_word(self.sp);
    self.sp += 2;
    return value;
  }

  // Addressing

  fn immediate(&mut self) -> ImmediateAddressingMode {
    ImmediateAddressingMode
  }

  fn immediate_signed(&mut self) -> ImmediateSignedAddressingMode {
    ImmediateSignedAddressingMode
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

  fn register(&self, value: u8) -> RegisterAdressingMode {
    RegisterAdressingMode { value: value }
  }

  fn register_b(&self) -> RegisterAdressingMode {
    let val = self.b;
    self.register(val)
  }

  fn register_c(&self) -> RegisterAdressingMode {
    let val = self.c;
    self.register(val)
  }

  fn register_d(&self) -> RegisterAdressingMode {
    let val = self.d;
    self.register(val)
  }

  fn register_e(&self) -> RegisterAdressingMode {
    let val = self.e;
    self.register(val)
  }

  fn register_h(&self) -> RegisterAdressingMode {
    let val = self.h;
    self.register(val)
  }

  fn register_l(&self) -> RegisterAdressingMode {
    let val = self.l;
    self.register(val)
  }

  fn register_a(&self) -> RegisterAdressingMode {
    let val = self.a;
    self.register(val)
  }

  fn register_hl(&self) -> SixteenBitRegisterAdressingMode {
    let hl = (self.h << 8) as u16 + self.l as u16;
    SixteenBitRegisterAdressingMode { value: hl }
  }

  // 16-bit register sets

  fn set_bc(&mut self, value: u16) {
    self.b = get_upper_bytes(value);
    self.c = get_lower_bytes(value);
  }

  fn set_de(&mut self, value: u16) {
    self.d = get_upper_bytes(value);
    self.e = get_lower_bytes(value);
  }

  fn set_hl(&mut self, value: u16) {
    self.h = get_upper_bytes(value);
    self.l = get_lower_bytes(value);
  }

  fn set_af(&mut self, value: u16) {
    self.a = get_upper_bytes(value);
    let lower = get_lower_bytes(value);
    self.flags.z = (lower & 0x80) != 0;
    self.flags.n = (lower & 0x40) != 0;
    self.flags.c = (lower & 0x20) != 0;
    self.flags.h = (lower & 0x10) != 0;
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

  fn pop_bc(&mut self) {
    let value = self.pop_word();
    self.set_bc(value);
  }

  fn pop_de(&mut self) {
    let value = self.pop_word();
    self.set_de(value);
  }

  fn pop_hl(&mut self) {
    let value = self.pop_word();
    self.set_hl(value);
  }

  fn pop_af(&mut self) {
    let value = self.pop_word();
    self.set_af(value);
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

  fn add_a<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => {
        let result = self.a as u16 + byte as u16;
        self.flags.z = (result & 0xff) == 0;
        self.flags.n = false;
        self.flags.h = ((self.a & 0x0f) + (byte & 0x0f)) & 0x10 == 0x10;
        self.flags.c = result > 0xff;
        self.a = (result & 0xff) as u8
      },
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn adc_a<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => {
        let mut result = self.a as u16 + byte as u16;
        if self.flags.c {
          result += 1;
        }
        self.flags.z = (result & 0xff) == 0;
        self.flags.n = false;
        self.flags.h = ((self.a & 0x0f) + (byte & 0x0f)) & 0x10 == 0x10;
        self.flags.c = result > 0xff;
        self.a = (result & 0xff) as u8
      },
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn cp<AM:AddressingMode>(&mut self, am: AM) -> u8 {
    match am.load(self) {
      Byte(byte) => {
        self.flags.z = self.a == byte;
        self.flags.n = true;
        self.flags.h = (self.a & 0xf) < (byte & 0xf);
        self.flags.c = self.a < byte;
        return self.a - byte;
      }
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn sub<AM:AddressingMode>(&mut self, am: AM) {
    self.a = self.cp(am);
  }

  fn sbc_a<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => {
        let byte = byte - 1;
        self.flags.z = self.a == byte;
        self.flags.n = true;
        self.flags.h = (self.a & 0xf) < (byte & 0xf);
        self.flags.c = self.a < byte;
      }
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn and<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => {
        self.a &= byte;
        self.flags.z = self.a == 0;
        self.flags.n = false;
        self.flags.h = true;
        self.flags.c = false;
      }
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn xor<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => {
        self.a ^= byte;
        self.flags.z = self.a == 0;
        self.flags.n = false;
        self.flags.h = false;
        self.flags.c = false;
      }
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn or<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Byte(byte) => {
        self.a |= byte;
        self.flags.z = self.a == 0;
        self.flags.n = false;
        self.flags.h = false;
        self.flags.c = false;
      }
      _ => fail!("Unexpected addressing mode")
    }
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
      Word(word) => self.set_hl(word),
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn ld_mem_a<AM:AddressingMode>(&mut self, am: AM) {
    let data = Byte(self.a);
    am.store(self, data);
  }

  fn ld_mem_hl<AM:AddressingMode>(&mut self, am: AM) {
    let hl = (self.h << 8) as u16 + self.l as u16;
    let data = Word(hl);
    am.store(self, data);
  }

  fn ld_mem<AM1:AddressingMode,AM2:AddressingMode>(&mut self, loc: AM1, val: AM2) {
    let data = val.load(self);
    loc.store(self, data);
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

  fn dec_h(&mut self) {
    self.h -= 1;
    if self.h  == 0 {
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
      SignedByte(byte) => {
        self.pc += byte as u16;
      },
      _ => fail!()
    }
    self.m = 8;
  }

  fn jr_nz<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      SignedByte(byte) => {
        if !self.flags.z {
          self.pc += byte as u16
        }
      },
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn jr_z<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      SignedByte(byte) => {
        if self.flags.z {
          self.pc += byte as u16
        }
      },
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn jr_nc<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      SignedByte(byte) => {
        if !self.flags.c {
          self.pc += byte as u16
        }
      },
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn jr_c<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      SignedByte(byte) => {
        if self.flags.c {
          self.pc += byte as u16
        }
      },
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn jp_nz<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Word(word) => {
        if !self.flags.z {
          self.pc = word;
        }
      },
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn jp_z<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Word(word) => {
        if self.flags.z {
          self.pc = word;
        }
      },
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn jp_nc<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Word(word) => {
        if !self.flags.c {
          self.pc = word;
        }
      },
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn jp_c<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Word(word) => {
        if self.flags.c {
          self.pc = word;
        }
      },
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn jp<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Word(word) => {
        self.pc = word;
      }
      _ => fail!("Unexpected addressing mode")
    }
  }

  fn ret_nz(&mut self) {
    if !self.flags.z {
      self.pc = self.pop_word();
    }
  }

  fn ret_z(&mut self) {
    if self.flags.z {
      self.pc = self.pop_word();
    }
  }

  fn ret_nc(&mut self) {
    if !self.flags.c {
      self.pc = self.pop_word();
    }
  }

  fn ret_c(&mut self) {
    if self.flags.c {
      self.pc = self.pop_word();
    }
  }

  fn ret(&mut self) {
    self.pc = self.pop_word();
  }

  fn reti(&mut self) {
    self.pc = self.pop_word();
    self.interrups = true
  }

  // Loads

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

  // Miscellaneous

  fn daa(&mut self) {
    self.flags.c = false;
    if (self.a & 0x0f) > 9 {
      self.a += 0x06;
    }

    if ((self.a & 0xf0) >> 4) > 9 {
      self.flags.c = true;
      self.a += 0x60;
    }

    self.flags.h = false;
    self.flags.z = self.a == 0;
  }

  fn cpl(&mut self) {
    self.a = !self.a;
  }

  fn scf(&mut self) {
    self.flags.n = false;
    self.flags.h = false;
    self.flags.c = true;
  }

  fn ccf(&mut self) {
    self.flags.n = false;
    self.flags.h = false;
    self.flags.c = !self.flags.c;
  }
}

fn get_lower_bytes(word: u16) -> u8 {
  (word & 0xff) as u8
}
fn get_upper_bytes(word: u16) -> u8 {
  ((word & 0xff00) >> 8) as u8
}
