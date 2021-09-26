# CPU 6502

A non cycle-accurate emulator implementing all legal 6502 opcodes.

## What does `non cycle-accurate` mean?

Every instruction on the 6502 takes a number of clock cycles to complete a single instruction, depending on many factors including memory paging.

This emulator does have a concept of clock, but every instruction takes just one clock cycle to complete. Also, there are no initial clock cycles.

## What about interrupts?

As of now, this is no reliable emulator. I'm planning on working on a cycle-accurate 1:1 rapresentation of the 6502 based on this project, which will include interrupts and resets, **and will not include any reference to the standard library**.

However, BRK interrupts are still completely functional and work as intended.

## Sounds cool, how do I run it?

This project is written in Rust, so you will need `cargo` (or at least `rustc`) in order to compile it.

Using `cargo`:

```
cargo build --release
```

The output binary will be available here: `./target/release/cpu6502`

### Actually running it

Once you've built the binary you can run:

```
./target/release/cpu6502 program.o65
```

where `program.o65` is the binary file containing your program.

### A note on running programs

All programs are loaded by an offset of `0x8000` into memory. So you'll need to specify the reset vector to point to that memory location.

You can see an example of how this is done in any of the examples under the `examples/` folder

### Running an example

Examples are built following `xa` assembler guidelines, and use its pseudo-opcodes (or macros) for memory allignment.

Compiling an example with `xa` is really straight-forward:

```
xa example/fibonacci.s
```

Then run it with the emulator binary:

```
./target/release/cpu6502 a.o65
```

And the program should panic upon reaching the jamming opcode `0x22`, printing the processor status, with the result of the fibonacci sequence stored in the `A` register (the accumulator)
