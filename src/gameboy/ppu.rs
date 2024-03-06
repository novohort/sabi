use crate::gameboy::memory::Memory;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

pub struct PPU {
    pub scanline_counter: usize,
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            scanline_counter: 0,
        }
    }

    fn get_color(&self, color_id: u8) -> u32 {
        // assume simple grayscale palette for demonstration
        match color_id {
            0 => 0xFFFFFF,
            1 => 0xAAAAAA,
            2 => 0x555555,
            _ => 0x000000,
        }
    }

    pub fn render_background(&self, buffer: &mut [u32], memory: &Memory) {
        let tile_data_start = 0x8000; // base addr for tile data
        let bg_map_start = 0x9800; // base addr for background map

        for map_y in 0..32 {
            for map_x in 0..32 {
                let tile_index_address = bg_map_start + map_y * 32 + map_x;
                let tile_index = memory.read_byte(tile_index_address) as u16;
                let tile_addr = tile_data_start + tile_index * 16; // calc address for tile data

                for tile_y in 0..8 {
                    let addr = tile_addr + tile_y * 2;
                    let byte1 = memory.read_byte(addr);
                    let byte2 = memory.read_byte(addr + 1);

                    for tile_x in 0..8 {
                        let bit_index = 7 - tile_x;
                        let color_bit =
                            ((byte1 >> bit_index) & 1) | (((byte2 >> bit_index) & 1) << 1);
                        let color = self.get_color(color_bit);

                        // calc actual x, y positions on the screen
                        let x = (map_x * 8 + tile_x) as usize;
                        let y = (map_y * 8 + tile_y) as usize;

                        // dont draw outside buffer
                        if x < SCREEN_WIDTH && y < SCREEN_HEIGHT {
                            buffer[y * SCREEN_WIDTH + x] = color;
                        }
                    }
                }
            }
        }
    }

    pub fn step(&mut self, buffer: &mut [u32]) {
        self.scanline_counter = (self.scanline_counter + 1) % 144;
        let color = if self.scanline_counter % 2 == 0 {
            0xFFFFFF
        } else {
            0x000000
        };

        for y in 0..144 {
            for x in 0..160 {
                buffer[y * 160 + x] = color;
            }
        }
    }
}
