use crate::instruction::Opcode;

pub const MEMORY_SIZE: usize = 65_536; // 64KB

pub struct CPU {
    pub registers: [u32; 8],        // R0-R7
    pub pc: u16,                    // Program Counter
    pub sp: u16,                    // Stack Pointer
    pub z: bool,                    // Zero Flag
    pub memory: [u8; MEMORY_SIZE],  // Memory
    pub halted: bool,               // Halted State
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0; 8],
            pc: 0,
            sp: 0xFFFF,
            z: false,
            memory: [0; MEMORY_SIZE],
            halted: false,
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write_u8(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    pub fn fetch_u8(&mut self) -> u8 {
        let byte = self.read_u8(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    pub fn fetch_u16(&mut self) -> u16 {
        let lo = self.fetch_u8() as u16;
        let hi = self.fetch_u8() as u16;
        (hi << 8) | lo
    }

    pub fn step(&mut self) {
        let opcode_byte = self.fetch_u8();
        let opcode = Opcode::from_byte(opcode_byte).expect("Invalid opcode");
        
        match opcode {
            Opcode::Mov => {
                let dest = self.fetch_u8() as usize;
                let imm = self.fetch_u8() as u32;

                self.registers[dest] = imm;
                self.z = imm == 0;
            }

            Opcode::Add => {
                let dest = self.fetch_u8() as usize;
                let src1 = self.fetch_u8() as usize;
                let src2 = self.fetch_u8() as usize;

                let result = self.registers[src1].wrapping_add(self.registers[src2]);

                self.registers[dest] = result;
                self.z = result == 0;
            }

            Opcode::Sub => {
                let dest = self.fetch_u8() as usize;
                let src1 = self.fetch_u8() as usize;
                let src2 = self.fetch_u8() as usize;

                let result = self.registers[src1].wrapping_sub(self.registers[src2]);

                self.registers[dest] = result;
                self.z = result == 0;
            }

            Opcode::Load => {
                let dest = self.fetch_u8() as usize;
                let addr = self.fetch_u16();

                let b0 = self.read_u8(addr) as u32;
                let b1 = self.read_u8(addr.wrapping_add(1)) as u32;
                let b2 = self.read_u8(addr.wrapping_add(2)) as u32;
                let b3 = self.read_u8(addr.wrapping_add(3)) as u32;
                
                let value = b0 | (b1 << 8) | (b2 << 16) | (b3 << 24);

                self.registers[dest] = value;
                self.z = value == 0;
            }

            Opcode::Store => {
                let src = self.fetch_u8() as usize;
                let addr = self.fetch_u16();

                let value = self.registers[src];

                self.write_u8(addr, value as u8);
                self.write_u8(addr.wrapping_add(1), (value >> 8) as u8);
                self.write_u8(addr.wrapping_add(2), (value >> 16) as u8);
                self.write_u8(addr.wrapping_add(3), (value >> 24) as u8);
            }

            Opcode::Print => {
                let src = self.fetch_u8() as usize;
                println!("{}", self.registers[src]);
            }

            Opcode::Halt => {
                self.halted = true;
            }

            _ => unimplemented!(),
        }
    }
}