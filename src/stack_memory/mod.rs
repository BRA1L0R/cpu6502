use crate::cpu::addressable_bus::DataBus;

// use cpu6502::cpu::addressable_bus::DataBus;

pub struct StackMemory {
    memory: [u8; 65535],
}

impl DataBus for StackMemory {
    fn get(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn set(&mut self, addr: u16, x: u8) {
        self.memory[addr as usize] = x
    }
}

impl Default for StackMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl StackMemory {
    pub fn new() -> Self {
        Self { memory: [0; 65535] }
    }

    pub fn load_data(&mut self, offset: u16, program: &[u8]) {
        self.memory
            .iter_mut()
            .skip(offset as usize)
            .zip(program)
            .for_each(|(mem, prog)| *mem = *prog);
    }
}
