const STACK_OFFSET: u16 = 0x100;

use self::{
    instruction::{Addressing, Instruction},
    memory::Memory,
    status::ProcessorStatus,
};
use crate::cpu::instruction::InstructionType;
use std::fmt::Display;

mod instruction;
pub mod memory;
mod status;

pub struct Cpu {
    pub memory: Memory,

    program_counter: u16,

    accumulator: u8,
    x_register: u8,
    y_register: u8,

    stack_pointer: u8,

    processor_status: ProcessorStatus,
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PC: 0x{:X?} SP: 0x{:X?} --- ",
            self.program_counter, self.stack_pointer
        )?;
        writeln!(
            f,
            "A: 0x{:X?} X: 0x{:X?} Y: 0x{:X?} ",
            self.accumulator, self.x_register, self.y_register
        )?;

        Ok(())
    }
}

impl Cpu {
    pub fn load_memory(memory: Memory) -> Cpu {
        let rst_vector = memory.get_word(0xFFFC);

        Cpu {
            memory,

            program_counter: rst_vector,
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            stack_pointer: 0,
            processor_status: Default::default(),
        }
    }

    fn stack_push(&mut self, x: u8) {
        self.memory.set(STACK_OFFSET + self.stack_pointer as u16, x);
        self.stack_pointer -= 1;
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer += 1;
        self.memory.get(STACK_OFFSET + (self.stack_pointer) as u16)
    }

    fn read_byte(&mut self) -> u8 {
        self.program_counter += 1;
        self.memory.get(self.program_counter - 1)
    }

    fn read_instruction(&mut self) -> Instruction {
        let opcode = self.read_byte();

        Instruction::read_instruction(opcode, || self.read_byte())
    }

    fn address_addressing(&self, addressing: Addressing) -> u16 {
        match addressing {
            Addressing::Relative(offset) => self.program_counter.wrapping_add(offset as u16),
            Addressing::Zeropage(addr) => addr as u16, // TODO: these operations are all with carry
            Addressing::ZeropageX(addr) => (addr + self.x_register) as u16,
            Addressing::ZeropageY(addr) => (addr + self.y_register) as u16,
            Addressing::Absolute(addr) => addr,
            Addressing::AbsoluteX(addr) => addr + self.x_register as u16,
            Addressing::AbsoluteY(addr) => addr + self.y_register as u16,
            Addressing::Indirect(addr) => self.memory.get_word(addr),
            Addressing::IndirectX(addr) => self.memory.get_word((addr + self.x_register) as u16),
            Addressing::IndirectY(addr) => {
                self.memory.get_word(addr as u16) + self.y_register as u16
            }
            _ => 0,
        }
    }

    fn load_addressing(&self, addressing: Addressing) -> u8 {
        match addressing {
            Addressing::Immediate(x) => x,
            Addressing::Accumulator => self.accumulator,
            addr => self.memory.get(self.address_addressing(addr)),
        }
    }

    pub fn tick(&mut self) {
        let instruction = self.read_instruction();

        match instruction.instruction_type {
            InstructionType::STA => {
                let addr = self.address_addressing(instruction.addressing);
                self.memory.set(addr, self.accumulator)
            }
            InstructionType::LDA => {
                let mem = self.load_addressing(instruction.addressing);
                self.accumulator = mem;
            }
            InstructionType::PHA => self.stack_push(self.accumulator),
            InstructionType::PLA => self.accumulator = self.stack_pop(),
            InstructionType::JMP => {
                self.program_counter = self.address_addressing(instruction.addressing)
            }
            inst => panic!("instruction [{:?}] not yet implemented", inst),
        }

        println!("{}", self);
    }
}
