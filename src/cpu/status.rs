pub enum StatusFlag {
    Negative = 0b1000_0000,
    Overflow = 0b0100_0000,
    Break = 0b0001_0000,
    Decimal = 0b0000_1000,
    Interrupt = 0b0000_0100,
    Zero = 0b0000_0010,
    Carry = 0b0000_0001,
}

#[derive(Default)]
pub struct ProcessorStatus(u8);

impl ProcessorStatus {
    pub fn set_flag(&mut self, flag: StatusFlag, set: bool) {
        let flag = flag as u8;

        if set {
            self.0 |= flag;
        } else {
            self.0 &= 0xFF - flag;
        }
    }

    pub fn get_flag(&self, flag: StatusFlag) -> bool {
        (self.0 & flag as u8) != 0
    }
}
