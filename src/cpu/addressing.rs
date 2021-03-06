use super::{addressable_bus::DataBus, instruction::Addressing, Cpu};

impl<T: DataBus> Cpu<T> {
    pub fn address_addressing(&self, addressing: Addressing) -> u16 {
        match addressing {
            Addressing::Relative(offset) => {
                let offset = offset as i8;
                self.program_counter + (offset as u16)
            }
            Addressing::Zeropage(addr) => addr as u16, // TODO: these operations are all with carry
            Addressing::ZeropageX(addr) => (addr + self.x_register) as u16,
            Addressing::ZeropageY(addr) => (addr + self.y_register) as u16,
            Addressing::Absolute(addr) => addr,
            Addressing::AbsoluteX(addr) => addr + self.x_register as u16,
            Addressing::AbsoluteY(addr) => addr + self.y_register as u16,
            Addressing::Indirect(addr) => self.bus.get_word(addr),
            Addressing::IndirectX(addr) => self.bus.get_word((addr + self.x_register) as u16),
            Addressing::IndirectY(addr) => self.bus.get_word(addr as u16) + self.y_register as u16,
            _ => 0,
        }
    }

    pub fn load_addressing(&self, addressing: Addressing) -> u8 {
        match addressing {
            Addressing::Immediate(x) => x,
            Addressing::Accumulator => self.accumulator,
            addr => self.bus.get(self.address_addressing(addr)),
        }
    }

    pub fn write_addressing(&mut self, addressing: Addressing, x: u8) {
        match addressing {
            Addressing::Accumulator => self.accumulator = x,
            addr => self.bus.set(self.address_addressing(addr), x),
        }
    }
}
