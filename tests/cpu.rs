extern crate gbrs;
use gbrs::cpu::CPU;
use gbrs::mmu::MMU;
use gbrs::cartridge::Cartridge;
use std::num::Wrapping as W;

macro_rules! make_rom {
    ($($rest:expr),*) => {{
        let mut rom = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            // Entry point
            0x00, 0xc3, 0x50, 0x01,
            // Logo
            0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0c, 0x00, 0x0d,
            0x00, 0x08, 0x11, 0x1f, 0x88, 0x89, 0x00, 0x0e, 0xdc, 0xcc, 0x6e, 0xe6, 0xdd, 0xdd, 0xd9, 0x99,
            0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc, 0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e,
            // Title
            0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21, 0x21,
            // Manufacturer code
            0x00, 0x00,
            // SGB Flag
            0x00,
            // Cartridge type
            0x00,
            // ROM size
            0x00,
            // RAM size
            0x00,
            // Destination code
            0x01,
            // Old licensee code
            0x00,
            // Mask ROM version number
            0x00,
            // Header checksum
            0x00,
            // Global checksum
            0x00, 0x00,
            $($rest),*
        ];
        let mut checksum = 0u8;
        for i in (0x0134..0x014c) {
            checksum = (W(checksum) + W(rom[i])).0;
        }
        rom[0x014d] = checksum;
        rom.into_boxed_slice()
    }}
}

#[test]
fn ld_bc_immediate_word() {
    let mut rom = make_rom!(0x01, 0xfe, 0xca, 0x10);

    let cart = Cartridge::new(rom);
    let mut mmu: MMU = MMU::new();
    mmu.load_cartridge(cart);
    let mut cpu: CPU = CPU::new(mmu);
    let mut i = 0;
    while !cpu.stopped {
        i += 1;
        cpu.step();
    }
    assert_eq!(cpu.b, 0xca);
    assert_eq!(cpu.c, 0xfe);
}

#[test]
fn registers_8bit_wrap_around_upon_overflow() {
    let rom = make_rom!(
        0x06, 0xff, // LD B, 0xff
        0x0e, 0xff, // LD C, 0xff
        0x16, 0xff, // LD D, 0xff
        0x1e, 0xff, // LD E, 0xff
        0x26, 0xff, // LD H, 0xff
        0x2e, 0xff, // LD L, 0xff
        0x3e, 0xff, // LD A, 0xff
        0x04,       // INC B
        0x0c,       // INC C
        0x14,       // INC D
        0x1c,       // INC E
        0x24,       // INC H
        0x2c,       // INC L
        0x3c,       // INC A
        0x10        // STOP
    );
    let cart = Cartridge::new(rom);
    let mut mmu: MMU = MMU::new();
    mmu.load_cartridge(cart);
    let mut cpu: CPU = CPU::new(mmu);
    while !cpu.stopped {
        cpu.step();
    }
    assert_eq!(cpu.b, 0);
    assert_eq!(cpu.c, 0);
    assert_eq!(cpu.d, 0);
    assert_eq!(cpu.e, 0);
    assert_eq!(cpu.h, 0);
    assert_eq!(cpu.l, 0);
    assert_eq!(cpu.a, 0);
}

#[test]
fn registers_8bit_wrap_around_upon_underflow() {
    let mut rom = make_rom!(
        0x06, 0x00, // LD B, 0x00
        0x0e, 0x00, // LD C, 0x00
        0x16, 0x00, // LD D, 0x00
        0x1e, 0x00, // LD E, 0x00
        0x26, 0x00, // LD H, 0x00
        0x2e, 0x00, // LD L, 0x00
        0x3e, 0x00, // LD A, 0x00
        0x05,       // DEC B
        0x0d,       // DEC C
        0x15,       // DEC D
        0x1d,       // DEC E
        0x25,       // DEC H
        0x2d,       // DEC L
        0x3d,       // DEC A
        0x10        // STOP
    );
    let cart = Cartridge::new(rom);
    let mut mmu: MMU = MMU::new();
    mmu.load_cartridge(cart);
    let mut cpu: CPU = CPU::new(mmu);
    while !cpu.stopped {
        cpu.step();
    }
    assert_eq!(cpu.b, 255);
    assert_eq!(cpu.c, 255);
    assert_eq!(cpu.d, 255);
    assert_eq!(cpu.e, 255);
    assert_eq!(cpu.h, 255);
    assert_eq!(cpu.l, 255);
    assert_eq!(cpu.a, 255);
}

#[test]
fn registers_16bit_wrap_around_upon_overflow() {
    let mut rom = make_rom!(
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
    let cart = Cartridge::new(rom);
    let mut mmu: MMU = MMU::new();
    mmu.load_cartridge(cart);
    let mut cpu: CPU = CPU::new(mmu);
    while !cpu.stopped {
        cpu.step();
    }
    assert_eq!(cpu.b, 0);
    assert_eq!(cpu.c, 0);
    assert_eq!(cpu.d, 0);
    assert_eq!(cpu.e, 0);
    assert_eq!(cpu.h, 0);
    assert_eq!(cpu.l, 0);
    assert_eq!(cpu.sp, 0);
}

#[test]
fn registers_16bit_wrap_around_upon_underflow() {
    let mut rom = make_rom!(
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
    let cart = Cartridge::new(rom);
    let mut mmu: MMU = MMU::new();
    mmu.load_cartridge(cart);
    let mut cpu: CPU = CPU::new(mmu);
    while !cpu.stopped {
        cpu.step();
    }
    assert_eq!(cpu.b, 255);
    assert_eq!(cpu.c, 255);
    assert_eq!(cpu.d, 255);
    assert_eq!(cpu.e, 255);
    assert_eq!(cpu.h, 255);
    assert_eq!(cpu.l, 255);
    assert_eq!(cpu.sp, 65535);
}
