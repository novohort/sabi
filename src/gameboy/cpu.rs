// Game Boy specific CPU emulation

pub struct CPU {
  // general purpose registers
  a: u8,  // accumulator
  f: u8,  // flags
  b: u8,  // general purpose
  c: u8,  // general purpose
  d: u8,  // general purpose
  e: u8,  // general purpose
  h: u8,  // general purpose
  l: u8,  // general purpose
  // special purpose registers
  sp: u16,  // stack pointer
  pc: u16,  // program counter
  // internal state
  memory: Vec<u8>,  // simplified memory model for now
  ime: bool,  // interrupt master enable flag
}

pub enum Flag {
  Z,  // zero
  N,  // subtract
  H,  // half carry
  C,  // carry
}

impl CPU {
  pub fn new(rom: Vec<u8>) -> CPU {
    CPU {
      a: 0,
      f: 0,
      b: 0,
      c: 0,
      d: 0,
      e: 0,
      h: 0,
      l: 0,
      sp: 0xFFFE, // initial stack pointer value
      pc: 0x0100, // execution begins at 0x0100
      memory: rom,  // load the ROM into memory
      ime: true,  // assume interrupts are enabled by default, adjust depending on needs;
    }
  }

  pub fn step(&mut self) {
    let opcode = self.fetch_opcode();
    self.execute_opcode(opcode);
  }

  fn fetch_opcode(&mut self) -> u8 {
    let opcode = self.memory[self.pc as usize];
    self.pc += 1;
    opcode
  }

  fn set_flag(&mut self, flag: Flag, value: bool) {
    match flag {
      Flag::Z => self.f = if value { self.f | 0x80 } else { self.f & 0x7F },
      Flag::N => self.f = if value { self.f | 0x40 } else { self.f & 0xBF },
      Flag::H => self.f = if value { self.f | 0x20 } else { self.f & 0xDF },
      Flag::C => self.f = if value { self.f | 0x10 } else { self.f & 0xEF },
    }
  }

  fn check_flag(&self, flag: Flag) -> bool {
    match flag {
      Flag::Z => (self.f & 0x80) != 0,
      Flag::N => (self.f & 0x40) != 0,
      Flag::H => (self.f & 0x20) != 0,
      Flag::C => (self.f & 0x10) != 0,
    }
  }


  fn execute_opcode(&mut self, opcode: u8) {
    match opcode {
      0x00 => self.nop(),
      0xC3 => self.jp_nn(),
      0xFE => self.cp_n(),
      0x28 => self.jr_z_n(),
      0xAF => self.xor_a(),
      0x18 => self.jr_n(),
      0xEA => self.ld_nn_a(),
      0xF3 => self.di(),
      0xE0 => self.ldh_n_a(),
      0x3E => self.ld_a_n(),
      0xCD => self.call_nn(),
      0xF0 => self.ldh_a_n(),
      // add implementations for more opcodes later, here.
      _ => panic!("Unimplemented opcode: 0x{:02X}", opcode),
    }
  }

  fn ldh_a_n(&mut self) {
    let n = self.fetch_opcode() as u16;
    let address = 0xFF00 + n;
    self.a = self.memory[address as usize];
    println!("OPCODE RAN: LDH_A_N")
  }

  fn call_nn(&mut self) {
    let lower_byte = self.fetch_opcode() as u16;
    let upper_byte = self.fetch_opcode() as u16;
    let address = (upper_byte << 8) | lower_byte;

    // push the current PC onto the stack. note that PC points to the next instruction
    self.sp = self.sp.wrapping_sub(1);
    self.memory[self.sp as usize] = ((self.pc >> 8) & 0xFF) as u8;
    self.sp = self.sp.wrapping_sub(1);
    self.memory[self.sp as usize] = (self.pc & 0xFF) as u8;

    self.pc = address;
    println!("OPCODE RAN: CALL_NN");
  }

  fn ld_a_n(&mut self) {
    self.a = self.fetch_opcode();
    println!("OPCODE RAN: LD_A_N");
  }

  fn ldh_n_a(&mut self) {
    let n = self.fetch_opcode() as u16;
    let address = 0xFF00 + n;
    self.memory[address as usize] = self.a;
    println!("OPCODE RAN: LDF_N_A");
  }

  fn di(&mut self) {
    self.ime = false; // disables interrupts by clearing the IME flag
    println!("OPCODE RAN: DI");
  }

  fn ld_nn_a(&mut self) {
    let lower_byte = self.fetch_opcode() as u16;
    let upper_byte = self.fetch_opcode() as u16;
    let address = (upper_byte << 8) | lower_byte;

    self.memory[address as usize] = self.a;
    println!("OPCODE RAN: LD_NN_A");
  }

  fn jr_n(&mut self) {
    let n = self.fetch_opcode() as i8;
    self.pc = self.pc.wrapping_add(n as u16);
    println!("OPCODE RAN: JR_N");
  }

  fn xor_a(&mut self) {
    self.a ^= self.a; // this performs XOR on A with tiself, always results in 0

    // reset A to 0, set Z flag, and reset N, H, and C flags
    self.set_flag(Flag::Z, true); // a XOR A will always set the Z flag
    self.set_flag(Flag::N, false);
    self.set_flag(Flag::H, false);
    self.set_flag(Flag::C, false);
    println!("OPCODE RAN: XOR_A");
  }

  fn jr_z_n(&mut self) {
    let n = self.fetch_opcode() as i8;
    if self.check_flag(Flag::Z) {
      let jump_address = self.pc.wrapping_add(n as u16);
      self.pc = jump_address;
    }
    println!("OPCODE RAN: JR_Z_N");
  }

  fn cp_n(&mut self) {
    let n = self.fetch_opcode();
    let a = self.a;
    let result = a.wrapping_sub(n);

    // set the zero flag if the result is 0
    self.set_flag(Flag::Z, result == 0);

    // set the subtract flag if the operation is a subtraction
    self.set_flag(Flag::N, true);

    // set the half carry flag if borrow from bit 4 occurred
    self.set_flag(Flag::H, (a & 0xF) < (n & 0xF));

    // set the carry flag if there is a borrow
    self.set_flag(Flag::C, a < n);
    println!("OPCODE RAN: CP_N");
  }

  fn jp_nn(&mut self) {
    let lower_byte = self.fetch_opcode() as u16;  // fetch the next byte as the lower part of the address
    let upper_byte = self.fetch_opcode() as u16;  // fetch the byte after that as the upper part of the address
    let new_address = (upper_byte << 8) | lower_byte; // combine the two bytes into a 16-bit address
    self.pc = new_address;  // set the program counter to the new address
    println!("OPCODE RAN: JP_NN");
  }
  
  fn nop(&self) {
    // NOP does nothing
    println!("OPCODE RAN: NOP");
  }
}
