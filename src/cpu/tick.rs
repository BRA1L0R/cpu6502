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

macro_rules! set_flag {
    ($self:ident, $flag:tt) => {
        $self.processor_status.set_flag(StatusFlag::$flag, true)
    };
}

macro_rules! assign_flag {
    ($self:ident.$var:ident $assign:tt $val:expr) => {{
        $self.$var $assign $val;
        $self.flag_value($self.$var);
    }};
}

impl Cpu {
    pub fn tick(&mut self) {
        let instruction = self.read_instruction();

        println!(
            "Instr: {:?} --- Addr: {:02X?}",
            instruction.instruction_type, instruction.addressing
        );
        println!("{}", self);

        let addr = instruction.addressing;
        match instruction.instruction_type {
            InstructionType::ADC => {
                let mem = self.load_addressing(addr);
                assign_flag!(self.accumulator = self.add_with_carry(mem, self.accumulator));
            }
            InstructionType::SBC => {
                let mem = self.load_addressing(addr);
                assign_flag!(self.accumulator = self.add_with_carry(self.accumulator, 255 - mem));
            }
            InstructionType::AND => assign_flag!(self.accumulator &= self.load_addressing(addr)),
            InstructionType::ASL => {
                let res = {
                    let val = self.mut_addressing(addr);
                    let carry = (*val & 0b1000_0000) != 0;

                    *val <<= 1;

                    (*val, carry)
                };

                self.processor_status.set_flag(StatusFlag::Carry, res.1);
                self.flag_value(res.0);
            }

            InstructionType::BIT => {
                let mem = self.load_addressing(addr);

                self.flag_value(mem);
                self.processor_status
                    .set_flag(StatusFlag::Overflow, (mem & 0b0100_0000) != 0);
            }
            InstructionType::BCC => flag_branch!(self, addr, !Carry),
            InstructionType::BCS => flag_branch!(self, addr, Carry),
            InstructionType::BEQ => flag_branch!(self, addr, Zero),
            InstructionType::BNE => flag_branch!(self, addr, !Zero),
            InstructionType::BMI => flag_branch!(self, addr, Negative),
            InstructionType::BPL => flag_branch!(self, addr, !Negative),
            InstructionType::BVC => flag_branch!(self, addr, !Overflow),
            InstructionType::BVS => flag_branch!(self, addr, Overflow),

            InstructionType::BRK => todo!(),

            // InstructionType::CLC => clear_flag!(self, Carry),
            InstructionType::CLC => clear_flag!(self, Carry),
            InstructionType::CLD => clear_flag!(self, Decimal),
            InstructionType::CLI => clear_flag!(self, Interrupt),
            InstructionType::CLV => clear_flag!(self, Overflow),

            InstructionType::CMP => self.cmp(self.accumulator, addr),
            InstructionType::CPX => self.cmp(self.x_register, addr),
            InstructionType::CPY => self.cmp(self.y_register, addr),

            InstructionType::DEC => {
                let addr = self.address_addressing(addr);
                let val = self.memory.get(addr) - 1;

                self.memory.set(addr, val);
                self.flag_value(val);
            }
            InstructionType::DEX => assign_flag!(self.x_register -= 1),
            InstructionType::DEY => assign_flag!(self.y_register -= 1),

            InstructionType::EOR => assign_flag!(self.accumulator ^= self.load_addressing(addr)),

            InstructionType::INC => {
                let addr = self.address_addressing(addr);
                let val = self.memory.get(addr) + 1;

                self.memory.set(addr, val);
                self.flag_value(val);
            }
            InstructionType::INX => assign_flag!(self.x_register += 1),
            InstructionType::INY => assign_flag!(self.y_register += 1),

            InstructionType::JMP => self.program_counter = self.address_addressing(addr),
            InstructionType::JSR => {
                self.stack_push_word(self.program_counter + 2);
                self.program_counter = self.address_addressing(addr);
            }

            InstructionType::LDA => assign_flag!(self.accumulator = self.load_addressing(addr)),
            InstructionType::LDX => assign_flag!(self.x_register = self.load_addressing(addr)),
            InstructionType::LDY => assign_flag!(self.y_register = self.load_addressing(addr)),

            InstructionType::LSR => {
                let res = {
                    let val = self.mut_addressing(addr);
                    let carry = (*val & 0b0000_0001) != 0;

                    *val >>= 1;

                    (*val, carry)
                };

                self.processor_status.set_flag(StatusFlag::Carry, res.1);
                self.flag_value(res.0);
            }

            InstructionType::STA => *self.mut_addressing(addr) = self.accumulator,
            InstructionType::NOP => (), // nop does nop-thing
            InstructionType::ORA => assign_flag!(self.accumulator |= self.load_addressing(addr)),

            InstructionType::PHA => self.stack_push(self.accumulator),
            InstructionType::PHP => self.stack_push(self.processor_status.0),
            InstructionType::PLA => assign_flag!(self.accumulator = self.stack_pop()),
            InstructionType::PLP => self.processor_status.0 = self.stack_pop(),

            InstructionType::ROL => {
                let res = {
                    let carry_in = self.processor_status.get_flag(StatusFlag::Carry);
                    let mem = self.mut_addressing(addr);
                    let carry_out = (*mem & 0b1000_0000) != 0;

                    *mem <<= 1;

                    if carry_in {
                        *mem |= 0b0000_0001;
                    } else {
                        *mem &= 0b1111_1110;
                    }

                    (*mem, carry_out)
                };

                self.processor_status.set_flag(StatusFlag::Carry, res.1);
                self.flag_value(res.0);
            }
            InstructionType::ROR => {
                let res = {
                    let carry_in = self.processor_status.get_flag(StatusFlag::Carry);
                    let mem = self.mut_addressing(addr);
                    let carry_out = (*mem & 0b0000_0001) != 0;

                    *mem <<= 1;

                    if carry_in {
                        *mem |= 0b1000_0000;
                    } else {
                        *mem &= 0b0111_1111;
                    }

                    (*mem, carry_out)
                };

                self.processor_status.set_flag(StatusFlag::Carry, res.1);
                self.flag_value(res.0);
            }

            InstructionType::RTI => todo!(),
            InstructionType::RTS => self.program_counter = self.stack_pop_word() + 1,

            InstructionType::SEC => set_flag!(self, Carry),
            InstructionType::SED => set_flag!(self, Decimal),
            InstructionType::SEI => set_flag!(self, Interrupt),

            InstructionType::STX => *self.mut_addressing(addr) = self.x_register,
            InstructionType::STY => *self.mut_addressing(addr) = self.y_register,

            InstructionType::TAX => assign_flag!(self.x_register = self.accumulator),
            InstructionType::TAY => assign_flag!(self.y_register = self.accumulator),
            InstructionType::TSX => assign_flag!(self.x_register = self.stack_pointer),
            InstructionType::TXA => assign_flag!(self.accumulator = self.x_register),
            InstructionType::TXS => assign_flag!(self.stack_pointer = self.x_register),
            InstructionType::TYA => assign_flag!(self.accumulator = self.y_register),
        }
    }
}

// enum RotationDirection

// fn rotate(mem: &mut u8) {}
//
