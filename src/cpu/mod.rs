const VECTOR_NMI: u16 = 0xFFFA;
const VECTOR_RESET: u16 = 0xFFFC;
const VECTOR_IRQ: u16 = 0xFFFE;

use self::{
    instruction::{Addressing, Instruction},
    memory::Memory,
    status::{ProcessorStatus, StatusFlag},
};
use std::fmt::Display;

mod addressing;
mod instruction;
mod memops;
pub mod memory;
mod status;
mod tick;

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
        write!(
            f,
            "A: 0x{:X?} X: 0x{:X?} Y: 0x{:X?} --- ",
            self.accumulator, self.x_register, self.y_register
        )?;
        writeln!(f, "ST: {:08b}", self.processor_status.0)?;

        Ok(())
    }
}

impl Cpu {
    pub fn load_memory(memory: Memory) -> Cpu {
        let rst_vector = memory.get_word(VECTOR_RESET);

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

    fn read_instruction(&mut self) -> Instruction {
        let opcode = self.read_byte();

        Instruction::read_instruction(opcode, || self.read_byte())
    }

    fn add_with_carry(&mut self, x: u8, y: u8) -> u8 {
        let carry = self.processor_status.get_flag(StatusFlag::Carry);

        let (res, part_carry) = x.overflowing_add(y);
        let (res, res_carry) = res.overflowing_add(carry as u8);

        let res_carry = part_carry | res_carry;

        self.processor_status.set_flag(StatusFlag::Carry, res_carry);
        self.processor_status
            .set_flag(StatusFlag::Overflow, carry ^ res_carry);

        res
    }

    fn flag_value(&mut self, val: u8) {
        self.processor_status
            .set_flag(StatusFlag::Negative, (val & 0b1000_0000) != 0);
        self.processor_status.set_flag(StatusFlag::Zero, val == 0);
    }

    fn cmp(&mut self, x: u8, y: Addressing) {
        let y = self.load_addressing(y);

        self.processor_status.set_flag(StatusFlag::Carry, x >= y);
        self.processor_status.set_flag(StatusFlag::Zero, x == y);
        self.processor_status
            .set_flag(StatusFlag::Negative, ((x - y) & 0b1000_0000) != 0);
    }

    fn interrupt(&mut self, vector: u16) {
        let vector = self.memory.get_word(vector);

        self.stack_push_word(self.program_counter);
        self.stack_push(self.processor_status.0);

        self.processor_status.set_flag(StatusFlag::Interrupt, true);
        self.program_counter = vector;
    }

    fn pull_status(&mut self) {
        let ps = self.stack_pop().into();

        self.processor_status.set_flag(
            StatusFlag::Ignored,
            self.processor_status.get_flag(StatusFlag::Ignored),
        );
        self.processor_status.set_flag(
            StatusFlag::Break,
            self.processor_status.get_flag(StatusFlag::Break),
        );

        self.processor_status = ps;
    }
}
