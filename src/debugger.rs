use cpu::{Registers, Flags};
use std::io::{stdin, stdout, Write};
use std::process::exit;
use std::collections::HashSet;
use cpu::CPU;

#[derive(Debug)]
pub struct Debugger {
    pc: u16,
    pc_breaks: HashSet<u16>,
    instruction_breaks: HashSet<u8>,
    instruction: u8
}

impl Debugger {
    pub fn new() -> Debugger {
        Self {
            pc: 0,
            pc_breaks: HashSet::new(),
            instruction_breaks: HashSet::new(),
            instruction: 0
        }
    }

    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
    }

    pub fn add_pc_break(&mut self, pc: u16) {
        self.pc_breaks.insert(pc);
    }

    pub fn add_instr_break(&mut self, instruction: u8) {
        self.instruction_breaks.insert(instruction);
    }

    pub fn set_instruction(&mut self, instruction: u8) {
        self.instruction = instruction;
    }

    pub fn debug(&mut self, cpu: &mut CPU) {
        if self.pc_breaks.contains(&self.pc)  ||
                self.instruction_breaks.contains(&self.instruction) {
            self.start_debugger(cpu);
        }
    }

    fn start_debugger(&mut self, cpu: &mut CPU) {
        println!("Debugger started");

        loop {
            let cmd = self.get_command();
            match cmd.as_ref() {
                "c" => return,
                "e" => exit(0),
                "p" => self.show_state(cpu),
                "s" => cpu.step(self),
                cmd => { println!("Invalid command {}", cmd) }
            }
        }
    }

    fn show_state(&self, cpu: &CPU) {
            println!("
Registers:
a = {:03}
b = {:03} | c = {:03}
d = {:03} | e = {:03}
h = {:03} | l = {:03}
sp = {:04x} | pc = {:04x}
Flags:
z {} | n {} | h {} | c {}
op = {:02x}",
                cpu.registers.a,
                cpu.registers.b, cpu.registers.c,
                cpu.registers.d, cpu.registers.e,
                cpu.registers.h, cpu.registers.l,
                cpu.registers.sp, self.pc,
                cpu.flags.z, cpu.flags.n, cpu.flags.h, cpu.flags.c,
                self.instruction
                );
    }

    fn get_command(&self) -> String {
            print!("gbrs> ");
            stdout().flush().ok().expect("Couldn't flush stdout");
            let mut cmd = String::new();
            stdin().read_line(&mut cmd).expect("Couldn't read stdin");
            cmd.trim().to_string()
    }
}
