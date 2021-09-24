use cpu::{memory::Memory, Cpu};

// use std::fmt::Display;
const OFFSET: u16 = 0x8000;

mod cpu;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let program = std::fs::read(args.next().expect("program file path"))?;

    let mut memory = Memory::new();
    memory.load_program(OFFSET, &program);

    let mut cpu = Cpu::load_memory(memory);
    loop {
        cpu.tick();
    }
}
