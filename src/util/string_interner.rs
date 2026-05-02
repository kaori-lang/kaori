use foldhash::HashMap;

#[derive(Default, Debug)]
pub struct StringInterner {
    map: HashMap<&'static str, usize>,
    strings: Vec<&'static str>,
}

impl StringInterner {
    pub fn get_or_intern(&mut self, s: &str) -> usize {
        if let Some(&index) = self.map.get(s) {
            return index;
        }

        let s = s.to_owned().leak();
        let index = self.strings.len();
        self.strings.push(s);
        self.map.insert(s, index);

        index
    }

    pub fn resolve(&self, index: usize) -> &'static str {
        self.strings[index]
    }
}
