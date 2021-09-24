pub struct Memory {
    memory: [u8; 65535],
}

impl Memory {
    pub fn new() -> Self {
        Self { memory: [0; 65535] }
    }

    pub fn load_program(&mut self, offset: u16, program: &[u8]) {
        self.memory
            .iter_mut()
            .skip(offset as usize)
            .zip(program)
            .for_each(|(mem, prog)| *mem = *prog);
    }

    pub fn get_word(&self, offset: u16) -> u16 {
        let ll = self.get(offset);
        let hh = self.get(offset + 1);

        ((hh as u16) << 8) + ll as u16
    }

    pub fn get(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn ref_mut(&mut self, addr: u16) -> &mut u8 {
        &mut self.memory[addr as usize]
    }

    pub fn set_word(&mut self, offset: u16, x: u16) {
        self.set(offset, x as u8);
        self.set(offset + 1, (x >> 8) as u8);
    }

    pub fn set(&mut self, addr: u16, x: u8) {
        self.memory[addr as usize] = x
    }
}
