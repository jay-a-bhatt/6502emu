pub mod status {
    use crate::Byte;
    pub const NEGATIVE: Byte = 0b10000000;
	pub const OVERFLOW: Byte = 0b01000000;
	pub const BREAK: Byte = 0b000010000;
	pub const UNUSED: Byte = 0b000100000;
	pub const INTERRUPT_DISABLE: Byte = 0b000000100;
	pub const ZERO: Byte = 0b00000001;
}