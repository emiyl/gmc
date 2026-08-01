use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct StringPool {
    strings: Vec<String>,
    index_by_value: HashMap<String, usize>,
}

impl StringPool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intern(&mut self, value: &str) -> usize {
        if let Some(id) = self.index_by_value.get(value) {
            return *id;
        }

        let id = self.strings.len();
        self.strings.push(value.to_string());
        self.index_by_value.insert(value.to_string(), id);
        id
    }

    pub fn values(&self) -> &[String] {
        &self.strings
    }
}
