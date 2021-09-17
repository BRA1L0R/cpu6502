use cpu::{memory::Memory, Cpu};

// use std::fmt::Display;

mod cpu;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let program = std::fs::read(args.next().expect("program file path"))?;
    let offset = args
        .next()
        .map_or(Ok(0), |off| u16::from_str_radix(&off, 16))?;

    let mut memory = Memory::new();
    memory.load_program(offset, &program);
    memory.set_word(0xFFFC, offset);

    let mut cpu = Cpu::load_memory(memory);
    loop {
        cpu.tick();
    }
}
