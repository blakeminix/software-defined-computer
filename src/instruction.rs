pub enum Opcode {
    Add = 0x01,
    Sub = 0x02,
    Mov = 0x03,
    Print = 0x04,
    Load = 0x05,
    Store = 0x06,
    Jmp = 0x07,
    Jz = 0x08,
    Jnz = 0x09,
    Call = 0x0A,
    Ret = 0x0B,
    Push = 0x0C,
    Pop = 0x0D,
    Halt = 0xFF,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x01 => Some(Opcode::Add),
            0x02 => Some(Opcode::Sub),
            0x03 => Some(Opcode::Mov),
            0x04 => Some(Opcode::Print),
            0x05 => Some(Opcode::Load),
            0x06 => Some(Opcode::Store),
            0x07 => Some(Opcode::Jmp),
            0x08 => Some(Opcode::Jz),
            0x09 => Some(Opcode::Jnz),
            0x0A => Some(Opcode::Call),
            0x0B => Some(Opcode::Ret),
            0x0C => Some(Opcode::Push),
            0x0D => Some(Opcode::Pop),
            0xFF => Some(Opcode::Halt),
            _ => None,
        }
    }
}