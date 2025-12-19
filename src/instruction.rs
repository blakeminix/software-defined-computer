pub enum Opcode {
    Add = 0x01,
    Sub = 0x02,
    Mov = 0x03,
    Print = 0x04,
    Load = 0x05,
    Store = 0x06,
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
            0xFF => Some(Opcode::Halt),
            _ => None,
        }
    }
}