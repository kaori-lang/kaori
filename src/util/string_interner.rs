use foldhash::HashMap;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct StringIndex(pub u32);

#[derive(Default, Debug)]
pub struct StringInterner {
    map: HashMap<&'static str, usize>,
    strings: Vec<&'static str>,
}

impl StringInterner {
    pub fn get_or_intern(&mut self, s: &str) -> StringIndex {
        if let Some(&index) = self.map.get(s) {
            return StringIndex(index as u32);
        }

        let s = s.to_owned().leak();
        let index = self.strings.len();
        self.strings.push(s);
        self.map.insert(s, index);

        StringIndex(index as u32)
    }

    pub fn resolve(&self, index: StringIndex) -> &'static str {
        self.strings[index.0 as usize]
    }
}
