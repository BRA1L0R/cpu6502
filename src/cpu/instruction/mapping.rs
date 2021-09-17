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

            _ => todo!(),
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
            JMP => {

                (0x4C, Absolute),
                (0x6C, Indirect)

            },
            STA => {
                (0x85, Zeropage),
                (0x95, ZeropageX),
                (0x8d, Absolute),
                (0x9d, AbsoluteX),
                (0x99, AbsoluteY),
                (0x81, IndirectX),
                (0x91, IndirectY)
            };

            NOP => 0x01,
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
            TXS => 0x9A
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
