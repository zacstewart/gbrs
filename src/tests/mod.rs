#[cfg(test)]

use cpu::CPU;
use mmu::MMU;
use cartridge::Cartridge;

#[test]
fn ld_bc_immediate_word() {
    let cart = Cartridge::load("data/ld_bc_d16.gb");
    let mut mmu: MMU = MMU::new();
    mmu.load_cartridge(cart);
    let mut cpu: CPU = CPU::new(mmu);
    cpu.execute();
    assert_eq!(cpu.b, 0xca);
    assert_eq!(cpu.c, 0xfe);
}

#[test]
fn registers_8bit_wrap_around_upon_overflow() {
    let mut mmu: MMU = MMU::new();
    mmu.load_rom("data/overflow_8bit_registers.gb");
    let mut cpu: CPU = CPU::new(mmu);
    cpu.execute();
    assert_eq!(cpu.b, 0);
    assert_eq!(cpu.c, 0);
    assert_eq!(cpu.d, 0);
    assert_eq!(cpu.e, 0);
    assert_eq!(cpu.a, 255);
}

#[test]
fn registers_8bit_wrap_around_upon_underflow() {
    let mut mmu: MMU = MMU::new();
    mmu.load_rom("data/underflow_8bit_registers.gb");
    let mut cpu: CPU = CPU::new(mmu);
    cpu.execute();
    assert_eq!(cpu.b, 255);
    assert_eq!(cpu.c, 255);
    assert_eq!(cpu.d, 255);
    assert_eq!(cpu.e, 255);
    assert_eq!(cpu.a, 1);
}
