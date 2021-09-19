use super::Cpu;
use crate::cpu::{
    instruction::{Addressing, InstructionType},
    status::StatusFlag,
};

macro_rules! flag_branch {
    ($self:ident, $addr:ident, $status:tt) => {
        if $self.processor_status.get_flag(StatusFlag::$status) {
            $self.program_counter = $self.address_addressing($addr);
        }
    };
    ($self:ident, $addr:ident, !$status:tt) => {
        if !$self.processor_status.get_flag(StatusFlag::$status) {
            $self.program_counter = $self.address_addressing($addr);
        };
    };
}

macro_rules! clear_flag {
    ($self:ident, $flag:tt) => {
        $self.processor_status.set_flag(StatusFlag::$flag, false)
    };
}

impl Cpu {
    pub fn tick(&mut self) {
        let instruction = self.read_instruction();

        let addr = instruction.addressing;
        match instruction.instruction_type {
            InstructionType::ADC => {
                let (result, carry) = self.load_addressing(addr).overflowing_add(
                    self.accumulator + self.processor_status.get_flag(StatusFlag::Carry) as u8,
                );

                self.accumulator = result;
                self.processor_status.set_flag(StatusFlag::Carry, carry);
            }
            InstructionType::AND => self.accumulator &= self.load_addressing(addr),
            InstructionType::ASL => match addr {
                Addressing::Accumulator => self.accumulator <<= 1,
                _ => {
                    let addr = self.address_addressing(addr);
                    self.memory.set(addr, self.memory.get(addr) << 1);
                }
            },
            InstructionType::BCC => flag_branch!(self, addr, !Carry),
            InstructionType::BCS => flag_branch!(self, addr, Carry),
            InstructionType::BEQ => flag_branch!(self, addr, Zero),
            InstructionType::BNE => flag_branch!(self, addr, !Zero),
            InstructionType::BIT => {
                let mem = self.load_addressing(addr);
                self.processor_status
                    .set_flag(StatusFlag::Negative, (mem & 0b1000_0000) != 0);
                self.processor_status
                    .set_flag(StatusFlag::Overflow, (mem & 0b0100_0000) != 0);
            }
            InstructionType::BMI => flag_branch!(self, addr, Negative),
            InstructionType::BPL => flag_branch!(self, addr, !Negative),
            InstructionType::BRK => todo!(),
            InstructionType::BVC => flag_branch!(self, addr, !Overflow),
            InstructionType::BVS => flag_branch!(self, addr, Overflow),
            InstructionType::CLC => clear_flag!(self, Carry),
            InstructionType::CLD => clear_flag!(self, Decimal),
            InstructionType::CLI => clear_flag!(self, Interrupt),
            InstructionType::CLV => clear_flag!(self, Overflow),
            InstructionType::STA => {
                let addr = self.address_addressing(addr);
                self.memory.set(addr, self.accumulator);
            }
            InstructionType::LDA => {
                let mem = self.load_addressing(addr);
                self.accumulator = mem;
            }
            InstructionType::PHA => self.stack_push(self.accumulator),
            InstructionType::PLA => self.accumulator = self.stack_pop(),
            InstructionType::JMP => self.program_counter = self.address_addressing(addr),
            inst => panic!("instruction [{:?}] not yet implemented", inst),
        }

        println!("{}", self);
    }
}
