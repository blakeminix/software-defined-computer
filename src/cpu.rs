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
}