#![allow(dead_code)]
#![feature(convert)]
extern crate sdl2;
extern crate gbrs;

use gbrs::cpu::CPU;
use gbrs::disasm::Disassembler;
use gbrs::mmu::MMU;
use gbrs::cartridge::Cartridge;
use std::convert::AsRef;
use std::env;

fn main() {
  let args: Vec<_> = env::args().collect();

  let cart = Cartridge::load(&args[2]);
  let size = cart.size();
  let mut mmu: MMU = MMU::new();
  mmu.load_cartridge(cart);

  match args[1].as_ref() {
      "run" => {
          println!("Loading ROM and beginning emulation");
          let mut cpu: CPU = CPU::new(mmu);
          while !cpu.stopped {
              cpu.step();
          }
      }
      "disasm" => {
          let mut disasm = Disassembler::new(mmu);
          disasm.disassemble(size);
      }
      _ => {}
  }
}
