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

            // CPU emulation loop
            let mut cpu_stepper: i64 = 0;
            loop {
                cpu.step();
                cpu_stepper += 1;
                println!("STEP: {}", cpu_stepper);  // modify this to output relevant CPU state info

                // temp condition to break loop, will need to implement different logic later
                // to decide when to stop the emulation (eg special keypress, exit opcode, etc)
                // this is just a placeholder to illustrate breaking out of the loop
                if cpu_stepper >= 1_000_000 {
                    println!("reached 1 million steps, stopping emulation");    // this is an arbitrarily large number just for demonstration
                    break;
                }
            }
        },
        None => println!("No file selected."),
    }

    Ok(())
}
