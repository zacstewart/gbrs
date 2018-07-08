#[macro_use]
extern crate log;

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
      0x80 => { let val = $this.register_b(); $this.add_a(val); }
      0x81 => { let val = $this.register_c(); $this.add_a(val); }
      0x82 => { let val = $this.register_d(); $this.add_a(val); }
      0x83 => { let val = $this.register_e(); $this.add_a(val); }
      0x84 => { let val = $this.register_h(); $this.add_a(val); }
      0x85 => { let val = $this.register_l(); $this.add_a(val); }
      0x86 => { let val = $this.address_hl(); $this.add_a(val); }
      0x87 => { let val = $this.register_a(); $this.add_a(val); }
      0x88 => { let val = $this.register_b(); $this.adc_a(val); }
      0x89 => { let val = $this.register_c(); $this.adc_a(val); }
      0x8a => { let val = $this.register_d(); $this.adc_a(val); }
      0x8b => { let val = $this.register_e(); $this.adc_a(val); }
      0x8c => { let val = $this.register_h(); $this.adc_a(val); }
      0x8d => { let val = $this.register_l(); $this.adc_a(val); }
      0x8e => { let val = $this.address_hl(); $this.adc_a(val); }
      0x8f => { let val = $this.register_a(); $this.adc_a(val); }
      0x90 => { let val = $this.register_b(); $this.sub(val); }
      0x91 => { let val = $this.register_c(); $this.sub(val); }
      0x92 => { let val = $this.register_d(); $this.sub(val); }
      0x93 => { let val = $this.register_e(); $this.sub(val); }
      0x94 => { let val = $this.register_h(); $this.sub(val); }
      0x95 => { let val = $this.register_l(); $this.sub(val); }
      0x96 => { let val = $this.address_hl(); $this.sub(val); }
      0x97 => { let val = $this.register_a(); $this.sub(val); }
      0x98 => { let val = $this.register_b(); $this.sbc_a(val); }
      0x99 => { let val = $this.register_c(); $this.sbc_a(val); }
      0x9a => { let val = $this.register_d(); $this.sbc_a(val); }
      0x9b => { let val = $this.register_e(); $this.sbc_a(val); }
      0x9c => { let val = $this.register_h(); $this.sbc_a(val); }
      0x9d => { let val = $this.register_l(); $this.sbc_a(val); }
      0x9e => { let val = $this.address_hl(); $this.sbc_a(val); }
      0x9f => { let val = $this.register_a(); $this.sbc_a(val); }
      0xa0 => { let val = $this.register_b(); $this.and(val); }
      0xa1 => { let val = $this.register_c(); $this.and(val); }
      0xa2 => { let val = $this.register_d(); $this.and(val); }
      0xa3 => { let val = $this.register_e(); $this.and(val); }
      0xa4 => { let val = $this.register_h(); $this.and(val); }
      0xa5 => { let val = $this.register_l(); $this.and(val); }
      0xa6 => { let val = $this.address_hl(); $this.and(val); }
      0xa7 => { let val = $this.register_a(); $this.and(val); }
      0xa8 => { let val = $this.register_b(); $this.xor(val); }
      0xa9 => { let val = $this.register_c(); $this.xor(val); }
      0xaa => { let val = $this.register_d(); $this.xor(val); }
      0xab => { let val = $this.register_e(); $this.xor(val); }
      0xac => { let val = $this.register_h(); $this.xor(val); }
      0xad => { let val = $this.register_l(); $this.xor(val); }
      0xae => { let val = $this.address_hl(); $this.xor(val); }
      0xaf => { let val = $this.register_a(); $this.xor(val); }
      0xb0 => { let val = $this.register_b(); $this.or(val); }
      0xb1 => { let val = $this.register_c(); $this.or(val); }
      0xb2 => { let val = $this.register_d(); $this.or(val); }
      0xb3 => { let val = $this.register_e(); $this.or(val); }
      0xb4 => { let val = $this.register_h(); $this.or(val); }
      0xb5 => { let val = $this.register_l(); $this.or(val); }
      0xb6 => { let val = $this.address_hl(); $this.or(val); }
      0xb7 => { let val = $this.register_a(); $this.or(val); }
      0xb8 => { let val = $this.register_b(); $this.cp(val); }
      0xb9 => { let val = $this.register_c(); $this.cp(val); }
      0xba => { let val = $this.register_d(); $this.cp(val); }
      0xbb => { let val = $this.register_e(); $this.cp(val); }
      0xbc => { let val = $this.register_h(); $this.cp(val); }
      0xbd => { let val = $this.register_l(); $this.cp(val); }
      0xbe => { let val = $this.address_hl(); $this.cp(val); }
      0xbf => { let val = $this.register_a(); $this.cp(val); }
      0xc0 => $this.ret_nz(),
      0xc1 => $this.pop_bc(),
      0xc2 => { let loc = $this.immediate_word(); $this.jp_nz(loc); }
      0xc3 => { let loc = $this.immediate_word(); $this.jp(loc); }
      0xc4 => { let val = $this.immediate_word(); $this.call_nz(val); }
      0xc5 => $this.push_bc(),
      0xc6 => { let val = $this.immediate(); $this.add_a(val); }
      0xc7 => $this.rst(0x00),
      0xc8 => $this.ret_z(),
      0xc9 => $this.ret(),
      0xca => { let loc = $this.immediate_word(); $this.jp_z(loc); }
      0xcb => { let op = $this.take_byte(); decode_prefixed_op!(op, $this); }
      0xcc => { let val = $this.immediate_word(); $this.call_z(val); }
      0xcd => { let val = $this.immediate_word(); $this.call(val); }
      0xce => { let val = $this.immediate(); $this.adc_a(val); }
      0xcf => $this.rst(0x08),
      0xd0 => $this.ret_nc(),
      0xd1 => $this.pop_de(),
      0xd2 => { let loc = $this.immediate_word(); $this.jp_nc(loc); }
      0xd3 => {}
      0xd4 => { let val = $this.immediate_word(); $this.call_nc(val); }
      0xd5 => $this.push_de(),
      0xd6 => { let val = $this.immediate(); $this.sub(val); }
      0xd7 => $this.rst(0x10),
      0xd8 => $this.ret_c(),
      0xd9 => $this.reti(),
      0xda => { let loc = $this.immediate_word(); $this.jp_c(loc); }
      0xdb => {}
      0xdc => { let val = $this.immediate_word(); $this.call_c(val); }
      0xdd => {}
      0xde => { let val = $this.immediate(); $this.sbc_a(val); }
      0xdf => $this.rst(0x18),
      0xe0 => { let loc = $this.immediate(); let val = $this.register_a(); $this.ldh_mem(loc, val); }
      0xe1 => $this.pop_hl(),
      0xe2 => { let loc = $this.address_c(); let val = $this.register_a(); $this.ld_mem(loc, val); }
      0xe5 => $this.push_hl(),
      0xe6 => { let val = $this.immediate(); $this.and(val); }
      0xe7 => $this.rst(0x20),
      0xe8 => { let val = $this.immediate_signed(); $this.add_sp(val); }
      0xe9 => { let loc = $this.register_hl(); $this.jp(loc); }
      0xea => { let val = $this.immediate_word_address(); $this.ld_mem_a(val); }
      0xee => { let val = $this.immediate(); $this.xor(val); }
      0xef => $this.rst(0x28),
      0xf0 => { let val = $this.immediate(); $this.ldh_a(val); }
      0xf1 => $this.pop_af(),
      0xf2 => { let val = $this.address_c(); $this.ld_a(val) }
      0xf3 => { $this.disable_interrupts() }
      0xf5 => $this.push_af(),
      0xf6 => { let val = $this.immediate(); $this.or(val); }
      0xf7 => $this.rst(0x30),
      0xf8 => { $this.ld_hl_sp_plus_immediate_signed() }
      0xf9 => { let am = $this.register_hl(); $this.ld_sp(am); }
      0xfa => { let am = $this.immediate_word_address(); $this.ld_a(am); }
      0xfb => { $this.enable_interrupts(); }
      0xfe => { let val = $this.immediate(); $this.cp(val); }
      0xff => $this.rst(0x38),
      _ => { panic!("Unknown OP: {:2x}", $op) }
    }
  }
}
macro_rules! decode_prefixed_op {
  ($op:expr, $this:ident) => {
    match $op {
        0x40 => { let val = $this.register_b(); $this.bit(0, val) }
        0x41 => { let val = $this.register_c(); $this.bit(0, val) }
        0x42 => { let val = $this.register_d(); $this.bit(0, val) }
        0x43 => { let val = $this.register_e(); $this.bit(0, val) }
        0x44 => { let val = $this.register_h(); $this.bit(0, val) }
        0x45 => { let val = $this.register_l(); $this.bit(0, val) }
        0x46 => { let val = $this.address_hl(); $this.bit(0, val) }
        0x47 => { let val = $this.register_a(); $this.bit(0, val) }
        0x48 => { let val = $this.register_b(); $this.bit(1, val) }
        0x49 => { let val = $this.register_c(); $this.bit(1, val) }
        0x4a => { let val = $this.register_d(); $this.bit(1, val) }
        0x4b => { let val = $this.register_e(); $this.bit(1, val) }
        0x4c => { let val = $this.register_h(); $this.bit(1, val) }
        0x4d => { let val = $this.register_l(); $this.bit(1, val) }
        0x4e => { let val = $this.address_hl(); $this.bit(1, val) }
        0x4f => { let val = $this.register_a(); $this.bit(1, val) }

        0x50 => { let val = $this.register_b(); $this.bit(2, val) }
        0x51 => { let val = $this.register_c(); $this.bit(2, val) }
        0x52 => { let val = $this.register_d(); $this.bit(2, val) }
        0x53 => { let val = $this.register_e(); $this.bit(2, val) }
        0x54 => { let val = $this.register_h(); $this.bit(2, val) }
        0x55 => { let val = $this.register_l(); $this.bit(2, val) }
        0x56 => { let val = $this.address_hl(); $this.bit(2, val) }
        0x57 => { let val = $this.register_a(); $this.bit(2, val) }
        0x58 => { let val = $this.register_b(); $this.bit(3, val) }
        0x59 => { let val = $this.register_c(); $this.bit(3, val) }
        0x5a => { let val = $this.register_d(); $this.bit(3, val) }
        0x5b => { let val = $this.register_e(); $this.bit(3, val) }
        0x5c => { let val = $this.register_h(); $this.bit(3, val) }
        0x5d => { let val = $this.register_l(); $this.bit(3, val) }
        0x5e => { let val = $this.address_hl(); $this.bit(3, val) }
        0x5f => { let val = $this.register_a(); $this.bit(3, val) }

        0x60 => { let val = $this.register_b(); $this.bit(4, val) }
        0x61 => { let val = $this.register_c(); $this.bit(4, val) }
        0x62 => { let val = $this.register_d(); $this.bit(4, val) }
        0x63 => { let val = $this.register_e(); $this.bit(4, val) }
        0x64 => { let val = $this.register_h(); $this.bit(4, val) }
        0x65 => { let val = $this.register_l(); $this.bit(4, val) }
        0x66 => { let val = $this.address_hl(); $this.bit(4, val) }
        0x67 => { let val = $this.register_a(); $this.bit(4, val) }
        0x68 => { let val = $this.register_b(); $this.bit(5, val) }
        0x69 => { let val = $this.register_c(); $this.bit(5, val) }
        0x6a => { let val = $this.register_d(); $this.bit(5, val) }
        0x6b => { let val = $this.register_e(); $this.bit(5, val) }
        0x6c => { let val = $this.register_h(); $this.bit(5, val) }
        0x6d => { let val = $this.register_l(); $this.bit(5, val) }
        0x6e => { let val = $this.address_hl(); $this.bit(5, val) }
        0x6f => { let val = $this.register_a(); $this.bit(5, val) }

        0x70 => { let val = $this.register_b(); $this.bit(6, val) }
        0x71 => { let val = $this.register_c(); $this.bit(6, val) }
        0x72 => { let val = $this.register_d(); $this.bit(6, val) }
        0x73 => { let val = $this.register_e(); $this.bit(6, val) }
        0x74 => { let val = $this.register_h(); $this.bit(6, val) }
        0x75 => { let val = $this.register_l(); $this.bit(6, val) }
        0x76 => { let val = $this.address_hl(); $this.bit(6, val) }
        0x77 => { let val = $this.register_a(); $this.bit(6, val) }
        0x78 => { let val = $this.register_b(); $this.bit(7, val) }
        0x79 => { let val = $this.register_c(); $this.bit(7, val) }
        0x7a => { let val = $this.register_d(); $this.bit(7, val) }
        0x7b => { let val = $this.register_e(); $this.bit(7, val) }
        0x7c => { let val = $this.register_h(); $this.bit(7, val) }
        0x7d => { let val = $this.register_l(); $this.bit(7, val) }
        0x7e => { let val = $this.address_hl(); $this.bit(7, val) }
        0x7f => { let val = $this.register_a(); $this.bit(7, val) }

        _ => println!("Prefixed op: {:2x}", $op)
    }
  }
}

pub mod cpu;
pub mod disasm;
pub mod mmu;
pub mod cartridge;
mod gpu;
mod joypad;
mod timer;
mod memory_map;
mod data;
mod debugger;
