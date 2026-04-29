use ahash::HashMap;

pub struct StringInterner {
    map: HashMap<String, usize>,
    strings: Vec<String>,
}

impl StringInterner {
    pub fn intern(&mut self, s: &str) -> usize {
        if let Some(&key) = self.map.get(s) {
            return key;
        }

        let key = self.strings.len() as usize;
        self.strings.push(s.to_owned());
        self.map.insert(s.to_owned(), key);
        key
    }

    pub fn resolve(&self, key: usize) -> &str {
        &self.strings[key as usize]
    }
}
