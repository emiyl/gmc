use num_enum::TryFromPrimitive;

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum Opcode {
    Mul = 0x08,
    Div = 0x09,
    Rem = 0x0A,
    Mod = 0x0B,
    Add = 0x0C,
    Sub = 0x0D,
    And = 0x0E,
    Or = 0x0F,
    Xor = 0x10,
    Neg = 0x11,
    Not = 0x12,
    Shl = 0x13,
    Shr = 0x14,
    Pop = 0x45,
    PushI = 0x84,
    PopZ = 0x9E,
    PushVar = 0xC0,
    Call = 0xD9,
}

pub struct Bytecode {
    pub data: Vec<u8>,
}

impl Bytecode {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn write_u32(&mut self, value: u32) {
        self.data.extend(value.to_le_bytes());
    }

    pub fn write_i32(&mut self, value: i32) {
        self.data.extend(value.to_le_bytes());
    }
}
