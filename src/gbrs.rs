#![feature(macro_rules)]

use std::io::File;
use cpu::CPU;
use disasm::Disassembler;
use mmu::MMU;

macro_rules! decode_op {
  ($op:expr, $this:ident) => {
    match $op {
      0x00 => $this.nop(),
      0x01 => { let v = $this.immediate_word(); $this.ld_bc(v); },
      0x02 => { let am = $this.address_bc(); $this.ld_mem_a(am); },
      0x03 => $this.inc_bc(),
      0x04 => $this.inc_b(),
      0x05 => $this.dec_b(),
      0x06 => { let v = $this.immediate(); $this.ld_b(v); },
      0x07 => $this.rlca(),
      0x08 => { let am = $this.immediate_word(); $this.ld_mem_sp(am); },
      0x09 => $this.add_hl_bc(),
      0x0a => { let am = $this.address_bc(); $this.ld_a(am); },
      0x0b => $this.dec_bc(),
      0x0c => $this.inc_c(),
      0x0d => $this.dec_c(),
      0x0e => { let am = $this.immediate(); $this.ld_c(am); },
      0x0f => $this.rrca(),
      0x10 => $this.stop(),
      0x11 => { let am = $this.immediate_word(); $this.ld_de(am) },
      0x12 => { let am = $this.address_de(); $this.ld_mem_a(am); },
      0x13 => $this.inc_de(),
      0x14 => $this.inc_d(),
      0x15 => $this.dec_d(),
      0x16 => { let am = $this.immediate(); $this.ld_d(am); },
      0x17 => $this.rla(),
      0x18 => { let v = $this.immediate_signed(); $this.jr(v) },
      0x19 => $this.add_hl_de(),
      0x1a => { let am = $this.address_de(); $this.ld_a(am); },
      0x1b => $this.dec_de(),
      0x1c => $this.inc_e(),
      0x1d => $this.dec_e(),
      0x1e => { let am = $this.immediate(); $this.ld_e(am); },
      0x1f => $this.rra(),
      0x20 => { let v = $this.immediate_signed(); $this.jr_nz(v); },
      0x21 => { let am = $this.immediate_word(); $this.ld_hl(am); },
      0x22 => { let am = $this.address_hli(); $this.ld_mem_a(am) },
      0x23 => $this.inc_hl(),
      0x24 => $this.inc_h(),
      0x25 => $this.dec_h(),
      0x26 => { let am = $this.immediate(); $this.ld_h(am); },
      0x27 => { $this.daa(); }
      0x28 => { let v = $this.immediate_signed(); $this.jr_z(v); },
      0x29 => $this.add_hl_hl(),
      0x2a => { let am = $this.address_hli(); $this.ld_a(am); },
      0x2b => $this.dec_hl(),
      0x2c => $this.inc_l(),
      0x2d => $this.dec_l(),
      0x2e => { let am = $this.immediate(); $this.ld_l(am); },
      0x2f => $this.cpl(),
      0x30 => { let v = $this.immediate_signed(); $this.jr_nc(v); },
      0x31 => { let am = $this.immediate_word(); $this.ld_sp(am); },
      0x32 => { let am = $this.address_hld(); $this.ld_mem_a(am); },
      0x33 => $this.inc_sp(),
      0x34 => { let am = $this.address_hl(); $this.inc(am); },
      0x35 => { let am = $this.address_hl(); $this.dec(am); },
      0x36 => { let loc = $this.address_hl(); let val = $this.immediate(); $this.ld_mem(loc, val); }
      0x37 => $this.scf(),
      0x38 => { let v = $this.immediate_signed(); $this.jr_c(v); },
      0x39 => $this.add_hl_sp(),
      0x3a => { let am = $this.address_hld(); $this.ld_a(am); },
      0x3b => $this.dec_sp(),
      0x3c => $this.inc_a(),
      0x3d => $this.dec_a(),
      0x3e => { let am = $this.immediate(); $this.ld_a(am); },
      0x3f => $this.ccf(),
      0x40 => { let val = $this.register_b(); $this.ld_b(val); }
      0x41 => { let val = $this.register_c(); $this.ld_b(val); }
      0x42 => { let val = $this.register_d(); $this.ld_b(val); }
      0x43 => { let val = $this.register_e(); $this.ld_b(val); }
      0x44 => { let val = $this.register_h(); $this.ld_b(val); }
      0x45 => { let val = $this.register_l(); $this.ld_b(val); }
      0x46 => { let val = $this.address_hl(); $this.ld_b(val); }
      0x47 => { let val = $this.register_a(); $this.ld_b(val); }
      0x48 => { let val = $this.register_b(); $this.ld_c(val); }
      0x49 => { let val = $this.register_c(); $this.ld_c(val); }
      0x4a => { let val = $this.register_d(); $this.ld_c(val); }
      0x4b => { let val = $this.register_e(); $this.ld_c(val); }
      0x4c => { let val = $this.register_h(); $this.ld_c(val); }
      0x4d => { let val = $this.register_l(); $this.ld_c(val); }
      0x4e => { let val = $this.address_hl(); $this.ld_c(val); }
      0x4f => { let val = $this.register_a(); $this.ld_c(val); }
      0x50 => { let val = $this.register_b(); $this.ld_d(val); }
      0x51 => { let val = $this.register_c(); $this.ld_d(val); }
      0x52 => { let val = $this.register_d(); $this.ld_d(val); }
      0x53 => { let val = $this.register_e(); $this.ld_d(val); }
      0x54 => { let val = $this.register_h(); $this.ld_d(val); }
      0x55 => { let val = $this.register_l(); $this.ld_d(val); }
      0x56 => { let val = $this.address_hl(); $this.ld_d(val); }
      0x57 => { let val = $this.register_a(); $this.ld_d(val); }
      0x58 => { let val = $this.register_b(); $this.ld_e(val); }
      0x59 => { let val = $this.register_c(); $this.ld_e(val); }
      0x5a => { let val = $this.register_d(); $this.ld_e(val); }
      0x5b => { let val = $this.register_e(); $this.ld_e(val); }
      0x5c => { let val = $this.register_h(); $this.ld_e(val); }
      0x5d => { let val = $this.register_l(); $this.ld_e(val); }
      0x5e => { let val = $this.address_hl(); $this.ld_e(val); }
      0x5f => { let val = $this.register_a(); $this.ld_e(val); }
      0x60 => { let val = $this.register_b(); $this.ld_h(val); }
      0x61 => { let val = $this.register_c(); $this.ld_h(val); }
      0x62 => { let val = $this.register_d(); $this.ld_h(val); }
      0x63 => { let val = $this.register_e(); $this.ld_h(val); }
      0x64 => { let val = $this.register_h(); $this.ld_h(val); }
      0x65 => { let val = $this.register_l(); $this.ld_h(val); }
      0x66 => { let val = $this.address_hl(); $this.ld_h(val); }
      0x67 => { let val = $this.register_a(); $this.ld_h(val); }
      0x68 => { let val = $this.register_b(); $this.ld_l(val); }
      0x69 => { let val = $this.register_c(); $this.ld_l(val); }
      0x6a => { let val = $this.register_d(); $this.ld_l(val); }
      0x6b => { let val = $this.register_e(); $this.ld_l(val); }
      0x6c => { let val = $this.register_h(); $this.ld_l(val); }
      0x6d => { let val = $this.register_l(); $this.ld_l(val); }
      0x6e => { let val = $this.address_hl(); $this.ld_l(val); }
      0x6f => { let val = $this.register_a(); $this.ld_l(val); }
      0x70 => { let loc = $this.address_hl(); let val = $this.register_b(); $this.ld_mem(loc, val); }
      0x71 => { let loc = $this.address_hl(); let val = $this.register_c(); $this.ld_mem(loc, val); }
      0x72 => { let loc = $this.address_hl(); let val = $this.register_d(); $this.ld_mem(loc, val); }
      0x73 => { let loc = $this.address_hl(); let val = $this.register_e(); $this.ld_mem(loc, val); }
      0x74 => { let loc = $this.address_hl(); let val = $this.register_h(); $this.ld_mem(loc, val); }
      0x75 => { let loc = $this.address_hl(); let val = $this.register_l(); $this.ld_mem(loc, val); }
      0x76 => $this.halt(),
      0x77 => { let loc = $this.address_hl(); let val = $this.register_a(); $this.ld_mem(loc, val); }
      0x78 => { let val = $this.register_b(); $this.ld_a(val); }
      0x79 => { let val = $this.register_c(); $this.ld_a(val); }
      0x7a => { let val = $this.register_d(); $this.ld_a(val); }
      0x7b => { let val = $this.register_e(); $this.ld_a(val); }
      0x7c => { let val = $this.register_h(); $this.ld_a(val); }
      0x7d => { let val = $this.register_l(); $this.ld_a(val); }
      0x7e => { let val = $this.address_hl(); $this.ld_a(val); }
      0x7f => { let val = $this.register_a(); $this.ld_a(val); }
      _ => {}//println!("{}", $this)
    }
  }
}

mod cpu;
mod disasm;
mod mmu;

fn main() {
  match File::open(&Path::new("data/Tetris.gb")).read_to_end() {
    Ok(contents) => {
      let program = contents.as_slice();
      let mut mmu: MMU = MMU::new();
      mmu.load_rom(program);
      let mut cpu: CPU = CPU::new(mmu);
      let mut disasm = Disassembler::new(mmu);

      println!("Loaded ROM and beginning emulation");
      disasm.disassemble(contents.len());
      //cpu.execute();
    },
    _ => fail!("Failed to read ROM.")
  }
}
