use num_enum::TryFromPrimitive;

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum Opcode {
    PushI = 0x84,
    Add = 0x0C,
    Pop = 0x45,
    PushVar = 0xC0,
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
