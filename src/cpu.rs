use mmu::MMU;
use std::num::Wrapping as W;
use memory_map::{ReadByte, WriteByte};
use data::Data;

trait AddressingMode {
  fn load(&self, cpu: &mut CPU) -> Data;
  fn store(&self, cpu: &mut CPU, value: Data);
}

struct ImmediateAddressingMode;
impl AddressingMode for ImmediateAddressingMode {
  fn load(&self, cpu: &mut CPU) -> Data {
    Data::Byte(cpu.take_byte())
  }
  fn store(&self, _: &mut CPU, _: Data) {
    panic!("Can't write to ROM!")
  }
}

struct ImmediateSignedAddressingMode;
impl AddressingMode for ImmediateSignedAddressingMode {
  fn load(&self, cpu: &mut CPU) -> Data {
    Data::SignedByte(cpu.take_byte() as i8)
  }
  fn store(&self, _: &mut CPU, _: Data) {
    panic!("Can't write to ROM!")
  }
}

struct ImmediateWordAddressingMode;
impl AddressingMode for ImmediateWordAddressingMode {
  fn load(&self, cpu: &mut CPU) -> Data {
    Data::Word(cpu.take_word())
  }
  fn store(&self, _: &mut CPU, _: Data) {
    panic!("Can't write to ROM!")
  }
}

struct ImmediateWordAddressAddressingMode;
impl AddressingMode for ImmediateWordAddressAddressingMode {
  fn load(&self, cpu: &mut CPU) -> Data {
    let am = MemoryAddressingMode { address: cpu.take_word() };
    am.load(cpu)
  }
  fn store(&self, cpu: &mut CPU, value: Data) {
    let am = MemoryAddressingMode { address: cpu.take_word() };
    am.store(cpu, value);
  }
}

struct MemoryAddressingMode {
  address: u16
}

impl AddressingMode for MemoryAddressingMode {
  fn load(&self, cpu: &mut CPU) -> Data {
    Data::Byte(cpu.mmu.read_byte(self.address))
  }

  fn store(&self, cpu: &mut CPU, value: Data) {
    match value {
      Data::Byte(b) => cpu.mmu.write_byte(self.address, b),
      _ => {}
    }
  }
}

struct RegisterAdressingMode {
  value: u8
}

impl AddressingMode for RegisterAdressingMode {
  fn load(&self, _: &mut CPU) -> Data {
    Data::Byte(self.value)
  }

  fn store(&self, cpu: &mut CPU, value: Data) {
    panic!("Can't write registers yet");
  }
}

struct SixteenBitRegisterAdressingMode {
  value: u16
}

impl AddressingMode for SixteenBitRegisterAdressingMode {
  fn load(&self, _: &mut CPU) -> Data {
    Data::Word(self.value)
  }

