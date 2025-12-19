use crate::instruction::Opcode;

pub const MEMORY_SIZE: usize = 65_536; // 64KB

pub struct CPU {
    pub registers: [i32; 8],        // R0-R7
    pub pc: u16,                    // Program Counter
    pub sp: u16,                    // Stack Pointer
    pub fp: u16,                    // Frame Pointer
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
            fp: 0xFFFF,
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

    pub fn push(&mut self, value: i32) {
        self.sp = self.sp.wrapping_sub(4);
        self.write_u8(self.sp, (value & 0xFF) as u8);
        self.write_u8(self.sp + 1, ((value >> 8) & 0xFF) as u8);
        self.write_u8(self.sp + 2, ((value >> 16) & 0xFF) as u8);
        self.write_u8(self.sp + 3, ((value >> 24) & 0xFF) as u8);
    }

    pub fn pop(&mut self) -> i32 {
        let b0 = self.read_u8(self.sp) as i32;
        let b1 = self.read_u8(self.sp + 1) as i32;
        let b2 = self.read_u8(self.sp + 2) as i32;
        let b3 = self.read_u8(self.sp + 3) as i32;

        self.sp = self.sp.wrapping_add(4);
        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
    }

    pub fn read_reg(&self, reg: u8) -> i32 {
        match reg {
            0..=7 => self.registers[reg as usize],
            8 => self.sp as i32,
            9 => self.fp as i32,
            _ => panic!("Invalid register"),
        }
    }

    pub fn write_reg(&mut self, reg: u8, value: i32) {
        match reg {
            0..=7 => self.registers[reg as usize] = value,
            8 => self.sp = value as u16,
            9 => self.fp = value as u16,
            _ => panic!("Invalid register"),
        }
    }

    pub fn step(&mut self) {
        let opcode_byte = self.fetch_u8();
        let opcode = Opcode::from_byte(opcode_byte).expect("Invalid opcode");
        
        match opcode {
            Opcode::Mov => {
                let dest = self.fetch_u8();
                let imm = self.fetch_u16() as i32;

                self.write_reg(dest, imm);
                self.z = imm == 0;
            }

            Opcode::Add => {
                let dest = self.fetch_u8();
                let src1 = self.fetch_u8();
                let src2 = self.fetch_u8();

                let result = self.read_reg(src1).wrapping_add(self.read_reg(src2));

                self.write_reg(dest, result);
                self.z = result == 0;
            }

            Opcode::Sub => {
                let dest = self.fetch_u8();
                let src1 = self.fetch_u8();
                let src2 = self.fetch_u8();

                let result = self.read_reg(src1).wrapping_sub(self.read_reg(src2));

                self.write_reg(dest, result);
                self.z = result == 0;
            }

            Opcode::Load => {
                let dest = self.fetch_u8();
                let addr = self.fetch_u16();

                let b0 = self.read_u8(addr) as i32;
                let b1 = self.read_u8(addr.wrapping_add(1)) as i32;
                let b2 = self.read_u8(addr.wrapping_add(2)) as i32;
                let b3 = self.read_u8(addr.wrapping_add(3)) as i32;
                
                let value = b0 | (b1 << 8) | (b2 << 16) | (b3 << 24);

                self.write_reg(dest, value);
                self.z = value == 0;
            }

            Opcode::Store => {
                let src = self.fetch_u8();
                let addr = self.fetch_u16();

                let value = self.read_reg(src);

                self.write_u8(addr, value as u8);
                self.write_u8(addr.wrapping_add(1), (value >> 8) as u8);
                self.write_u8(addr.wrapping_add(2), (value >> 16) as u8);
                self.write_u8(addr.wrapping_add(3), (value >> 24) as u8);
            }

            Opcode::Jmp => {
                let addr = self.fetch_u16();
                self.pc = addr;
            }

            Opcode::Jz => {
                let addr = self.fetch_u16();
                if self.z {
                    self.pc = addr;
                }
            }

            Opcode::Jnz => {
                let addr = self.fetch_u16();
                if !self.z {
                    self.pc = addr;
                }
            }

            Opcode::Call => {
                let addr = self.fetch_u16();
                self.push(self.fp as i32);
                self.push(self.pc as i32);
                self.fp = self.sp;
                self.pc = addr;
            }

            Opcode::Ret => {
                self.sp = self.fp;
                self.pc = self.pop() as u16;
                self.fp = self.pop() as u16;
            }

            Opcode::Push => {
                let reg = self.fetch_u8();
                self.push(self.read_reg(reg));
            }

            Opcode::Pop => {
                let reg = self.fetch_u8();
                let pop = self.pop();
                self.write_reg(reg, pop);
            }

            Opcode::LoadR => {
                let dest = self.fetch_u8();
                let base = self.fetch_u8();
                let offset = self.fetch_u16() as i16;

                let addr = self.read_reg(base).wrapping_add(offset as i32) as u16;
                let value = self.read_u8(addr) as i32 | 
                            ((self.read_u8(addr + 1) as i32) << 8) | 
                            ((self.read_u8(addr + 2) as i32) << 16) | 
                            ((self.read_u8(addr + 3) as i32) << 24);
                
                self.write_reg(dest, value);
                self.z = value == 0;
            }

            Opcode::StoreR => {
                let src = self.fetch_u8();
                let base = self.fetch_u8();
                let offset = self.fetch_u16() as i16;

                let addr = self.read_reg(base).wrapping_add(offset as i32) as u16;
                let value = self.read_reg(src);

                self.write_u8(addr, value as u8);
                self.write_u8(addr + 1, (value >> 8) as u8);
                self.write_u8(addr + 2, (value >> 16) as u8);
                self.write_u8(addr + 3, (value >> 24) as u8);
            }

            Opcode::MovR => {
                let dest = self.fetch_u8();
                let src = self.fetch_u8();

                let value = self.read_reg(src);
                self.write_reg(dest, value);
                self.z = value == 0;
            }

            Opcode::Print => {
                let src = self.fetch_u8();
                println!("{}", self.read_reg(src));
            }

            Opcode::Halt => {
                self.halted = true;
            }

            _ => unimplemented!(),
        }
    }
}