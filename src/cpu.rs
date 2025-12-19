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