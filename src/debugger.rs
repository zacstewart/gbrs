use cpu::{Registers, Flags};
use memory_map::{ReadByte, WriteByte};
use std::io::{stdin, stdout, Write};
use std::process::exit;
use std::collections::HashSet;
use cpu::CPU;
use std::u16;
use std::str::SplitWhitespace;

#[derive(Clone)]
enum Command {
    Continue,
    DeleteBreak(usize),
    Exit,
    Memory(u16),
    Registers,
    Step,
    Unknown(String),
    Watch(Box<Command>)
}

#[derive(Eq,PartialEq)]
enum Breakpoint {
    Pc(u16),
    Instruction(u8)
}

pub struct Debugger {
    breakpoints: Vec<Breakpoint>,
    instruction: u8,
    pc: u16,
    step: bool,
    watches: Vec<Command>
}

impl Debugger {
    pub fn new() -> Debugger {
        Self {
            breakpoints: vec![],
            instruction: 0,
            pc: 0,
            step: false,
            watches: vec![]
        }
    }

    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
    }

    pub fn add_pc_break(&mut self, pc: u16) {
        self.breakpoints.push(Breakpoint::Pc(pc));
    }

    pub fn add_instr_break(&mut self, instruction: u8) {
        self.breakpoints.push(Breakpoint::Instruction(instruction));
    }

    pub fn set_instruction(&mut self, instruction: u8) {
        self.instruction = instruction;
    }

    pub fn debug(&mut self, cpu: &mut CPU) {
        self.run_watches(cpu);
        if self.breakpoints.contains(&Breakpoint::Pc(self.pc))  ||
                self.breakpoints.contains(&Breakpoint::Instruction(self.instruction)) ||
                self.step {
            self.step = false;
            self.start_debugger(cpu);
        }
    }

    fn run_command(&mut self, cpu: &mut CPU, cmd: Command) -> bool {
        match cmd {
            Command::Continue => return false,
            Command::DeleteBreak(i) => { self.breakpoints.remove(i); },
            Command::Exit => exit(0),
            Command::Memory(address) => self.show_memory(cpu, address),
            Command::Registers => self.show_state(cpu),
            Command::Step => {
                self.step = true;
                return false;
            },
            Command::Watch(cmd) => self.watches.push(*cmd),
            Command::Unknown(cmd) => { println!("Invalid command {}", cmd) }
        }

        true
    }

    fn run_watches(&mut self, cpu: &mut CPU) {
        let cmds = self.watches.clone();
        for cmd in cmds {
            self.run_command(cpu, cmd);
        }
    }

    fn start_debugger(&mut self, cpu: &mut CPU) {
        let mut run = true;
        while run {
            let cmd = self.get_command();
            run = self.run_command(cpu, cmd);
        }
    }

    fn show_state(&self, cpu: &CPU) {
            println!("
Registers:
a = {:02x}
b = {:02x} | c = {:02x}
d = {:02x} | e = {:02x}
h = {:02x} | l = {:02x}
sp = {:04x} | pc = {:04x}
Flags:
z {} | n {} | h {} | c {}
Interrupts enabled:
{}
op = {:02x}",
                cpu.registers.a,
                cpu.registers.b, cpu.registers.c,
                cpu.registers.d, cpu.registers.e,
                cpu.registers.h, cpu.registers.l,
                cpu.registers.sp, self.pc,
                cpu.flags.z, cpu.flags.n, cpu.flags.h, cpu.flags.c,
                cpu.interrupts,
                self.instruction
                );
    }

    fn show_memory(&self, cpu: &CPU, address: u16) {
        let mem_value = cpu.mmu.read_byte(address);
        println!("Memory [{:04x}] = {:02x}", address, mem_value);
    }

    fn get_command(&self) -> Command {
        print!("gbrs> ");
        stdout().flush().ok().expect("Couldn't flush stdout");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Couldn't read stdin");
        let mut cmd = input.split_whitespace();
        self.parse_command(&mut cmd)
    }

    fn parse_command(&self, cmd: &mut SplitWhitespace) -> Command {
        match cmd.next() {
            Some("c") => Command::Continue,
            Some("db") => {
                match cmd.next().map(|n| usize::from_str_radix(n, 10)) {
                    Some(Ok(i)) => Command::DeleteBreak(i),
                    Some(Err(err)) => Command::Unknown("Couldn't parse index".to_string()),
                    _ => Command::Unknown("Expected breakpoint index".to_string())
                }
            },
            Some("e") => Command::Exit,
            Some("m") => {
                match cmd.next().map(|n| u16::from_str_radix(n, 16)) {
                    Some(Ok(address)) => Command::Memory(address),
                    Some(Err(err)) => Command::Unknown("Couldn't parse address".to_string()),
                    _ => Command::Unknown("Expected memory address".to_string())
                }
            },
            Some("r") => Command::Registers,
            Some("s") => Command::Step,
            Some("w") => Command::Watch(Box::new(self.parse_command(cmd))),
            Some(c) => Command::Unknown(c.to_string()),
            None => Command::Unknown("Must provide a command".to_string())
        }
    }
}
