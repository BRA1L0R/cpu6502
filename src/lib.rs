use std::ops::Add;

const STACK_OFFSET: u16 = 0x100;

enum Addressing {
    Accumulator,
    Implied,

    Relative(u8),

    Immediate(u8),

    Zeropage(u8),
    ZeropageX(u8),

    Absolute(u16),
    AbsoluteX(u16),
    AbsoluteY(u16),

    Indirect(u16),
    IndirectX(u8),
    IndirectY(u8),
}

enum InstructionType {
    ADC, //     add with carry
    AND, // and (with accumulator)
    ASL, // arithmetic shift left
    BCC, // branch on carry clear
    BCS, // branch on carry set
    BEQ, // branch on equal (zero set)
    BIT, // bit test
    BMI, // branch on minus (negative set)
    BNE, // branch on not equal (zero clear)
    BPL, // branch on plus (negative clear)
    BRK, // break / interrupt
    BVC, // branch on overflow clear
    BVS, // branch on overflow set
    CLC, // clear carry
    CLD, // clear decimal
    CLI, // clear interrupt disable
    CLV, // clear overflow
    CMP, // compare (with accumulator)
    CPX, // compare with X
    CPY, // compare with Y
    DEC, // decrement
    DEX, // decrement X
    DEY, // decrement Y
    EOR, // exclusive or (with accumulator)
    INC, // increment
    INX, // increment X
    INY, // increment Y
    JMP, // jump
    JSR, // jump subroutine
    LDA, // load accumulator
    LDX, // load X
    LDY, // load Y
    LSR, // logical shift right
    NOP, // no operation
    ORA, // or with accumulator
    PHA, // push accumulator
    PHP, // push processor status (SR)
    PLA, // pull accumulator
    PLP, // pull processor status (SR)
    ROL, // rotate left
    ROR, // rotate right
    RTI, // return from interrupt
    RTS, // return from subroutine
    SBC, // subtract with carry
    SEC, // set carry
    SED, // set decimal
    SEI, // set interrupt disable
    STA, // store accumulator
    STX, // store X
    STY, // store Y
    TAX, // transfer accumulator to X
    TAY, // transfer accumulator to Y
    TSX, // transfer stack pointer to X
    TXA, // transfer X to accumulator
    TXS, // transfer X to stack pointer
    TYA, // transfer Y to accumulator
}

struct Instruction {
    instruction_type: InstructionType,
    addressing: Addressing,
}

// macro_rules! opcode_match {
//     ($opcode:expr, $type:tt, $addressing:tt) => {
//         $opcode => (InstructionType::$type, Addressing::$addressing(0))
//     };
// }

macro_rules! opcode_match {
    ($opcode: expr $(, $type:tt => {$(($matchcode:expr, $addressing:tt),)*} )*) => {
        match $opcode {
            $(
                $(
                    $matchcode => (InstructionType::$type, Addressing::$addressing(0)),
                )*
            )*
            _ => todo!(),
        }
    };
}

impl Instruction {
    fn read_instruction(opcode: u8, mut read_extra: impl FnMut() -> u8) -> Instruction {
        let mut instr = opcode_match! {
            opcode,

            ADC => {
                (0x69, Immediate),
            }
        }; // turi gay

        match &mut instr.1 {
            Addressing::Relative(x)
            | Addressing::Immediate(x)
            | Addressing::Zeropage(x)
            | Addressing::ZeropageX(x)
            | Addressing::IndirectX(x)
            | Addressing::IndirectY(x) => *x = read_extra(),

            Addressing::Indirect(x)
            | Addressing::Absolute(x)
            | Addressing::AbsoluteX(x)
            | Addressing::AbsoluteY(x) => *x = ((read_extra() as u16) << 8) + read_extra() as u16,

            _ => (),
        }

        Instruction {
            instruction_type: instr.0,
            addressing: instr.1,
        }
    }
}

struct Memory {
    memory: [u8; 65535],
}

impl Memory {
    fn load(program: &[u8]) -> Memory {
        let mut memory = [0u8; 65535];

        memory
            .iter_mut()
            .zip(program.iter())
            .for_each(|(mem, prog)| *mem = *prog);

        Memory { memory }
    }

    fn get(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn set(&mut self, addr: u16, x: u8) {
        self.memory[addr as usize] = x
    }
}

struct Cpu {
    memory: Memory,

    program_counter: u16,

    accumulator: u8,
    x_register: u8,
    y_register: u8,

    stack_pointer: u8,

    processor_status: u8,
}

impl Cpu {
    fn stack_push(&mut self, x: u8) {
        self.stack_pointer -= 1;
        self.memory.set(STACK_OFFSET + self.stack_pointer as u16, x)
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer += 1;

        self.memory
            .get(STACK_OFFSET + (self.stack_pointer - 1) as u16)
    }

    fn load(program: &[u8]) -> Cpu {
        let memory = Memory::load(program);

        Cpu {
            memory,

            program_counter: 0,
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            stack_pointer: 0,
            processor_status: 0,
        }
    }

    fn read_byte(&mut self) -> u8 {
        self.program_counter += 1;
        self.memory.get(self.program_counter - 1)
    }

    fn read_instruction(&mut self) -> Instruction {
        let opcode = self.read_byte();

        Instruction::read_instruction(opcode, || self.read_byte())
    }

    fn run(&mut self) -> ! {
        loop {
            let instruction = self.read_instruction();

            // match instruction.instruction_type {
            //     InstructionType::STA => match instruction.addressing {
            //         Addressing::Zeropage(addr) => self.memory.set(addr as u16, self.accumulator),
            //         _ => todo!(),
            //     },
            //     InstructionType::LDA => match instruction.addressing {
            //         Addressing::Immediate(x) => self.accumulator = x,
            //         _ => todo!(),
            //     },
            // }

            println!("{}", self.memory.get(0xFF))
        }
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let program = std::fs::read(args.next().expect("program file path"))?;

    let mut cpu = Cpu::load(&program);
    cpu.run();
}
