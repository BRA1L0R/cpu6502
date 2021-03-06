const VECTOR_NMI: u16 = 0xFFFA;
const VECTOR_RESET: u16 = 0xFFFC;
const VECTOR_IRQ: u16 = 0xFFFE;

use self::{
    addressable_bus::DataBus,
    error::CpuError,
    instruction::{Addressing, Instruction},
    status::{ProcessorStatus, StatusFlag},
};
use std::fmt::Display;

pub mod addressable_bus;
pub mod addressing;
pub mod error;
pub mod instruction;
pub mod memops;
pub mod shifting;
pub mod status;
pub mod tick;

pub struct Cpu<T: DataBus> {
    pub bus: T,

    pub program_counter: u16,

    pub accumulator: u8,
    pub x_register: u8,
    pub y_register: u8,

    pub stack_pointer: u8,

    pub processor_status: ProcessorStatus,
}

impl<T: DataBus> Display for Cpu<T> {
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

impl<T: DataBus> Cpu<T> {
    pub fn load_memory(memory: T) -> Cpu<T> {
        let rst_vector = memory.get_word(VECTOR_RESET);

        Cpu {
            bus: memory,

            program_counter: rst_vector,
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            stack_pointer: 0,
            processor_status: Default::default(),
        }
    }

    fn read_instruction(&mut self) -> Result<Instruction, CpuError> {
        let opcode = self.read_byte();

        Instruction::read_instruction(opcode, || self.read_byte())
            .ok_or(CpuError::UnknownOpcode(opcode))
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
        let vector = self.bus.get_word(vector);

        self.stack_push_word(self.program_counter);
        self.stack_push(self.processor_status.0);

        self.processor_status.set_flag(StatusFlag::Interrupt, true);
        self.program_counter = vector;
    }

    fn pull_status(&mut self) {
        let mut ps: ProcessorStatus = self.stack_pop().into();

        ps.set_flag(
            StatusFlag::Ignored,
            self.processor_status.get_flag(StatusFlag::Ignored),
        );
        ps.set_flag(
            StatusFlag::Break,
            self.processor_status.get_flag(StatusFlag::Break),
        );

        self.processor_status = ps;
    }
}
