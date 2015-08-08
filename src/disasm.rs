use mmu::MMU;

#[derive(Debug)]
pub struct Disassembler {
  mmu: MMU,
  pc: u16
}

impl Disassembler {
  pub fn new(mmu: MMU) -> Disassembler {
    Disassembler {
      mmu: mmu,
      pc: 0
    }
  }

  pub fn disassemble(&mut self, length: usize) {
    for _ in (0..length) {
      self.step();
    }
  }

  fn step(&mut self) {
    let instruction = self.take_byte();
    decode_op!(instruction, self);
  }

  fn take_byte(&mut self) -> u8 {
    let immediate = self.mmu.read_byte(self.pc);
    self.pc += 1;
    immediate
  }

  fn take_word(&mut self) -> u16 {
    let lower = self.take_byte() as u16;
    let upper = (self.take_byte() << 8) as u16;
    lower | upper
  }

  // Addressing modes

  fn immediate(&mut self) -> String {
    (format!("{}", self.take_byte())).to_string()
  }

  fn immediate_signed(&mut self) -> String {
    (format!("{}", self.take_byte() as i8)).to_string()
  }

  fn immediate_word(&mut self) -> String {
    (format!("{}", self.take_word())).to_string()
  }

  fn address_bc(&self) -> String {
    "(BC)".to_string()
  }

  fn address_de(&self) -> String {
    "(DE)".to_string()
  }

  fn address_hl(&self) -> String {
    "(HL)".to_string()
  }

  fn address_hli(&self) -> String {
    "(HL+)".to_string()
  }

  fn address_hld(&self) -> String {
    "(HL-)".to_string()
  }

  fn address_c(&self) -> String {
    "(C)".to_string()
  }

  fn register_b(&self) -> String {
    "B".to_string()
  }

  fn register_c(&self) -> String {
    "C".to_string()
  }

  fn register_d(&self) -> String {
    "D".to_string()
  }

  fn register_e(&self) -> String {
    "E".to_string()
  }

  fn register_h(&self) -> String {
    "H".to_string()
  }

  fn register_l(&self) -> String {
    "L".to_string()
  }

  fn register_a(&self) -> String {
    "A".to_string()
  }

  fn register_hl(&self) -> String {
    "HL".to_string()
  }

  // Operations

  fn nop(&self) {
    println!("NOP");
  }

  fn halt(&self) {
    println!("HALT");
  }

  fn stop(&self) {
    println!("STOP");
  }

  fn daa(&self) {
    println!("DAA");
  }

  fn cpl(&self) {
    println!("CPL");
  }

  fn scf(&self) {
    println!("SCF");
  }

  fn ccf(&self) {
    println!("CCF");
  }

  // Loads

  fn ld_bc(&self, am: String) {
    println!("LD BC,{}", am);
  }

  fn ld_de(&self, am: String) {
    println!("LD DE,{}", am);
  }

  fn ld_hl(&self, am: String) {
    println!("LD HL,{}", am);
  }

  fn ld_sp(&self, am: String) {
    println!("LD SP,{}", am);
  }

  fn ld_b(&self, am: String) {
    println!("LD B,{}", am);
  }

  fn ld_c(&self, am: String) {
    println!("LD C,{}", am);
  }

  fn ld_d(&self, am: String) {
    println!("LD D,{}", am);
  }

  fn ld_e(&self, am: String) {
    println!("LD E,{}", am);
  }

  fn ld_h(&self, am: String) {
    println!("LD H,{}", am);
  }

  fn ld_l(&self, am: String) {
    println!("LD L,{}", am);
  }

  fn ld_a(&self, am: String) {
    println!("LD A,{}", am);
  }

  fn ldh_a(&self, am: String) {
    println!("LDH A,{}", am);
  }

  fn ld_mem_sp(&self, am: String) {
    println!("LD {},SP", am);
  }

  fn ld_mem_a(&self, am: String) {
    println!("LD {},A", am);
  }

  fn ld_mem_hl(&self, am: String) {
    println!("LD {},HL", am);
  }

  fn ld_mem(&self, loc: String, val: String) {
    println!("LD {}, {}", loc, val);
  }

  fn ldh_mem(&self, loc: String, val: String) {
    println!("LDH {}, {}", loc, val);
  }

  fn pop_bc(&self) {
    println!("POP BC");
  }

  fn pop_de(&self) {
    println!("POP DE");
  }

  fn pop_hl(&self) {
    println!("POP HL");
  }

  fn pop_af(&self) {
    println!("PUSH AF");
  }

  fn push_bc(&self) {
    println!("PUSH BC");
  }

  fn push_de(&self) {
    println!("PUSH DE");
  }

  fn push_hl(&self) {
    println!("PUSH HL");
  }

