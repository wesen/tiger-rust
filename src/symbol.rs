use std::collections::HashMap;

pub type SymbolId = u32;

pub struct SymbolTable {
    str_to_sym: HashMap<String, SymbolId>,
    sym_to_str: HashMap<SymbolId, String>,
    next_sym: SymbolId,
}

impl<'a> SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            str_to_sym: HashMap::new(),
            sym_to_str: HashMap::new(),
            next_sym: SymbolId::default(),
        }
    }

    pub fn name(&'a self, id: &SymbolId) -> Option<&'a String> {
        self.sym_to_str.get(id)
    }

    pub fn symbol(&mut self, s: &str) -> SymbolId {
        if self.str_to_sym.contains_key(s) {
            *self.str_to_sym.get(s).unwrap()
        } else {
            let sym = self.next_sym;
            self.next_sym += 1;
            self.str_to_sym.insert(s.to_owned(), sym);
            self.sym_to_str.insert(sym, s.to_owned());
            sym
        }
    }
}

#[test]
fn test_symbol_table() {
    let mut s = SymbolTable::new();

    assert_eq!(s.symbol("foobar"), 0);
    assert_eq!(s.name(&0).unwrap(), "foobar");
    assert_eq!(s.symbol("one"), 1);
}