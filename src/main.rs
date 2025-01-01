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

    pub fn initialize(&self) -> Self {
        Mem {
            data: [0; MAX_MEM]
        }
    }

    pub fn read(&self, address: u32) -> Option<Byte>{
        if address > MAX_MEM as u32 {
            return None
        } else {
            return Some(self.data[address as usize])
        }
    }

    pub fn write(&mut self, address: u32, value: Byte) -> Option<Byte> {
        if address > MAX_MEM as u32 {
            return None
        }
        else {
            self.data[address as usize] = value;
            return Some(self.data[address as usize])
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

    pub fn get_stats(&self, mem: &Mem) {
        print!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}", 
                self.pc, self.sp, self.a, self.x, self.y, self.c, 
                self.z, self.i, self.d, self.b, self.v, self.n);
        print!("{:#?}", mem.data);
    }

    // Reset the CPU
    pub fn reset(&mut self, mem: &Mem) {
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

        mem.initialize();
    }

    pub fn fetch_byte(&mut self, cycles: &mut u32, mem: &Mem) -> Byte {
        let data: Byte = mem.data[self.pc as usize];
        self.pc += 1;
        *cycles -= 1;
        return data;
    }

    // opcodes
    const INS_LDA_IM: Byte = 0xA9;

    pub fn execute(&mut self, cycles: &mut u32, mem: &Mem) {
        // Cycles = number of ticks to execute instructions
        while *cycles > 0 {
            let ins: Byte = self.fetch_byte(cycles, mem);
            match ins {
                INS_LDA_IM=> {
                    let value: Byte = self.fetch_byte(cycles, mem);
                    self.a = value;
                    self.z = (self.a == 0) as u8;
                    // TODO - need to fix logic below, think that's where the
                    //        problem in changing the registers is
                    self.n = self.a & 0b10000000;
                }
                _ => {println!("Instruction not handled")}
            }
        }
    }
}

fn main() {
    let mut cycles: u32 = 2;
    let mut mem: Mem = Mem::new();
    let mut cpu: CPU = CPU::new();
    cpu.reset(&mut mem);
    // Start - inline a little program
    mem.write(0xFFFC, CPU::INS_LDA_IM);
    // Above same thing as - mem.data[0xFFFC] = CPU::INS_LDA_IM;
    mem.write(0xFFFD, 0x42);
    // End - inline a little program

    cpu.execute(&mut cycles, &mut mem);
    let check_address: Option<u8> = mem.read(0xFFFD);
    print!("{:#?}\n", check_address);
    print!("{}\n", cpu.n);
}
