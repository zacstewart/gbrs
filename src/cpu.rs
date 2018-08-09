use std::num::Wrapping as W;

use data::Data;
use debugger;
use memory_map::{ReadByte, WriteByte};
use mmu::MMU;

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
  register: Register
}

impl AddressingMode for RegisterAdressingMode {
  fn load(&self, cpu: &mut CPU) -> Data {
    let val = match self.register {
      Register::A => cpu.registers.a,
      Register::B => cpu.registers.b,
      Register::C => cpu.registers.c,
      Register::D => cpu.registers.d,
      Register::E => cpu.registers.e,
      Register::H => cpu.registers.h,
      Register::L => cpu.registers.l
    };
    Data::Byte(val)
  }

  fn store(&self, cpu: &mut CPU, value: Data) {
    if let Data::Byte(b) = value {
      match self.register {
        Register::A => cpu.registers.a = b,
        Register::B => cpu.registers.b = b,
        Register::C => cpu.registers.c = b,
        Register::D => cpu.registers.d = b,
        Register::E => cpu.registers.e = b,
        Register::H => cpu.registers.h = b,
        Register::L => cpu.registers.l = b
      }
    }
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
  pub z: bool,
  pub n: bool,
  pub h: bool,
  pub c: bool
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

#[derive(Debug,Clone,Copy)]
pub struct Registers {
  pub pc: u16, // Program Counter
  pub sp: u16, // Stack pointer
  pub a: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub h: u8,
  pub l: u8,
}

enum Register {
  A, B, C, D, E, H, L
}

impl Registers {
  fn new() -> Registers {
    Registers {
      pc: 0,
      sp: 0,
      // Registers
      a: 0,
      b: 0,
      c: 0,
      d: 0,
      e: 0,
      h: 0,
      l: 0,
    }
  }
}

#[derive(Debug)]
pub struct CPU {
  pub mmu: MMU,
  clock: Clock,
  pub registers: Registers,

  // Internal clock
  m: u8,

  pub flags: Flags,
  pub interrupts: bool,

  pub stopped: bool
}

impl CPU {
  pub fn new(mmu: MMU) -> CPU {
    let clock = Clock::new();
    let flags = Flags::new();
    CPU {
      mmu: mmu,
      clock: clock,
      registers: Registers::new(),
      m: 0,
      flags: flags,
      interrupts: true,
      stopped: false
    }
  }

  pub fn step(&mut self, debugger: &mut debugger::Debugger) {
    self.m = 0;
    debugger.set_pc(self.registers.pc);

    // Break when leaving bios
    if self.registers.pc >= 0x00ff {
      debugger.add_pc_break(self.registers.pc);
    }

    // Automatically break if we enter invalid program address space
    if self.registers.pc >= 0x7fff {
      debugger.add_pc_break(self.registers.pc);
    }

    let instruction = self.take_byte();

    debugger.set_instruction(instruction);
    debugger.debug(self);

    decode_op!(instruction, self);

    self.clock.m = (W(self.clock.m) + W(self.m as u16)).0;
    self.mmu.step(self.m);
  }

  // Fetch from program

  fn take_byte(&mut self) -> u8 {
    let immediate = self.mmu.read_byte(self.registers.pc);
    self.registers.pc = (W(self.registers.pc) + W(1)).0;
    self.m = (W(self.m) + W(4)).0;
    return immediate;
  }

  fn take_word(&mut self) -> u16 {
    let immediate = self.mmu.read_word(self.registers.pc);
    self.registers.pc = (W(self.registers.pc) + W(2)).0;
    self.m = (W(self.m) + W(8)).0;
    return immediate;
  }

  // Pop off stack

  fn pop_byte(&mut self) -> u8 {
    let value = self.mmu.read_byte(self.registers.sp);
    self.registers.sp = self.registers.sp + 1;
    return value;
  }

  fn pop_word(&mut self) -> u16 {
    let value = self.mmu.read_word(self.registers.sp);
    self.registers.sp = (W(self.registers.sp) + W(2)).0;
    value
  }

  fn push_word(&mut self, value: u16) {
    self.registers.sp = (W(self.registers.sp) - W(2)).0;
    self.mmu.write_word(self.registers.sp, value);
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
    let address = ((self.registers.b as u16) << 8) + self.registers.c as u16;
    self.address(address)
  }

  fn address_de(&mut self) -> MemoryAddressingMode {
    let address = ((self.registers.d as u16) << 8) | self.registers.e as u16;
    self.address(address)
  }

  fn address_hl(&mut self) -> MemoryAddressingMode {
    let address = ((self.registers.h as u16) << 8) | self.registers.l as u16;
    self.address(address)
  }

  fn address_hli(&mut self) -> MemoryAddressingMode {
    let address = ((self.registers.h as u16) << 8) | self.registers.l as u16;
    self.increment_hl();
    self.address(address)
  }

  fn address_hld(&mut self) -> MemoryAddressingMode {
    let address = ((self.registers.h as u16) << 8) | self.registers.l as u16;
    self.decrement_hl();
    self.address(address)
  }

  fn address_c(&mut self) -> MemoryAddressingMode {
    let address = 0xff00 + self.registers.c as u16;
    self.address(address)
  }

  fn register(&self, register: Register) -> RegisterAdressingMode {
    RegisterAdressingMode { register: register }
  }

  fn register_b(&self) -> RegisterAdressingMode {
    self.register(Register::B)
  }

  fn register_c(&self) -> RegisterAdressingMode {
    self.register(Register::C)
  }

  fn register_d(&self) -> RegisterAdressingMode {
    self.register(Register::D)
  }

  fn register_e(&self) -> RegisterAdressingMode {
    ;
    self.register(Register::E)
  }

  fn register_h(&self) -> RegisterAdressingMode {
    self.register(Register::H)
  }

  fn register_l(&self) -> RegisterAdressingMode {
    self.register(Register::L)
  }

  fn register_a(&self) -> RegisterAdressingMode {
    self.register(Register::A)
  }

  fn register_hl(&self) -> SixteenBitRegisterAdressingMode {
    let hl = ((self.registers.h as u16) << 8) + self.registers.l as u16;
    SixteenBitRegisterAdressingMode { value: hl }
  }

  // 16-bit register sets

  fn set_bc(&mut self, value: u16) {
    self.registers.b = get_upper_bytes(value);
    self.registers.c = get_lower_bytes(value);
  }

  fn set_de(&mut self, value: u16) {
    self.registers.d = get_upper_bytes(value);
    self.registers.e = get_lower_bytes(value);
  }

  fn set_hl(&mut self, value: u16) {
    self.registers.h = get_upper_bytes(value);
    self.registers.l = get_lower_bytes(value);
  }

  fn set_af(&mut self, value: u16) {
    self.registers.a = get_upper_bytes(value);
    let lower = get_lower_bytes(value);
    self.flags.z = (lower & 0x80) != 0;
    self.flags.n = (lower & 0x40) != 0;
    self.flags.c = (lower & 0x20) != 0;
    self.flags.h = (lower & 0x10) != 0;
  }

  // 16-bit register gets

  fn get_bc(&self) -> u16 {
    let upper = (self.registers.b as u16) << 8;
    let lower = self.registers.c as u16;
    return upper | lower;
  }

  fn get_de(&self) -> u16 {
    let upper = (self.registers.d as u16) << 8;
    let lower = self.registers.e as u16;
    return upper | lower;
  }

  fn get_hl(&self) -> u16 {
    let upper = (self.registers.h as u16) << 8;
    let lower = self.registers.l as u16;
    return upper | lower;
  }

  fn get_af(&self) -> u16 {
    let upper = (self.registers.a as u16) << 8;
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
      Data::Byte(byte) => self.registers.b = byte,
      _ => {}
    }
  }

  fn ld_c<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => self.registers.c = byte,
      _ => {}
    }
  }

  fn ld_d<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => self.registers.d = byte,
      _ => {}
    }
  }

  fn ld_e<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => self.registers.e = byte,
      _ => {}
    }
  }

  fn ld_h<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => self.registers.h = byte,
      _ => {}
    }
  }

  fn ld_l<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => self.registers.l = byte,
      _ => {}
    }
  }

  fn ld_a<AM:AddressingMode>(&mut self, am: AM) {
    if let Data::Byte(byte) = am.load(self) {
      self.registers.a = byte;
      self.m += 4;
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
    let a = Data::Byte(self.registers.a);
    am.store(self, a);
  }

  fn st_sp<AM:AddressingMode>(&mut self, am: AM) {
    let value = Data::Word(self.registers.sp);
    am.store(self, value);
  }

  // Arithmetic

  fn add_hl(&mut self, value: u16) {
    let mut hl = ((self.registers.h as u16) << 8) + self.registers.l as u16;

    if (W(hl) + W(value)).0 < hl {
      self.flags.c = true;
    } else {
      self.flags.c = false;
    }

    self.flags.n = false;

    hl = (W(hl) + W(value)).0;

    self.registers.h = (hl >> 8) as u8;
    self.registers.l = (hl & 0xff) as u8;
    self.m = 8;
  }

  fn add_a<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => {
        let result = (W(self.registers.a as u16) + W(byte as u16)).0;

        self.flags.z = (result & 0xff) == 0;
        self.flags.n = false;
        self.flags.h = ((W(self.registers.a & 0x0f) + W(byte & 0x0f)).0 & 0x10) == 0x10;
        self.flags.c = result > 0xff;
        self.registers.a = (result & 0xff) as u8
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn add_sp<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::SignedByte(byte) => {
        let result = (W(self.registers.sp as i32) + W(byte as i32)).0 as u32;
        self.flags.z = false;
        self.flags.n = false;
        self.flags.h = (W(self.registers.sp & 0x0fff) + W(byte as u16)).0 > 0x0fff;
        self.flags.c = result > 0xffff;
        self.registers.sp = (result & 0xffff) as u16;
      }
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn adc_a<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => {
        let mut result = self.registers.a as u16 + byte as u16;
        if self.flags.c {
          result = result + 1;
        }
        self.flags.z = (result & 0xff) == 0;
        self.flags.n = false;
        self.flags.h = ((self.registers.a & 0x0f) + (byte & 0x0f)) & 0x10 == 0x10;
        self.flags.c = result > 0xff;
        self.registers.a = (result & 0xff) as u8
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn cp<AM:AddressingMode>(&mut self, am: AM) -> u8 {
    match am.load(self) {
      Data::Byte(byte) => {
        self.flags.z = self.registers.a == byte;
        self.flags.n = true;
        self.flags.h = (self.registers.a & 0xf) < (byte & 0xf);
        self.flags.c = self.registers.a < byte;
        return (W(self.registers.a) - W(byte)).0;
      }
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn sub<AM:AddressingMode>(&mut self, am: AM) {
    self.registers.a = self.cp(am);
  }

  fn sbc_a<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => {
        let byte = byte - 1;
        self.flags.z = self.registers.a == byte;
        self.flags.n = true;
        self.flags.h = (self.registers.a & 0xf) < (byte & 0xf);
        self.flags.c = self.registers.a < byte;
      }
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn and<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(byte) => {
        self.registers.a &= byte;
        self.flags.z = self.registers.a == 0;
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
        self.registers.a ^= byte;
        self.flags.z = self.registers.a == 0;
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
        self.registers.a |= byte;
        self.flags.z = self.registers.a == 0;
        self.flags.n = false;
        self.flags.h = false;
        self.flags.c = false;
      }
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn inc<AM:AddressingMode>(&mut self, am: AM) {
      match am.load(self) {
          Data::Byte(mut b) => {
              // Carry 3rd bit?
              self.flags.h = (b & 0x7) == 0x7;

              b = (W(b) + W(1)).0;

              self.flags.z = b == 0;
              self.flags.n = false;

              am.store(self, Data::Byte(b));
          }
          _ => panic!("Unexpected addressing mode")
      }
  }

  fn dec<AM:AddressingMode>(&mut self, am: AM) {
      match am.load(self) {
          Data::Byte(mut b) => {
              // Borrow from 4th bit?
              self.flags.h = (b & 0x8) == 0x8;

              b = (W(b) - W(1)).0;

              self.flags.z = b == 0;
              self.flags.n = true;

              am.store(self, Data::Byte(b));
          },
          _ => panic!("Unexpected addressing mode")
      }
  }

  // Ops

  fn nop(&mut self) {
      self.m = 4;
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
    self.m += 12
  }

  fn ld_sp<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Word(word) => self.registers.sp = word,
      _ => {}
    }
    self.m += 12
  }

  fn ld_hl_sp_plus_immediate_signed(&mut self) {
    let  sp = self.registers.sp as i16;
    let immediate = self.take_byte() as i16;

    self.set_hl((W(sp) + W(immediate)).0 as u16);
  }

  fn ldh_a<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(b) => {
        let address = 0xff00 + b as u16;
        let mem = self.address(address);
        match mem.load(self) {
          Data::Byte(val) => self.registers.a = val,
          _ => panic!("Unexpected addressing mode")
        }
      }
      _ => panic!("Unexpected addressing mode")
    }
    self.m  += 12
  }

  fn ld_mem_a<AM:AddressingMode>(&mut self, am: AM) {
    let data = Data::Byte(self.registers.a);
    am.store(self, data);
    self.m += 4;
  }

  fn ld_mem_hl<AM:AddressingMode>(&mut self, am: AM) {
    let hl = ((self.registers.h as u16) << 8) + self.registers.l as u16;
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
    self.m += 12
  }

  // 16-bit INCs

  fn inc_bc(&mut self) {
    if self.registers.c == 255 {
      self.registers.b = (W(self.registers.b) + W(1)).0;
    }
    self.registers.c = (W(self.registers.c) + W(1)).0;
    self.m += 4;
  }

  fn inc_de(&mut self) {
    if self.registers.e == 255 {
      self.registers.d = (W(self.registers.d) + W(1)).0;
    }
    self.registers.e = (W(self.registers.e) + W(1)).0;
    self.m += 4;
  }

  fn inc_hl(&mut self) {
    self.increment_hl();
    self.m += 4;
  }

  fn inc_sp(&mut self) {
    self.registers.sp = (W(self.registers.sp) + W(1)).0;
    self.m += 4;
  }

  fn increment_hl(&mut self) {
    if self.registers.l == 255 {
      self.registers.h = (W(self.registers.h) + W(1)).0;
    }
    self.registers.l = (W(self.registers.l) + W(1)).0;
  }

  // 16-bit DECs

  fn dec_bc(&mut self) {
    if self.registers.c == 0 {
      self.registers.b = (W(self.registers.b) - W(1)).0
    }
    self.registers.c = (W(self.registers.c) - W(1)).0;
    self.m += 4;
  }

  fn dec_de(&mut self) {
    if self.registers.e == 0 {
      self.registers.d = (W(self.registers.d) - W(1)).0
    }
    self.registers.e = (W(self.registers.e) - W(1)).0;
    self.m += 4;
  }

  fn dec_hl(&mut self) {
    self.decrement_hl();
    self.m += 4;
  }

  fn dec_sp(&mut self) {
    self.registers.sp = (W(self.registers.sp) - W(1)).0;
    self.m += 4;
  }

  fn decrement_hl(&mut self) {
    if self.registers.l == 0 {
      self.registers.h = (W(self.registers.h) - W(1)).0
    }
    self.registers.l = (W(self.registers.l) - W(1)).0;
  }

  // Rotations

  fn rl<AM: AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Byte(mut b) => {
        if b & 0x80 == 0x80 {
          self.flags.c = true;
        } else {
          self.flags.c = false;
        }

        b = (b << 1) | (b >> 7); // Rotate value left

        if b == 0 {
          self.flags.z = true;
        }

        am.store(self, Data::Byte(b));

        self.flags.n = false;
        self.flags.h = false;
      }
      _ => panic!()
    }
  }

  fn rlca(&mut self) {
    let am = self.register_a();
    self.rl(am);

    self.flags.z = false;
    self.flags.n = false;
    self.flags.h = false;
  }

  fn rla(&mut self) {
    let old_f: u8;
    if self.flags.c {
      old_f = 1;
    } else {
      old_f = 0;
    }

    if (self.registers.a & 0x80) == 0x80 {
      self.flags.c = true;
    } else {
      self.flags.c = false;
    }

    self.registers.a = (self.registers.a << 1) | old_f; // rotate a left, move f to end of a
  }

  fn rrca(&mut self) {
    if (self.registers.a & 1) == 1 {
      self.flags.c = true;
    } else {
      self.flags.c = false;
    }
    self.registers.a = (self.registers.a >> 1) | (self.registers.a << 7);
  }

  fn rra(&mut self) {
    let old_f: u8;
    if self.flags.c {
      old_f = 0x80;
    } else {
      old_f = 0;
    }
    if (self.registers.a & 1) == 1 {
      self.flags.c = true;
    } else {
      self.flags.c = false;
    }

    self.registers.a = (self.registers.a >> 1) | old_f;
  }

  // Jumps

  fn jr<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::SignedByte(byte) => {
        self.registers.pc = (W(self.registers.pc as i16) + W(byte as i16)).0 as u16;
      },
      _ => panic!()
    }
    self.m += 4;
  }

  fn jr_nz<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::SignedByte(byte) => {
        if !self.flags.z {
          self.registers.pc = (W(self.registers.pc as i16) + W(byte as i16)).0 as u16;
          self.m += 12
        } else {
          self.m += 8;
        }
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn jr_z<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::SignedByte(byte) => {
        if self.flags.z {
          self.registers.pc = (W(self.registers.pc as i16) + W(byte as i16)).0 as u16;
          self.m += 12;
        } else {
          self.m += 8;
        }
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn jr_nc<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::SignedByte(byte) => {
        if !self.flags.c {
          self.registers.pc = (W(self.registers.pc as i16) + W(byte as i16)).0 as u16;
          self.m += 12;
        } else {
          self.m += 8;
        }
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn jr_c<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::SignedByte(byte) => {
        if self.flags.c {
          self.registers.pc = (W(self.registers.pc as i16) + W(byte as i16)).0 as u16;
          self.m += 12;
        } else {
          self.m += 8;
        }
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn jp_nz<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Word(word) => {
        if !self.flags.z {
          self.registers.pc = word;
        }
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn jp_z<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Word(word) => {
        if self.flags.z {
          self.registers.pc = word;
        }
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn jp_nc<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Word(word) => {
        if !self.flags.c {
          self.registers.pc = word;
        }
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn jp_c<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Word(word) => {
        if self.flags.c {
          self.registers.pc = word;
        }
      },
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn jp<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Word(word) => {
        self.registers.pc = word;
      }
      _ => panic!("Unexpected addressing mode")
    }
  }

  fn ret_nz(&mut self) {
    if !self.flags.z {
      self.registers.pc = self.pop_word();
    }
  }

  fn ret_z(&mut self) {
    if self.flags.z {
      self.registers.pc = self.pop_word();
    }
  }

  fn ret_nc(&mut self) {
    if !self.flags.c {
      self.registers.pc = self.pop_word();
    }
  }

  fn ret_c(&mut self) {
    if self.flags.c {
      self.registers.pc = self.pop_word();
    }
  }

  fn ret(&mut self) {
    self.registers.pc = self.pop_word();
  }

  fn reti(&mut self) {
    self.registers.pc = self.pop_word();
    self.interrupts = true
  }

  fn call_nz<AM:AddressingMode>(&mut self, am: AM) {
    if !self.flags.z {
      let val = self.take_word();
      self.push_word(val);
      self.registers.pc = match am.load(self) {
        Data::Word(w) => w,
        _ => panic!("Unexpected addressing mode!")
      }
    }
  }

  fn call_z<AM:AddressingMode>(&mut self, am: AM) {
    if self.flags.z {
      let val = self.take_word();
      self.push_word(val);
      self.registers.pc = match am.load(self) {
        Data::Word(w) => w,
        _ => panic!("Unexpected addressing mode!")
      }
    }
  }

  fn call_nc<AM:AddressingMode>(&mut self, am: AM) {
    if !self.flags.c {
      let val = self.take_word();
      self.push_word(val);
      self.registers.pc = match am.load(self) {
        Data::Word(w) => w,
        _ => panic!("Unexpected addressing mode!")
      }
    }
  }

  fn call_c<AM:AddressingMode>(&mut self, am: AM) {
    if self.flags.c {
      let val = self.take_word();
      self.push_word(val);
      self.registers.pc = match am.load(self) {
        Data::Word(w) => w,
        _ => panic!("Unexpected addressing mode!")
      }
    }
  }

  fn call<AM:AddressingMode>(&mut self, am: AM) {
    match am.load(self) {
      Data::Word(addr) => {
        let pc = self.registers.pc;
        self.push_word(pc);
        self.registers.pc = addr;
      }
      _ => panic!("Unexpected addressing mode!")
    };
  }

  fn rst(&mut self, address: u16) {
    let pc = self.registers.pc;
    self.push_word(pc);
    self.registers.pc = address;
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
    self.m += 8;
  }

  fn add_hl_bc(&mut self) {
    let bc = ((self.registers.b as u16) << 8) + self.registers.c as u16;
    self.add_hl(bc);
  }

  fn add_hl_de(&mut self) {
    let de = ((self.registers.d as u16) << 8) + self.registers.e as u16;
    self.add_hl(de);
  }

  fn add_hl_hl(&mut self) {
    let hl = ((self.registers.h as u16) << 8) + self.registers.l as u16;
    self.add_hl(hl);
  }

  fn add_hl_sp(&mut self) {
    let sp = self.registers.sp;
    self.add_hl(sp);
  }

  fn stop(&mut self) {
    self.stopped = true;
    self.registers.pc = self.registers.pc + 1;
  }

  fn halt(&mut self) {
      // TODO: HALT
      panic!("Halt");
  }

  fn disable_interrupts(&mut self) {
      self.interrupts = false;
  }

  fn enable_interrupts(&mut self) {
      self.interrupts = true;
  }

  // Miscellaneous

  fn daa(&mut self) {
    self.flags.c = false;
    if (self.registers.a & 0x0f) > 9 {
      self.registers.a = self.registers.a + 0x06;
    }

    if ((self.registers.a & 0xf0) >> 4) > 9 {
      self.flags.c = true;
      self.registers.a = self.registers.a + 0x60;
    }

    self.flags.h = false;
    self.flags.z = self.registers.a == 0;
    self.m += 4;
  }

  fn cpl(&mut self) {
    self.registers.a = !self.registers.a;
    self.m += 4;
  }

  fn scf(&mut self) {
    self.flags.n = false;
    self.flags.h = false;
    self.flags.c = true;
    self.m += 4;
  }

  fn ccf(&mut self) {
    self.flags.n = false;
    self.flags.h = false;
    self.flags.c = !self.flags.c;
    self.m += 4;
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
      self.m += 8;
  }
}

fn get_lower_bytes(word: u16) -> u8 {
  (word & 0xff) as u8
}
fn get_upper_bytes(word: u16) -> u8 {
  ((word & 0xff00) >> 8) as u8
}

#[cfg(test)]
mod tests {
    use cartridge::Cartridge;
    use mmu::MMU;
    use super::CPU;

    macro_rules! assert_cyles_equal {
        ([$($program:expr),*], $cycles:expr) => {{
            let cart = Cartridge::new(vec![$($program),*].into_boxed_slice());
            let mut mmu: MMU = MMU::new();
            mmu.load_cartridge(cart);
            let mut cpu: CPU = CPU::new(mmu);
            cpu.step();
            assert_eq!(cpu.clock.m, $cycles);
        }}
    }

    #[test]
    fn instruction_timings() {
        assert_cyles_equal!([0x00], 4);              // 0x00 nop
        assert_cyles_equal!([0x01, 0x00, 0x00], 12); // 0x01 ld bc d16
        assert_cyles_equal!([0x02], 8);              // 0x02 ld (bc) a
        assert_cyles_equal!([0x03], 8);              // 0x03 inc bc
        assert_cyles_equal!([0x04], 4);              // 0x04 inc b
        assert_cyles_equal!([0x05], 4);              // 0x05 dec b
        assert_cyles_equal!([0x06, 0x01], 8);        // 0x06 LD B,d8
        assert_cyles_equal!([0x07], 4);              // 0x07 RLCA
        assert_cyles_equal!([0x08, 0x01, 0x02], 20); // 0x08 LD (a16),SP
        assert_cyles_equal!([0x09], 8);              // 0x09 ADD HL,BC
        assert_cyles_equal!([0x0a], 8);              // 0x0a LD A,(BC)
        assert_cyles_equal!([0x0b], 8);              // 0x0b DEC BC
        assert_cyles_equal!([0x0c], 4);              // 0x0c INC C
        assert_cyles_equal!([0x0d], 4);              // 0x0d DEC C
        assert_cyles_equal!([0x0e], 8);              // 0x0e LD C,d8
        assert_cyles_equal!([0x0f], 4);              // 0x0f RRCA

        assert_cyles_equal!([0x10], 4);              // 0x10 stop
        assert_cyles_equal!([0x11], 12);             // 0x11 LD DE,d16
        assert_cyles_equal!([0x12], 8);              // 0x12 ld (de) a
        assert_cyles_equal!([0x13], 8);              // 0x13 inc de
        assert_cyles_equal!([0x14], 4);              // 0x14 INC D
        assert_cyles_equal!([0x15], 4);              // 0x15 DEC D
        assert_cyles_equal!([0x16, 0x00], 8);        // 0x16 LD D,d8
        assert_cyles_equal!([0x17], 4);              // 0x17 RLA
        assert_cyles_equal!([0x18], 12);             // 0x18 JR r8
        assert_cyles_equal!([0x19], 8);              // 0x19 ADD HL,DE
        assert_cyles_equal!([0x1a], 8);              // 0x1a LD A,(DE)
        assert_cyles_equal!([0x1b], 8);              // 0x1b DEC DE
        assert_cyles_equal!([0x1c], 4);              // 0x1c INC E
        assert_cyles_equal!([0x1d], 4);              // 0x1d DEC E
        assert_cyles_equal!([0x1e], 8);              // 0x1e LD E,d8
        assert_cyles_equal!([0x1f], 4);              // 0x1f RRA

        assert_cyles_equal!([0x20, 0x00], 8);        // JR NZ,r8
        assert_cyles_equal!([0x20, 0x00], 12);       // JR NZ,r8
        assert_cyles_equal!([0x21], 8);              // LD HL,d16
        assert_cyles_equal!([0x22], 8);              // 0x22 ld (hl+) a
        assert_cyles_equal!([0x23], 8);              // 0x23 inc hl
        assert_cyles_equal!([0x32], 8);              // 0x32 ld (hl-) a
        assert_cyles_equal!([0x33], 8);              // 0x33 inc sp
    }
}
