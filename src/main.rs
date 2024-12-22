type Byte = u8;    // equivalent to unsigned char
type Word = u16;   // equivalent to unsigned short

struct Mem {

}

struct CPU {
    pc: Word, // Program counter
    sp: Word, // Stack pointer

    // Registers
    a: Byte,
    x: Byte,
    y: Byte,

    // Processor Status Flags
    c: Byte, // Carry flag
    z: Byte, // Zero flag
    i: Byte, // Interrupt disable
    d: Byte, // Decimal mode
    b: Byte, // Break command
    v: Byte, // Overflow flag
    n: Byte, // Negative flag
}

impl CPU {
    // Constructor with default values
    pub fn new() -> Self {
        CPU {
            pc: 0,
            sp: 0xFF, // Usually starts at top of stack
            a: 0,
            x: 0,
            y: 0,
            c: 0,
            z: 0,
            i: 0,
            d: 0,
            b: 0,
            v: 0,
            n: 0,
        }
    }

    // Program Counter methods
    pub fn get_pc(&self) -> Word {
        self.pc
    }

    pub fn set_pc(&mut self, value: Word) {
        self.pc = value;
    }

    pub fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }

    // Stack Pointer methods
    pub fn get_sp(&self) -> Word {
        self.sp
    }

    pub fn set_sp(&mut self, value: Word) {
        self.sp = value;
    }

    // Register methods
    pub fn get_a(&self) -> Byte {
        self.a
    }

    pub fn set_a(&mut self, value: Byte) {
        self.a = value;
        self.update_zero_and_negative_flags(value);
    }

    pub fn get_x(&self) -> Byte {
        self.x
    }

    pub fn set_x(&mut self, value: Byte) {
        self.x = value;
        self.update_zero_and_negative_flags(value);
    }

    pub fn get_y(&self) -> Byte {
        self.y
    }

    pub fn set_y(&mut self, value: Byte) {
        self.y = value;
        self.update_zero_and_negative_flags(value);
    }

    // Flag methods
    pub fn get_carry(&self) -> bool {
        self.c != 0
    }

    pub fn set_carry(&mut self, value: bool) {
        self.c = if value { 1 } else { 0 };
    }

    pub fn get_zero(&self) -> bool {
        self.z != 0
    }

    pub fn set_zero(&mut self, value: bool) {
        self.z = if value { 1 } else { 0 };
    }

    pub fn get_interrupt(&self) -> bool {
        self.i != 0
    }

    pub fn set_interrupt(&mut self, value: bool) {
        self.i = if value { 1 } else { 0 };
    }

    pub fn get_decimal(&self) -> bool {
        self.d != 0
    }

    pub fn set_decimal(&mut self, value: bool) {
        self.d = if value { 1 } else { 0 };
    }

    pub fn get_break(&self) -> bool {
        self.b != 0
    }

    pub fn set_break(&mut self, value: bool) {
        self.b = if value { 1 } else { 0 };
    }

    pub fn get_overflow(&self) -> bool {
        self.v != 0
    }

    pub fn set_overflow(&mut self, value: bool) {
        self.v = if value { 1 } else { 0 };
    }

    pub fn get_negative(&self) -> bool {
        self.n != 0
    }

    pub fn set_negative(&mut self, value: bool) {
        self.n = if value { 1 } else { 0 };
    }

    // Helper method to update Zero and Negative flags based on a value
    fn update_zero_and_negative_flags(&mut self, value: Byte) {
        self.set_zero(value == 0);
        self.set_negative((value & 0x80) != 0);
    }

    // Get status register as a single byte
    pub fn get_status(&self) -> Byte {
        (self.c << 0) |
        (self.z << 1) |
        (self.i << 2) |
        (self.d << 3) |
        (self.b << 4) |
        (1 << 5) |     // Bit 5 is always 1
        (self.v << 6) |
        (self.n << 7)
    }

    // Set status register from a single byte
    pub fn set_status(&mut self, status: Byte) {
        self.c = (status >> 0) & 1;
        self.z = (status >> 1) & 1;
        self.i = (status >> 2) & 1;
        self.d = (status >> 3) & 1;
        self.b = (status >> 4) & 1;
        self.v = (status >> 6) & 1;
        self.n = (status >> 7) & 1;
    }

    // Reset the CPU
    pub fn reset(&mut self) {
        // Set pc to default reset vector hex address
        self.pc = 0xFFFC;
        // Set sp to initialize at 100 (not accurate to Commodore64, still works)
        self.sp = 0x0100;
        // Clear all flags (not accurate, but still boots)
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.c = 0;
        self.z = 0;
        self.i = 0;
        self.d = 0;
        self.b = 0;
        self.v = 0;
        self.n = 0;
    }
}

fn main() {
    let mut cpu: CPU = CPU::new();
    println!("Hello, world!");
}