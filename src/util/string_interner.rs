use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct Symbol(pub u32);

#[derive(Default, Debug)]
pub struct StringInterner {
    map: HashMap<&'static str, Symbol>,
    strings: Vec<&'static str>,
}

impl StringInterner {
    pub fn get_or_intern(&mut self, s: &str) -> Symbol {
        if let Some(&symbol) = self.map.get(s) {
            return symbol;
        }

        let s = s.to_owned().leak();
        let index = self.strings.len();
        let symbol = Symbol(index as u32);

        self.strings.push(s);
        self.map.insert(s, symbol);

        symbol
    }

    pub fn resolve(&self, index: Symbol) -> &'static str {
        self.strings[index.0 as usize]
    }
}
