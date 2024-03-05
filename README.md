# Welcome to Sabi

In response to Nintendo slapping Yuzu with a fat lawsuit, we're creating Sabi as a new open source emulation platform. The goal for Sabi is, quite honestly, utterly fucking insane. We're aiming to be able to emulate a variety of systems, and we'd like to build for cross-platform compatibility. With that being said, while our goals are extremely ambitious, we're going to start small with the Gameboy — objectively one of the easier platforms to emulate.

Reminder that emulation is legal and is morally correct.

We do not condone piracy.

## Why Sabi?

Sabi is a direct japanese translation for "rust", the language the emulator is built in. We thought it sounded cool, and we're terrible with names, so we're sticking with this.

## TODO

### CPU Emulation
Interpret and execute GB opcode instructions. Will need to implement a basic fetch-decode-execute cycle that can read and execute a simple set of instructions (e.g., NOP, LD, JP).

### Memory management
Emulate the GB's memory model, including ROM, RAM, and memory-mapped I/O. This will need a focus on ROM loading since we're gonna be loading `.gb` files, and basic read/write functions to memory addresses.

### Graphics rendering
Draw the screen contents by emulating the GB's pixel processing unit. The graphics system can be initially simple, just emulating the drawing of tiles and sprites to a frame buffer. May need to look into Rust graphics libraries such as `pixels` or `minifb` to display the contents of the frame buffer onto the screen.

### Input handling
Capture and process user input as GB button presses. Could use `winit` maybe to help with creating a window and handling input.

### Sound emulation
Eventually replicate the GB's sound synthesis. Look into using something like `cpal` to implement sound.
