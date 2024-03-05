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

  fn execute_opcode(&mut self, opcode: u8) {
    match opcode {
      0x00 => self.nop(),
      // add implementations for more opcodes later, here.
      _ => panic!("Unimplemented opcode: 0x{:02X}", opcode),
    }
  }

  // opcode implementations
  fn nop(&self) {
    // NOP does nothing
  }
}
