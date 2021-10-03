use clap::{App, Arg};
use cpu::{memory::Memory, Cpu};


pub mod cpu;


pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("CPU6502")
        .author("Pietro T. - BRA1L0R")
        // .about("6502 emulator")
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Waits for carriage return in stdin to perform a clock cycle and prints cpu information at each"),
        ).arg(
            Arg::with_name("offset")
                .short("o")
                .long("offset")
                .default_value("8000")
                .help("Specifies the load offset into memory for the given binary")
        )
        .arg(Arg::with_name("binary").required(true).help("Specifies the program to run")).get_matches();

    let program = std::fs::read(matches.value_of("binary").unwrap())?;

    let offset = matches.value_of("offset").unwrap().trim_start_matches("0x");
    let offset = u16::from_str_radix(offset, 16)?;

    let mut memory = Memory::new();
    memory.load_program(offset, &program);

    let mut cpu = Cpu::load_memory(memory);
    let debug = matches.is_present("debug");

    loop {
        let instr = cpu.tick()?;

        if debug {
            println!("Executing: {:?}", instr);
            println!("{}", cpu);
            std::io::stdin().read_line(&mut String::new())?;
        }
    }
}
