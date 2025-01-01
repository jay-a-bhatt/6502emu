// On Video #1, timestamp 13:30
//  https://youtu.be/qJgsuQoy9bc?si=VPn5THsHB6xoRgiL

type Byte = u8;    // equivalent to unsigned char
type Word = u16;   // equivalent to unsigned short

const MAX_MEM: usize = 1024 * 64;

struct Mem {
    data: [Byte; MAX_MEM]
}

impl Mem {
    pub fn new() -> Self {
        // Array is zero-initialized in constructor
        Mem {
            data: [0; MAX_MEM]
        }
    }

    pub fn initialize() -> Self {
        Mem {
            data: [0; MAX_MEM]
        }
    }
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
            sp: 0,
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

    // Reset the CPU
    pub fn reset(&mut self, mut mem: Mem) {
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

        Mem::initialize();
    }
}

fn main() {
    let mut mem: Mem = Mem::new();
    let mut cpu: CPU = CPU::new();
    CPU::reset(&mut cpu, mem);
    println!("Hello, world!");
}