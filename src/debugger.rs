use cpu::CPU;

pub fn breakpoint(cpu: &CPU) {
    println!("a: {}", cpu.a);
}
