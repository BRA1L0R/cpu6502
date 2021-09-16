use std::fmt::Display;

const STACK_OFFSET: u16 = 0x100;

#[derive(Debug)]
enum Addressing {
    Accumulator,
    Implied,

    Relative(u8),

    Immediate(u8),

    Zeropage(u8),
    ZeropageX(u8),
    ZeropageY(u8),

    Absolute(u16),
    AbsoluteX(u16),
    AbsoluteY(u16),

    Indirect(u16),
    IndirectX(u8),
    IndirectY(u8),
}

#[derive(Debug)]
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
    fn read_instruction(opcode: u8, mut read_byte: impl FnMut() -> u8) -> Instruction {
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

struct Memory {
    memory: [u8; 65535],
}

impl Memory {
    fn load_program(offset: u16, program: &[u8]) -> Memory {
        let mut memory = [0u8; 65535];

        memory
            .iter_mut()
            .skip(offset as usize)
            .zip(program)
            .for_each(|(mem, prog)| *mem = *prog);

        Memory { memory }
    }

    fn get_word(&self, offset: u16) -> u16 {
        let hh = self.get(offset);
        let ll = self.get(offset + 1);

        ((hh as u16) << 8) + ll as u16
    }

    fn get(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn set_word(&mut self, offset: u16, x: u16) {
        self.memory[offset as usize] = (x >> 8) as u8;
        self.memory[(offset + 1) as usize] = x as u8;
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
    fn stack_push(&mut self, x: u8) {
        self.memory.set(STACK_OFFSET + self.stack_pointer as u16, x);
        self.stack_pointer -= 1;
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer += 1;
        self.memory.get(STACK_OFFSET + (self.stack_pointer) as u16)
    }

    fn load_memory(memory: Memory) -> Cpu {
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

    fn run(&mut self) -> ! {
        let rst_vector = self.memory.get_word(0xFFFC);
        println!("reset vector {}", rst_vector);

        self.program_counter = rst_vector;

        loop {
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
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let program = std::fs::read(args.next().expect("program file path"))?;
    let offset = args
        .next()
        .map_or(Ok(0), |off| u16::from_str_radix(&off, 16))?;

    let mut memory = Memory::load_program(offset, &program);
    memory.set_word(0xFFFC, offset);

    let mut cpu = Cpu::load_memory(memory);
    cpu.run();
}
