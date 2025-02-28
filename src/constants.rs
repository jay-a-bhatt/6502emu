use std::fmt;

pub type Byte = u8;
pub type Word = u16;

pub const MAX_MEM: usize = 1024 * 64;

#[derive(Debug, Copy, Clone)]
pub struct SByte(u8); // Equivalent to a char in C
impl fmt::Display for SByte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 >= 32 && self.0 <= 126 {
            write!(f, "{}", self.0 as char)
        } else {
            write!(f, "_")
        }
    }
}

impl From<u8> for SByte {
    fn from(value: u8) -> Self {
        SByte(value)
    }
}