use super::Cpu;

const STACK_OFFSET: u16 = 0x100;

impl Cpu {
    pub fn stack_push(&mut self, x: u8) {
        self.memory.set(STACK_OFFSET + self.stack_pointer as u16, x);
        self.stack_pointer -= 1;
    }

    pub fn stack_push_word(&mut self, x: u16) {
        self.stack_pointer -= 2;
        self.memory
            .set_word(STACK_OFFSET + self.stack_pointer as u16 + 1, x);
    }

    pub fn stack_pop(&mut self) -> u8 {
        self.stack_pointer += 1;
        self.memory.get(STACK_OFFSET + (self.stack_pointer) as u16)
    }

    pub fn stack_pop_word(&mut self) -> u16 {
        self.stack_pointer += 2;
        self.memory
            .get_word(STACK_OFFSET + (self.stack_pointer) as u16 - 1)
    }

    pub fn read_byte(&mut self) -> u8 {
        self.program_counter += 1;
        self.memory.get(self.program_counter - 1)
    }
}
