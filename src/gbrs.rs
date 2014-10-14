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
      0x76 => $this.halt(),
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
