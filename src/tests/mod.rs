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
    let cart = Cartridge::load("data/overflow_8bit_registers.gb");
    let mut mmu: MMU = MMU::new();
    mmu.load_cartridge(cart);
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
    let cart = Cartridge::load("data/underflow_8bit_registers.gb");
    let mut mmu: MMU = MMU::new();
    mmu.load_cartridge(cart);
    let mut cpu: CPU = CPU::new(mmu);
    cpu.execute();
    assert_eq!(cpu.b, 255);
    assert_eq!(cpu.c, 255);
    assert_eq!(cpu.d, 255);
    assert_eq!(cpu.e, 255);
    assert_eq!(cpu.a, 1);
}

#[test]
fn registers_16bit_wrap_around_upon_overflow() {
    let rom: Vec<u8> = vec!(
        0x01, 0xff, 0xff,   // LD BC, 65535
        0x03,               // INC BC

        0x11, 0xff, 0xff,   // LD DE, 65535
        0x13,               // INC DE

        0x21, 0xff, 0xff,   // LD HL, 65535
        0x23,               // INC HL

        0x31, 0xff, 0xff,   // LD SP, 65535
        0x33,               // INC SP

        0x10                // STOP
    );
    let cart = Cartridge::new(rom.into_boxed_slice());
    let mut mmu: MMU = MMU::new();
    mmu.load_cartridge(cart);
    let mut cpu: CPU = CPU::new(mmu);
    cpu.execute();
    assert_eq!(cpu.b, 0);
    assert_eq!(cpu.c, 0);
    assert_eq!(cpu.d, 0);
    assert_eq!(cpu.e, 0);
    assert_eq!(cpu.h, 0);
    assert_eq!(cpu.l, 0);
    assert_eq!(cpu.sp, 0);
}

#[test]
fn registers_16bt_wrap_around_upon_underflow() {
    let rom: Vec<u8> = vec!(
        0x01, 0x00, 0x00,   // LD BC, 0
        0x0b,               // DEC BC

        0x11, 0x00, 0x00,   // LD DE, 0
        0x1b,               // DEC DE

        0x21, 0x00, 0x00,   // LD HL, 0
        0x2b,               // DEC HL

        0x31, 0x00, 0x00,   // LD SP, 0
        0x3b,               // DEC SP

        0x10                // STOP
    );
    let cart = Cartridge::new(rom.into_boxed_slice());
    let mut mmu: MMU = MMU::new();
    mmu.load_cartridge(cart);
    let mut cpu: CPU = CPU::new(mmu);
    cpu.execute();
    assert_eq!(cpu.b, 255);
    assert_eq!(cpu.c, 255);
    assert_eq!(cpu.d, 255);
    assert_eq!(cpu.e, 255);
    assert_eq!(cpu.h, 255);
    assert_eq!(cpu.l, 255);
    assert_eq!(cpu.sp, 65535);
}