  fn push_af(&self) {
    println!("PUSH AF");
  }

  // Increments & Decrements

  fn inc(&self, am: String) {
    println!("INC {}", am);
  }

  fn dec(&self, am: String) {
    println!("DEC {}", am);
  }

  fn inc_bc(&self) {
    println!("INC BC");
  }

  fn inc_de(&self) {
    println!("INC DE");
  }

  fn inc_hl(&self) {
    println!("INC HL");
  }

  fn inc_sp(&self) {
    println!("INC SP");
  }

  fn dec_bc(&self) {
    println!("DEC BC");
  }

  fn dec_de(&self) {
    println!("DEC DE");
  }

  fn dec_hl(&self) {
    println!("DEC HL");
  }

  fn dec_sp(&self) {
    println!("DEC SP");
  }

  fn inc_b(&self) {
    println!("INC B");
  }

  fn inc_c(&self) {
    println!("INC C");
  }

  fn inc_d(&self) {
    println!("INC D");
  }

  fn inc_e(&self) {
    println!("INC E");
  }

  fn inc_h(&self) {
    println!("INC H");
  }

  fn inc_l(&self) {
    println!("INC L");
  }

  fn inc_a(&self) {
    println!("INC A");
  }

  fn dec_b(&self) {
    println!("DEC B");
  }

  fn dec_c(&self) {
    println!("DEC C");
  }

  fn dec_d(&self) {
    println!("DEC D");
  }

  fn dec_e(&self) {
    println!("DEC E");
  }

  fn dec_h(&self) {
    println!("DEC H");
  }

  fn dec_l(&self) {
    println!("DEC L");
  }

  fn dec_a(&self) {
    println!("DEC A");
  }

  // Arithmetic

  fn add_hl_bc(&self) {
    println!("ADD HL,BC");
  }

  fn add_hl_de(&self) {
    println!("ADD HL,DE");
  }

  fn add_hl_hl(&self) {
    println!("ADD HL,HL");
  }

  fn add_hl_sp(&self) {
    println!("ADD HL,SP");
  }

  fn add_a(&self, am: String) {
    println!("ADD A,{}", am);
  }

  fn adc_a(&self, am: String) {
    println!("ADC A,{}", am);
  }

  fn cp(&self, am: String) {
    println!("CP {}", am);
  }

  fn sub(&self, am: String) {
    println!("SUB {}", am);
  }

  fn sbc_a(&self, am: String) {
    println!("SBC A,{}", am);
  }

  fn and(&self, am: String) {
    println!("AND {}", am);
  }

  fn xor(&self, am: String) {
    println!("XOR {}", am);
  }

  fn or(&self, am: String) {
    println!("OR {}", am);
  }

  // Rotations

  fn rlca(&self) {
    println!("RLCA");
  }

  fn rrca(&self) {
    println!("RRCA");
  }

  fn rla(&self) {
    println!("RLA");
  }

  fn rra(&self) {
    println!("RRA");
  }

  // Jumps

  fn jr(&self, am: String) {
    println!("JR {}", am);
  }

  fn jr_nz(&self, am: String) {
    println!("JR NZ,{}", am);
  }

  fn jr_nc(&self, am: String) {
    println!("JR NC,{}", am);
  }

  fn jr_z(&self, am: String) {
    println!("JR Z,{}", am);
  }

  fn jr_c(&self, am: String) {
    println!("JR C,{}", am);
  }

  fn jp_nz(&self, am: String) {
    println!("JP NZ,{}", am);
  }

  fn jp_nc(&self, am: String) {
    println!("JP NC,{}", am);
  }

  fn jp_z(&self, am: String) {
    println!("JP Z,{}", am);
  }

  fn jp_c(&self, am: String) {
    println!("JR C,{}", am);
  }

  fn jp(&self, am: String) {
    println!("JP {}", am);
  }

  fn ret_nz(&self) {
    println!("RET NZ");
  }

  fn ret_z(&self) {
    println!("RET Z");
  }

  fn ret_nc(&self) {
    println!("RET NC");
  }

  fn ret_c(&self) {
    println!("RET C");
  }

  fn ret(&self) {
    println!("RET");
  }

  fn reti(&self) {
    println!("RETI");
  }

  fn call_nz(&self, am: String) {
    println!("CALL NZ,{}", am);
  }

  fn call_z(&self, am: String) {
    println!("CALL Z,{}", am);
  }

  fn call_nc(&self, am: String) {
    println!("CALL NC,{}", am);
  }

  fn call_c(&self, am: String) {
    println!("CALL C,{}", am);
  }

  fn call(&self, am: String) {
    println!("CALL {}", am);
  }

  fn rst(&self, address: u16) {
    println!("RST {:x}H", address);
  }
}