  fn store(&self, cpu: &mut CPU, value: Data) {
    panic!("Can't write registers yet");
  }
}
#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct CPU {
  mmu: MMU,
  clock: Clock,

  pub pc: u16, // Program Counter
  pub sp: u16, // Stack pointer

  // Registers
  pub a: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub h: u8,
  pub l: u8,

  // Internal clock
  m: u8,

  flags: Flags,
  interrups: bool,

  pub stopped: bool
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
      interrups: true,
      stopped: false
    }
  }

  pub fn step(&mut self) {
    let instruction = self.take_byte();
    decode_op!(instruction, self);
    self.clock.m = (W(self.clock.m) + W(self.m as u16)).0;
    self.mmu.step(self.m);
    if self.pc == 0x100 { self.mmu.leave_bios(); }
  }

  // Fetch from program

  fn take_byte(&mut self) -> u8 {
    let immediate = self.mmu.read_byte(self.pc);
    self.pc = (W(self.pc) + W(1)).0;
    return immediate;
  }

  fn take_word(&mut self) -> u16 {
    let immediate = self.mmu.read_word(self.pc);
    self.pc = (W(self.pc) + W(2)).0;
    return immediate;
  }

  // Pop off stack

  fn pop_byte(&mut self) -> u8 {
    let value = self.mmu.read_byte(self.sp);
    self.sp = self.sp + 1;
    return value;
  }

  fn pop_word(&mut self) -> u16 {
    let value = self.mmu.read_word(self.sp);
    let sp = (W(self.sp) + W(2)).0;
    return value;
  }

  fn push_word(&mut self, value: u16) {
    self.mmu.write_word(self.sp, value);
    self.sp = (W(self.sp) - W(2)).0;
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

  fn immediate_word_address(&mut self) -> ImmediateWordAddressAddressingMode {
    ImmediateWordAddressAddressingMode
  }

  fn address(&mut self, address: u16) -> MemoryAddressingMode {
    MemoryAddressingMode { address: address }
  }

  fn address_bc(&mut self) -> MemoryAddressingMode {
    let address = ((self.b as u16) << 8) + self.c as u16;
    self.address(address)
  }

  fn address_de(&mut self) -> MemoryAddressingMode {
    let address = ((self.d as u16) << 8) | self.e as u16;
    self.address(address)
  }

  fn address_hl(&mut self) -> MemoryAddressingMode {
    let address = ((self.h as u16) << 8) | self.l as u16;
    self.address(address)
  }

  fn address_hli(&mut self) -> MemoryAddressingMode {
    let address = ((self.h as u16) << 8) | self.l as u16;
    self.inc_hl();
    self.address(address)
  }

  fn address_hld(&mut self) -> MemoryAddressingMode {
    let address = ((self.h as u16) << 8) | self.l as u16;
    self.dec_hl();
    self.address(address)
  }

  fn address_c(&mut self) -> MemoryAddressingMode {
    let address = 0xff00 + self.c as u16;
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
    let hl = ((self.h as u16) << 8) + self.l as u16;
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

  // 16-bit register gets

  fn get_bc(&self) -> u16 {
    let upper = (self.b as u16) << 8;
    let lower = self.c as u16;
    return upper | lower;
  }

  fn get_de(&self) -> u16 {
    let upper = (self.d as u16) << 8;
    let lower = self.e as u16;
    return upper | lower;
  }

  fn get_hl(&self) -> u16 {
    let upper = (self.h as u16) << 8;
    let lower = self.l as u16;
    return upper | lower;
  }

  fn get_af(&self) -> u16 {
    let upper = (self.a as u16) << 8;
    let mut lower = 0u16;
    if self.flags.z { lower |= 0x80; }
    if self.flags.n { lower |= 0x40; }
    if self.flags.c { lower |= 0x20; }
    if self.flags.h { lower |= 0x10; }
    return upper | lower;
  }
  // Loads

  fn ld_b<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => self.b = byte,
      _ => {}
    }
  }

  fn ld_c<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => self.c = byte,
      _ => {}
    }
  }

  fn ld_d<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => self.d = byte,
      _ => {}
    }
  }

  fn ld_e<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => self.e = byte,
      _ => {}
    }
  }

  fn ld_h<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => self.h = byte,
      _ => {}
    }
  }

  fn ld_l<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => self.l = byte,
      _ => {}
    }
  }

  fn ld_a<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => self.a = byte,
      _ => {}
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

  fn push_bc(&mut self) {
    let value = self.get_bc();
    self.push_word(value);
  }

  fn push_de(&mut self) {
    let value = self.get_de();
    self.push_word(value);
  }

  fn push_hl(&mut self) {
    let value = self.get_hl();
    self.push_word(value);
  }

  fn push_af(&mut self) {
    let value = self.get_af();
    self.push_word(value);
  }

  // Stores

  fn st_a<AM:AddressingMode>(&mut self, am: AM) {
    let a = Data::Byte(self.a);
    am.store(self, a);
  }

  fn st_sp<AM:AddressingMode>(&mut self, am: AM) {
    let value = Data::Word(self.sp);
    am.store(self, value);
  }

  // Arithmetic

  fn add_hl(&mut self, value: u16) {
    let mut hl = ((self.h as u16) << 8) + self.l as u16;

    if (W(hl) + W(value)).0 < hl {
      self.flags.c = true;
    } else {
      self.flags.c = false;
    }

    self.flags.n = false;

    hl = (W(hl) + W(value)).0;

    self.h = (hl >> 8) as u8;
    self.l = (hl & 0xff) as u8;
    self.m = 3;
  }

  fn add_a<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => {
        let result = (W(self.a as u16) + W(byte as u16)).0;

        self.flags.z = (result & 0xff) == 0;
        self.flags.n = false;
        self.flags.h = ((W(self.a & 0x0f) + W(byte & 0x0f)).0 & 0x10) == 0x10;
        self.flags.c = result > 0xff;
        self.a = (result & 0xff) as u8
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn add_sp<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::SignedByte(byte) => {
        let result = (W(self.sp as i32) + W(byte as i32)).0 as u32;
        self.flags.z = false;
        self.flags.n = false;
        self.flags.h = (W(self.sp & 0x0fff) + W(byte as u16)).0 > 0x0fff;
        self.flags.c = result > 0xffff;
        self.sp = (result & 0xffff) as u16;
      }
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn adc_a<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => {
        let mut result = self.a as u16 + byte as u16;
        if self.flags.c {
          result = result + 1;
        }
        self.flags.z = (result & 0xff) == 0;
        self.flags.n = false;
        self.flags.h = ((self.a & 0x0f) + (byte & 0x0f)) & 0x10 == 0x10;
        self.flags.c = result > 0xff;
        self.a = (result & 0xff) as u8
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn cp<AM:AddressingMode>(&mut self, am: AM) -> u8 {
    match am.load(self) {
      Data::Byte(byte) => {
        self.flags.z = self.a == byte;
        self.flags.n = true;
        self.flags.h = (self.a & 0xf) < (byte & 0xf);
        self.flags.c = self.a < byte;
        return (W(self.a) - W(byte)).0;
      }
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn sub<AM:AddressingMode>(&mut self, am: AM) {
    self.a = self.cp(am);
  }

  fn sbc_a<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => {
        let byte = byte - 1;
        self.flags.z = self.a == byte;
        self.flags.n = true;
        self.flags.h = (self.a & 0xf) < (byte & 0xf);
        self.flags.c = self.a < byte;
      }
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn and<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => {
        self.a &= byte;
        self.flags.z = self.a == 0;
        self.flags.n = false;
        self.flags.h = true;
        self.flags.c = false;
      }
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn xor<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => {
        self.a ^= byte;
        self.flags.z = self.a == 0;
        self.flags.n = false;
        self.flags.h = false;
        self.flags.c = false;
      }
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn or<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => {
        self.a |= byte;
        self.flags.z = self.a == 0;
        self.flags.n = false;
        self.flags.h = false;
        self.flags.c = false;
      }
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn inc<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => {
        let byte = (W(byte) + W(1)).0;
        am.store(self, Data::Byte(byte));
      },
      _ => panic!()
    }
  }

  fn dec<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => {
        let byte = (W(byte) - W(1)).0;
        am.store(self, Data::Byte(byte));
      },
      _ => panic!()
    }
  }

  // Ops

  fn nop(&mut self) {
    self.m = 1;
    self.t = 4;
  }

  fn ld_bc<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Word(word) => self.set_bc(word),
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn ld_de<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Word(word) => self.set_de(word),
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn ld_hl<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Word(word) => self.set_hl(word),
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn ld_sp<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Word(word) => self.sp = word,
      _ => {}
    }
  }

  fn ld_hl_sp_plus_immediate_signed(&mut self) {
    let  sp = self.sp as i16;
    let immediate = self.take_byte() as i16;

    self.set_hl((W(sp) + W(immediate)).0 as u16);
  }

  fn ldh_a<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(b) => {
        let mem = self.address(b as u16 + 0xff00);
        match mem.load(self) {
          Data::Byte(val) => self.a = val,
          _ => panic!("Unexpected addressing mode")
        }
      }
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn ld_mem_a<AM:AddressingMode>(&mut self, am: AM) {
    let data = Data::Byte(self.a);
    am.store(self, data);
  }

  fn ld_mem_hl<AM:AddressingMode>(&mut self, am: AM) {
    let hl = ((self.h as u16) << 8) + self.l as u16;
    let data = Data::Word(hl);
    am.store(self, data);
  }

  fn ld_mem<AM1:AddressingMode,AM2:AddressingMode>(&mut self, loc: AM1, val: AM2) {
    let data = val.load(self);
    loc.store(self, data);
  }

  fn ldh_mem<AM1:AddressingMode,AM2:AddressingMode>(&mut self, loc: AM1, val: AM2) {
    match loc.load(self) {
      Data::Byte(b) => {
        let val = val.load(self);
        let loc = self.address(b as u16 + 0xff00);
        loc.store(self, val);
      }
      _ => {}
    }
  }

  // 16-bit INCs

  fn inc_bc(&mut self) {
    if self.c == 255 {
      self.b = (W(self.b) + W(1)).0;
    }
    self.c = (W(self.c) + W(1)).0;
    self.m = 1;
  }

  fn inc_de(&mut self) {
    if self.e == 255 {
      self.d = (W(self.d) + W(1)).0;
    }
    self.e = (W(self.e) + W(1)).0;
    self.m = 1;
  }

  fn inc_hl(&mut self) {
    if self.l == 255 {
      self.h = (W(self.h) + W(1)).0;
    }
    self.l = (W(self.l) + W(1)).0;
    self.m = 1;
  }

  fn inc_sp(&mut self) {
    self.sp = (W(self.sp) + W(1)).0;
    self.m = 1;
  }

  // 8-bit INCs

  fn inc_b(&mut self) {
    self.b = (W(self.b) + W(1)).0;
    if self.b == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn inc_c(&mut self) {
    self.c = (W(self.c) + W(1)).0;
    if self.c == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn inc_d(&mut self) {
    self.d = (W(self.d) + W(1)).0;
    if self.d == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn inc_e(&mut self) {
    self.e = (W(self.e) + W(1)).0;
    if self.e == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn inc_h(&mut self) {
    self.h = (W(self.h) + W(1)).0;
    if self.h == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn inc_l(&mut self) {
    self.l = (W(self.l) + W(1)).0;
    if self.l == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn inc_a(&mut self) {
    self.a = (W(self.a) + W(1)).0;
    if self.a == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  // 16-bit DECs

  fn dec_bc(&mut self) {
    if self.c == 0 {
      self.b = (W(self.b) - W(1)).0
    }
    self.c = (W(self.c) - W(1)).0;
    self.m = 8;
  }

  fn dec_de(&mut self) {
    if self.e == 0 {
      self.d = (W(self.d) - W(1)).0
    }
    self.e = (W(self.e) - W(1)).0;
    self.m = 8;
  }

  fn dec_hl(&mut self) {
    if self.l == 0 {
      self.h = (W(self.h) - W(1)).0
    }
    self.l = (W(self.l) - W(1)).0;
    self.m = 8;
  }

  fn dec_sp(&mut self) {
    self.sp = (W(self.sp) - W(1)).0;
  }

  // 8-bit DECs

  fn dec_b(&mut self) {
    self.b = (W(self.b) - W(1)).0;
    if self.b == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn dec_c(&mut self) {
    self.c = (W(self.c) - W(1)).0;
    if self.c == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn dec_d(&mut self) {
    self.d = (W(self.d) - W(1)).0;
    if self.d  == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn dec_e(&mut self) {
    self.e = (W(self.e) - W(1)).0;
    if self.e  == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn dec_h(&mut self) {
    self.h = (W(self.h) - W(1)).0;
    if self.h  == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn dec_l(&mut self) {
    self.l = (W(self.l) - W(1)).0;
    if self.l  == 0 {
      self.flags.z = true;
    } else {
      self.flags.z = false;
    }
  }

  fn dec_a(&mut self) {
    self.a = (W(self.a) - W(1)).0;
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
      Data::SignedByte(byte) => {
        self.pc = (W(self.pc as i16) + W(byte as i16)).0 as u16;
      },
      _ => panic!()
    }
  }

  fn jr_nz<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::SignedByte(byte) => {
        if !self.flags.z {
          self.pc = (W(self.pc as i16) + W(byte as i16)).0 as u16;
        }
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn jr_z<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::SignedByte(byte) => {
        if self.flags.z {
          self.pc = (W(self.pc as i16) + W(byte as i16)).0 as u16;
        }
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn jr_nc<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::SignedByte(byte) => {
        if !self.flags.c {
          self.pc = (W(self.pc as i16) + W(byte as i16)).0 as u16;
        }
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn jr_c<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::SignedByte(byte) => {
        if self.flags.c {
          self.pc = (W(self.pc as i16) + W(byte as i16)).0 as u16;
        }
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn jp_nz<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Word(word) => {
        if !self.flags.z {
          self.pc = word;
        }
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn jp_z<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Word(word) => {
        if self.flags.z {
          self.pc = word;
        }
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn jp_nc<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Word(word) => {
        if !self.flags.c {
          self.pc = word;
        }
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn jp_c<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Word(word) => {
        if self.flags.c {
          self.pc = word;
        }
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn jp<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Word(word) => {
        self.pc = word;
      }
      _ => panic!("Unexpected addressing mode")
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

  fn call_nz<AM:AddressingMode>(&mut self, am: AM) {
    if !self.flags.z {
      let val = self.take_word();
      self.push_word(val);
      self.pc = match am.load(self) {
        Data::Word(w) => w,
        _ => panic!("Unexpected addressing mode!")
      }
    }
  }

  fn call_z<AM:AddressingMode>(&mut self, am: AM) {
    if self.flags.z {
      let val = self.take_word();
      self.push_word(val);
      self.pc = match am.load(self) {
        Data::Word(w) => w,
        _ => panic!("Unexpected addressing mode!")
      }
    }
  }

  fn call_nc<AM:AddressingMode>(&mut self, am: AM) {
    if !self.flags.c {
      let val = self.take_word();
      self.push_word(val);
      self.pc = match am.load(self) {
        Data::Word(w) => w,
        _ => panic!("Unexpected addressing mode!")
      }
    }
  }

  fn call_c<AM:AddressingMode>(&mut self, am: AM) {
    if self.flags.c {
      let val = self.take_word();
      self.push_word(val);
      self.pc = match am.load(self) {
        Data::Word(w) => w,
        _ => panic!("Unexpected addressing mode!")
      }
    }
  }

  fn call<AM:AddressingMode>(&mut self, am: AM) {
      let val = self.take_word();
      self.push_word(val);
    self.pc = match am.load(self) {
      Data::Word(w) => w,
      _ => panic!("Unexpected addressing mode!")
    }
  }

  fn rst(&mut self, address: u16) {
    let pc = self.pc;
    self.push_word(pc);
    self.pc = address;
  }

  // Loads

  fn ld_mem_sp<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Word(word) => {
        let am = self.address(word);
        self.st_sp(am)
      },
      _ => {}
    }
    self.m = 3;
  }

  fn add_hl_bc(&mut self) {
    let bc = ((self.b as u16) << 8) + self.c as u16;
    self.add_hl(bc);
  }

  fn add_hl_de(&mut self) {
    let de = ((self.d as u16) << 8) + self.e as u16;
    self.add_hl(de);
  }

  fn add_hl_hl(&mut self) {
    let hl = ((self.h as u16) << 8) + self.l as u16;
    self.add_hl(hl);
  }

  fn add_hl_sp(&mut self) {
    let sp = self.sp;
    self.add_hl(sp);
  }

  fn stop(&mut self) {
    self.stopped = true;
    self.pc = self.pc + 1;
  }

  fn halt(&mut self) {
      // TODO: HALT
  }

  fn disable_interrupts(&mut self) {
      self.interrups = false;
  }

  fn enable_interrupts(&mut self) {
      self.interrups = true;
  }

  // Miscellaneous

  fn daa(&mut self) {
    self.flags.c = false;
    if (self.a & 0x0f) > 9 {
      self.a = self.a + 0x06;
    }

    if ((self.a & 0xf0) >> 4) > 9 {
      self.flags.c = true;
      self.a = self.a + 0x60;
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

  // Bit opcodes

  fn bit<AM:AddressingMode>(&mut self, bit: u8, am: AM) {
      match am.load(self) {
          Data::Byte(value) => {
              let check = 1 << bit;
              self.flags.z = value & check == 0;
              self.flags.n = false;
              self.flags.h = true;
          }
          _ => panic!("Unexpected addressing mode")
      }
      self.m = 8;
  }
}

fn get_lower_bytes(word: u16) -> u8 {
  (word & 0xff) as u8
}
fn get_upper_bytes(word: u16) -> u8 {
  ((word & 0xff00) >> 8) as u8
}
