# Yet Another CHIP-8 Virtual Machine and Interpreter.

CHIP-8 is both a simple interpreted programming language and a specification for
a virtual machine. It's also a good place to start experimenting with
emulation.

## Dependencies

This implementation uses SDL2 for rendering and input. Both Rust and SDL2 are
cross-platform so the emulator should be as well. I won't cover SDL2 installation
here but you will need it available when you build the executable.

## Building and Running

Since this is a Rust project, we proceed with `cargo`. That is, in the root
directory run `cargo build --release`. This will give you an executable in
`target/release` named `yac8`. Run it with a program file as the argument to
fire up the machine. You can also combine the build and run process using
`cargo run` if you feel like it.

`yac8` supports variable clock frequencies (defaults to 700hz), verbose logging
of instructions to the terminal, and a scan mode that parses and prints a
program's instructions without executing. The  executable has `--help`, so check
it out. The variable clock frequency is useful because Chip-8 doesn't actually
specify a clock speed for instruction execution, only timer countdown
rates (60hz). Programs work best with a variety of clock speeds.

Note that `yac8` doesn't yet support sound.  Programs are rendered to 10x the
original resolution of 64 by 32 and this is not currently configurable. Controls
are mapped as below and are also not currently configurable.

    your keyboard    Chip-8 hexpad input
      1 2 3 4      |      1 2 3 C
      Q W E R      |      4 5 6 D
      A S D F      |      7 8 9 E
      Z X C V      |      A 0 B F


## TODO:

* Add some visual examples
* Implement audio facilities
* Configurable keymapping
* A user interface and stepping mode for debugging.

  Lay out the instructions and opcodes as a navigable interface, dump the
  registers in a corner, and allow stepping through arbitrary numbers of
  instructions. I think I'd like this to be a terminal user interface, using
  something like tui-rs.
