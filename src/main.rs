// On Video #1, timestamp 35:00
//  https://youtu.be/qJgsuQoy9bc?si=VPn5THsHB6xoRgiL

// TODO - Check if functions returning an Option<> would be better off with a Result<>

// Stop unused functions warning
#![allow(dead_code, unused_variables)]

mod constants;
mod statusbits;
mod opcodes;

type Byte = constants::Byte;
type Word = constants::Word;
type SByte = constants::SByte;

const MAX_MEM: usize = constants::MAX_MEM;

#[derive(Debug)]
struct Mem {
    data: [Byte; MAX_MEM]
}

impl Mem {
    pub fn new() -> Self {
        Mem {
            data: [0; MAX_MEM]
        }
    }

    pub fn initialize(&self) -> Self {
        Mem {
            data: [0; MAX_MEM]
        }
    }

    pub fn dump_range(&self, start: usize, length: usize) {
        let end = (start + length).min(MAX_MEM);
        
        println!("Memory Dump (0x{:04X} - 0x{:04X}):", start, end - 1);
        println!("Addr  | 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F | ASCII");
        println!("------+-------------------------------------------------+----------------");
        
        for base in (start..end).step_by(16) {
            print!("{:04X} | ", base);
            
            for offset in 0..16 {
                if base + offset < end {
                    print!("{:02X} ", self.data[base + offset]);
                } else {
                    print!("   ");
                }
            }
            
            print!("| ");
            for offset in 0..16 {
                if base + offset < end {
                    let byte: Byte= self.data[base + offset];
                    if byte >= 32 && byte <= 126 {
                        print!("{}", char::from(byte));
                    } else {
                        print!(".");
                    }
                }
            }
            println!();
        }
    }
    
    pub fn dump_all(&self) {
        self.dump_range(0, MAX_MEM);
    }
    
    pub fn dump_around_address(&self, address: usize, context: usize) {
        let start: usize = address.saturating_sub(context);
        let length: usize = context * 2 + 1;
        self.dump_range(start, length);
    }

    pub fn read(&self, address: u32) -> Option<Byte>{
        if address > u32::try_from(MAX_MEM).unwrap() {
            return None
        } else {
            return Some(self.data[address as usize])
        }
    }

    pub fn write(&mut self, address: u32, value: Byte) -> Option<Byte> {
        if address > u32::try_from(MAX_MEM).unwrap() {
            return None
        } else {
            self.data[usize::try_from(address).unwrap()] = value;
            return Some(self.data[usize::try_from(address).unwrap()])
        }
    }
}

#[derive(Debug)]
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

    pub fn get_stats(&self) {
        print!("pc = {}\nsp = {}\na = {}\nx = {}\ny = {}\nc = {}\nz = {}\ni = {}\nd = {}\nb = {}\nv = {}\nn = {}\n", 
                self.pc, self.sp, self.a, self.x, self.y, self.c, 
                self.z, self.i, self.d, self.b, self.v, self.n);
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

    fn fetch_byte(&mut self, cycles: &mut u32, mem: &Mem) -> Byte {
        let data: Byte = mem.data[usize::from(self.pc)];
        self.pc += 1;
        *cycles -= 1;
        return data;
    }

    fn fetch_sbyte(&mut self, cycles: &mut u32, mem: &Mem) -> SByte {
        return SByte::from(self.fetch_byte(cycles, mem));
    }

    fn fetch_word(&mut self, cycles: &mut u32, mem: &Mem) -> Word {
        let data: Word = u16::from(mem.data[usize::from(self.pc)]);
        self.pc += 1;
        *cycles -= 2;
        return data;
    }

    fn read_byte(&self, cycles: &mut u32, address: Byte, mem: &Mem) -> Byte {
        // Does not increment pc, not executing code only reading memory
        let data: Byte = mem.data[usize::from(address)];
        *cycles -= 1;
        return data;
    }

    fn read_word(&self, cycles: &mut u32, address: Word, mem: &Mem) -> Word {
        let lo_byte: Byte = self.read_byte(cycles, u8::try_from(address).unwrap(), mem);
        let hi_byte: Byte = self.read_byte(cycles, u8::try_from(address + 1).unwrap(), mem);
        return u16::from(lo_byte | (hi_byte << u8::from(8)));
    }

    fn write_byte(&mut self, value: Byte, cycles: &mut u32, address: Word, mem: &mut Mem) {
        // Write one byte to memory
        mem.data[usize::from(address)] = value;
        *cycles -= 1;
    }

    fn write_word(&mut self, value: Word, cycles: &mut u32, address: Word, mem: &mut Mem) {
        // Write 2 bytes to memory
        mem.data[usize::from(address)] = u8::try_from(value & 0xFF).unwrap();
        mem.data[usize::from(address + 1)] = u8::try_from(value >> 8).unwrap();
        *cycles -=2;
    }

    fn sp_to_address(&self) -> Word {
        return 0x100 | self.sp;
    }

    fn push_word_to_stack(&mut self, value: Word, cycles: &mut u32, mem: &mut Mem) {
        self.write_byte(u8::try_from(value >> 8).unwrap(), cycles, self.sp_to_address(), mem);
        self.sp -= 1;
        self.write_byte(u8::try_from(value & 0xFF).unwrap(), cycles, self.sp_to_address(), mem);
        self.sp -= 1;
    }

    fn push_pc_to_stack(&mut self, cycles: &mut u32, mem: &mut Mem) {
        // Pushes the PC to stack
        self.push_word_to_stack(self.pc, cycles, mem);
    }

    fn push_pc_minus_one_to_stack(&mut self, cycles: &mut u32, mem: &mut Mem) {
        // Pushes the PC-1 to stack
        self.push_word_to_stack(self.pc - 1, cycles, mem);
    }

    fn push_pc_plus_one_to_stack(&mut self, cycles: &mut u32, mem: &mut Mem) {
        // Pushes the PC to stack
        self.push_word_to_stack(self.pc + 1, cycles, mem);
    }

    fn lda_set_status(&mut self) {
        self.z = Byte::from(self.a == 0);
        self.n = Byte::from((self.a & statusbits::status::NEGATIVE) > 0);
    }

    pub fn execute(&mut self, cycles: &mut u32, mem: &Mem) {
        // Cycles = number of ticks to execute instructions
        while *cycles > 0 {
            let ins: Byte = self.fetch_byte(cycles, mem);
            match ins {
                opcodes::lda::IMMEDIATE => {
                    let value: Byte = self.fetch_byte(cycles, mem);
                    self.a = value;
                    self.lda_set_status();
                }
                opcodes::lda::ZERO_PAGE => {
                    let zero_page_address: Byte = self.fetch_byte(cycles, mem);
                    self.a = self.read_byte(cycles, zero_page_address, mem);
                    self.lda_set_status();
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
    mem.write(0xFFFC, opcodes::lda::IMMEDIATE);
    mem.write(0xFFFD, 0x42);
    // End - inline a little program

    cpu.execute(&mut cycles, &mut mem);
    
    cpu.get_stats();
    mem.dump_range(0xFFFA, 0xFFFF);
}
