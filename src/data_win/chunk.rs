use crate::data_win::string_pool::StringPool;

#[derive(Clone, Copy, Debug)]
pub enum Patch {
    Str(usize, usize),
    Local(usize, usize),
}

#[derive(Debug)]
pub struct ChunkBuilder {
    pub name: [u8; 4],
    pub data: Vec<u8>,
    pub patches: Vec<Patch>,
}

impl ChunkBuilder {
    pub fn new(name: &str) -> Self {
        let bytes = name.as_bytes();
        assert_eq!(bytes.len(), 4, "chunk names are always 4 bytes");

        let mut chunk_name = [0u8; 4];
        chunk_name.copy_from_slice(bytes);

        Self {
            name: chunk_name,
            data: Vec::new(),
            patches: Vec::new(),
        }
    }

    pub fn empty_list(name: &str) -> Self {
        let mut chunk = Self::new(name);
        chunk.u32(0);
        chunk
    }

    pub fn pos(&self) -> usize {
        self.data.len()
    }

    pub fn u8(&mut self, value: u8) {
        self.data.push(value);
    }

    pub fn u16(&mut self, value: u16) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn u32(&mut self, value: u32) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn i32(&mut self, value: i32) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn u64(&mut self, value: u64) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn f32(&mut self, value: f32) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    pub fn bool32(&mut self, value: bool) {
        self.u32(if value { 1 } else { 0 });
    }

    pub fn bytes(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn zero_bytes(&mut self, count: usize) {
        self.data.extend(std::iter::repeat(0u8).take(count));
    }

    pub fn set_u32(&mut self, pos: usize, value: u32) {
        self.data[pos..pos + 4].copy_from_slice(&value.to_le_bytes());
    }

    pub fn str_ref(&mut self, pool: &mut StringPool, value: &str) {
        let id = pool.intern(value);
        let pos = self.pos();
        self.u32(0);
        self.patches.push(Patch::Str(pos, id));
    }

    pub fn local_ref_placeholder(&mut self) -> usize {
        let pos = self.pos();
        self.u32(0);
        pos
    }

    pub fn local_ref_set(&mut self, placeholder_pos: usize, target_rel: usize) {
        self.patches.push(Patch::Local(placeholder_pos, target_rel));
    }
}
