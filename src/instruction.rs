pub enum Opcode {
    Add = 0x01,
    Sub = 0x02,
    Mov = 0x03,
    Print = 0x04,
    Halt = 0xFF,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x01 => Some(Opcode::Add),
            0x02 => Some(Opcode::Sub),
            0x03 => Some(Opcode::Mov),
            0x04 => Some(Opcode::Print),
            0xFF => Some(Opcode::Halt),
            _ => None,
        }
    }
}