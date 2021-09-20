// use super::Instruction;
use super::{Addressing, Instruction, InstructionType};

macro_rules! opcode_match {
    ($opcode: expr
        $(, $type:tt => {
            $(($matchcode:expr, $addressing:tt)),*
            $(; $(($optcode:expr, $optaddr:tt)),*)? })*
        $(; $($impltype:tt => $implcode:expr),*)?) =>
    {
        match $opcode {
            $(
                $(
                    $matchcode => (InstructionType::$type, Addressing::$addressing(0)),

                )*
                $(
                    $(
                        $optcode => (InstructionType::$type, Addressing::$optaddr),
                    )*
                )?
            )*
            $(
                $(
                    $implcode => (InstructionType::$impltype, Addressing::Implied),
                )*
            )?

            _ => panic!("OpCode not yet mapped [{:X}]", $opcode),
        }
    };

}

impl Instruction {
    pub fn read_instruction(opcode: u8, mut read_byte: impl FnMut() -> u8) -> Instruction {
        let mut instr = opcode_match! {
            opcode,

            ADC => {
                (0x69, Immediate),
                (0x65, Zeropage),
                (0x75, ZeropageX),
                (0x6D, Absolute),
                (0x7D, AbsoluteX),
                (0x79, AbsoluteY),
                (0x61, IndirectX),
                (0x71, IndirectY)
            },
            AND => {
                (0x29, Immediate),
                (0x25, Zeropage),
                (0x35, ZeropageX),
                (0x2D, Absolute),
                (0x3D, AbsoluteX),
                (0x39, AbsoluteY),
                (0x21, IndirectX),
                (0x31, IndirectY)
            },
            ASL => {
                (0x06, Zeropage),
                (0x16, ZeropageX),
                (0x0E, Absolute),
                (0x1E, AbsoluteX);
                (0x0A, Accumulator)
            },
            BCC => {
                (0x90, Relative)
            },
            BCS => {
                (0xB0, Relative)
            },
            BEQ => {
                (0xF0, Relative)
            },
            BIT => {
                (0x24, Zeropage),
                (0x2C, Absolute)
            },
            BMI => {
                (0x30, Relative)
            },
            BNE => {
                (0xD0, Relative)
            },
            BPL => {
                (0x10, Relative)
            },
            BVC => {
                (0x50, Relative)
            },
            BVS => {
                (0x70, Relative)
            },
            CMP => {
                (0xC9, Immediate),
                (0xc5, Zeropage),
                (0xd5, ZeropageX),
                (0xcd, Absolute),
                (0xdd, AbsoluteX),
                (0xd9, AbsoluteY),
                (0xc1, IndirectX),
                (0xd1, IndirectY)
            },
            CPX => {
                (0xe0, Immediate),
                (0xe4, Zeropage),
                (0xec, Absolute)
            },
            CPY => {
                (0xc0, Immediate),
                (0xc4, Zeropage),
                (0xcc, Absolute)
            },
            DEC => {
                (0xc6, Zeropage),
                (0xd6, ZeropageX),
                (0xce, Absolute),
                (0xde, AbsoluteX)
            },
            EOR => {
                (0x49, Immediate),
                (0x45, Zeropage),
                (0x55, ZeropageX),
                (0x4d, Absolute),
                (0x5d, AbsoluteX),
                (0x59, AbsoluteY),
                (0x41, IndirectX),
                (0x51, IndirectY)
            },
            INC => {
                (0xe6, Zeropage),
                (0xf6, ZeropageX),
                (0xee, Absolute),
                (0xfe, AbsoluteX)
            },
            JMP => {
                (0x4C, Absolute),
                (0x6C, Indirect)
            },
            JSR => {
                (0x20, Absolute)
            },
            LDA => {
                (0xa9, Immediate),
                (0xa5, Zeropage),
                (0xb5, ZeropageX),
                (0xad, Absolute),
                (0xbd, AbsoluteX),
                (0xb9, AbsoluteY),
                (0xa1, IndirectX),
                (0xb1, IndirectY)
            },
            LDX => {
                (0xa2, Immediate),
                (0xa6, Zeropage),
                (0xb6, ZeropageY),
                (0xae, Absolute),
                (0xbe, AbsoluteY)
            },
            LDY => {
                (0xa0, Immediate),
                (0xa4, Zeropage),
                (0xb4, ZeropageX),
                (0xac, Absolute),
                (0xbc, AbsoluteX)
            },
            LSR => {
                (0x46, Zeropage),
                (0x56, ZeropageX),
                (0x4E, Absolute),
                (0x5e, AbsoluteX);
                (0x4a, Accumulator)
            },
            ORA => {
                (0x09, Immediate),
                (0x05, Zeropage),
                (0x15, ZeropageX),
                (0x0d, Absolute),
                (0x1d, AbsoluteX),
                (0x19, AbsoluteY),
                (0x01, IndirectX),
                (0x11, IndirectY)
            },
            ROL => {
                (0x26, Zeropage),
                (0x36, ZeropageX),
                (0x2e, Absolute),
                (0x3e, AbsoluteX);
                (0x2a, Accumulator)
            },
            ROR => {
                (0x66, Zeropage),
                (0x76, ZeropageX),
                (0x6e, Absolute),
                (0x7e, AbsoluteX);
                (0x6a, Accumulator)
            },
            SBC => {
                (0xe9, Immediate),
                (0xe5, Zeropage),
                (0xf5, ZeropageX),
                (0xed, Absolute),
                (0xfd, AbsoluteX),
                (0xf9, AbsoluteY),
                (0xe1, IndirectX),
                (0xf1, IndirectY)
            },
            STA => {
                (0x85, Zeropage),
                (0x95, ZeropageX),
                (0x8d, Absolute),
                (0x9d, AbsoluteX),
                (0x99, AbsoluteY),
                (0x81, IndirectX),
                (0x91, IndirectY)
            },
            STX => {
                (0x86, Zeropage),
                (0x96, ZeropageY),
                (0x8e, Absolute)
            },
            STY => {
                (0x84, Zeropage),
                (0x94, ZeropageX),
                (0x8c, Absolute)
            }
            ;

            NOP => 0xea,
            PLA => 0x68,
            BRK => 0x00,
            CLC => 0x18,
            CLD => 0xD8,
            CLI => 0x58,
            CLV => 0xB8,
            DEX => 0xCA,
            DEY => 0x88,
            INX => 0xE8,
            INY => 0xC8,
            PHA => 0x48,
            PHP => 0x08,
            PLP => 0x28,
            RTI => 0x40,
            RTS => 0x60,
            SEC => 0x38,
            SED => 0xF8,
            SEI => 0x78,
            TAX => 0xAA,
            TAY => 0xA8,
            TSX => 0xBA,
            TXA => 0x8A,
            TXS => 0x9A,
            TYA => 0x98
        };

        match &mut instr.1 {
            Addressing::Relative(x)
            | Addressing::Immediate(x)
            | Addressing::Zeropage(x)
            | Addressing::ZeropageX(x)
            | Addressing::IndirectX(x)
            | Addressing::IndirectY(x) => *x = read_byte(),

            Addressing::Indirect(x)
            | Addressing::Absolute(x)
            | Addressing::AbsoluteX(x)
            | Addressing::AbsoluteY(x) => {
                *x = (read_byte() as u16) + ((read_byte() as u16) << 8);
            }

            _ => (),
        }

        Instruction {
            instruction_type: instr.0,
            addressing: instr.1,
        }
    }
}
