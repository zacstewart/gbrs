use std::io::File;
use cpu::CPU;
use mmu::MMU;

mod cpu;
mod mmu;

fn main() {
  match File::open(&Path::new("data/Tetris.gb")).read_to_end() {
    Ok(contents) => {
      let program = contents.as_slice();
      let mut mmu: MMU = MMU::new();
      mmu.load_rom(program);
      let mut cpu: CPU = CPU::new(mmu);

      println!("Loaded ROM and beginning emulation");
      loop {
        let instruction = cpu.take_byte();
        cpu.execute(instruction);
      }
    },
    _ => fail!("Failed to read ROM.")
  }
}
