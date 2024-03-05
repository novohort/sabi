mod gameboy;
use gameboy::cpu::CPU;
use rfd::FileDialog;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
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

            // Placeholder for CPU emulation loop
            // Replace with actual emulation loop later
            let mut cpu_stepper: i16 = 0;
            for _ in 0..10 { // Limiting steps for demonstration
                cpu.step();
                cpu_stepper = cpu_stepper + 1;
                println!("STEP: {:?}", cpu_stepper); // Modify this to output relevant CPU state information
            }
        },
        None => println!("No file selected."),
    }

    Ok(())
}
