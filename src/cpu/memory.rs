use std::cell::RefCell;

pub struct Memory {
    memory: RefCell<[u8; 65535]>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            memory: RefCell::new([0; 65535]),
        }
    }

    pub fn load_program(&mut self, offset: u16, program: &[u8]) {
        self.memory
            .borrow_mut()
            .iter_mut()
            .skip(offset as usize)
            .zip(program)
            .for_each(|(mem, prog)| *mem = *prog);
    }

    pub fn get_word(&self, offset: u16) -> u16 {
        let hh = self.get(offset);
        let ll = self.get(offset + 1);

        ((hh as u16) << 8) + ll as u16
    }

    pub fn get(&self, addr: u16) -> u8 {
        self.memory.borrow()[addr as usize]
    }

    pub fn set_word(&mut self, offset: u16, x: u16) {
        self.set(offset, (x >> 8) as u8);
        self.set(offset + 1, x as u8);
    }

    pub fn set(&mut self, addr: u16, x: u8) {
        self.memory.borrow_mut()[addr as usize] = x
    }
}
