mod gameboy;
use gameboy::cpu::CPU;
use minifb::{Key, Window, WindowOptions};
use rfd::FileDialog;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut ppu = gameboy::ppu::PPU::new();
    let mut window = Window::new(
        "Sabi Emulator - ESC to exit",
        160, // gameboy screen width
        144, // gameboy screen height
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // set up buffer for the window pixels
    let mut buffer: Vec<u32> = vec![0; 160 * 144];

    let file = FileDialog::new()
        .add_filter("Game Boy ROM", &["gb"])
        .pick_file();

    match file {
        Some(path) => {
            println!("Selected file: {:?}", path);

            let mut file = File::open(path)?;
            let mut rom = Vec::new();
            file.read_to_end(&mut rom)?;

            let mut cpu = CPU::new(rom);

            // CPU emulation loop
            let mut cpu_stepper: i64 = 0;
            loop {
                cpu.step();
                let memory = cpu.get_memory();
                ppu.step(&mut buffer);
                ppu.render_background(&mut buffer, memory);
                cpu_stepper += 1;
                // println!("STEP: {}", cpu_stepper); // modify this to output relevant CPU state info

                if window.is_open() && !window.is_key_down(Key::Escape) {
                    window.update_with_buffer(&buffer, 160, 144).unwrap();
                } else {
                    break;
                }

                // temp condition to break loop, will need to implement different logic later
                // to decide when to stop the emulation (eg special keypress, exit opcode, etc)
                // this is just a placeholder to illustrate breaking out of the loop
                if cpu_stepper >= 1_000_000 {
                    println!("reached 1 million steps, stopping emulation"); // this is an arbitrarily large number just for demonstration
                    break;
                }
            }
        }
        None => println!("No file selected."),
    }

    Ok(())
}
