use std::fmt::Display;

#[derive(Debug)]
pub enum CpuError {
    UnknownOpcode(u8),
}

impl Display for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownOpcode(opcode) => write!(f, "Unknown opcode 0x{:02X}", opcode),
        }
    }
}

impl std::error::Error for CpuError {}
