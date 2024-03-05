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

  /// 0x00 is for NOP. This opcode doesn't do anything.
  /// 0xC3 is for JP nn. This is used to change the program counter (PC) to the address specified immediately after the opcode. It's a 3-byte instruction: the first byte is the opcode (0xC3), followed by two bytes that represent the address to jump to, in little-endian format (lower byte first).
  /// 0xFE is for CP n. This instruction compares the value 'n' with the accumulator 'A' by subtracting 'n' from 'A' and setting the flags accordingly, but without actually changing the value of 'A'.
  /// Flags affected: Z (zero flag): set if the result is 0. N (subtract flag) set to 1, indicating a subtraction operation. H (half carry flag): set if there is no borrow from bit 4. C (carry flag): set if there is no borrow.
  fn execute_opcode(&mut self, opcode: u8) {
    match opcode {
      0x00 => self.nop(),
      0xC3 => self.jp_nn(),
      0xFE => self.cp_n(),
      0x28 => self.jr_z_n(),
      // add implementations for more opcodes later, here.
      _ => panic!("Unimplemented opcode: 0x{:02X}", opcode),
    }
  }

  fn jr_z_n(&mut self) {
    let n = self.fetch_opcode() as i8;
    if self.check_flag(Flag::Z) {
      let jump_address = self.pc.wrapping_add(n as u16);
      self.pc = jump_address;
    }
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
  }

  fn jp_nn(&mut self) {
    let lower_byte = self.fetch_opcode() as u16;  // fetch the next byte as the lower part of the address
    let upper_byte = self.fetch_opcode() as u16;  // fetch the byte after that as the upper part of the address
    let new_address = (upper_byte << 8) | lower_byte; // combine the two bytes into a 16-bit address
    self.pc = new_address;  // set the program counter to the new address
  }
  
  fn nop(&self) {
    // NOP does nothing
  }
}
