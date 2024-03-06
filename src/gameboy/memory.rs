pub struct Memory {
    pub data: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        Memory {
            data: vec![0; size],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        *self.data.get(address as usize).unwrap_or(&0xFF)
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        if let Some(byte) = self.data.get_mut(address as usize) {
            *byte = value;
        }
    }

    pub fn load_rom(&mut self, rom_path: &str) {
        let rom_data = std::fs::read(rom_path).expect("Failed to read ROM file");
        for (index, &byte) in rom_data.iter().enumerate() {
            self.data[index] = byte;
        }
        println!("Successfully loaded rom into memory");
    }
}
