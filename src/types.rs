use symbol::SymbolId;
use ast::Ty;

use std::collections::HashMap;

struct Table {
    map: HashMap<SymbolId, Box<Ty>>,
}

impl Table {
    fn new() -> Table {
        Table { map: HashMap::new() }
    }

    fn enter(&mut self, s: SymbolId, ty: Box<Ty>) {
        self.map.insert(s, ty);
    }

    fn look(&self, s: SymbolId) -> Option<&Box<Ty>> {
        self.map.get(&s)
    }

    fn contains(&self, s: SymbolId) -> bool {
        self.map.contains_key(&s)
    }
}

#[test]
fn test_table() {
    use ast::Ty::*;

    let ty = Box::new(NameTy(0, 0));
    let mut table = Table::new();
    table.enter(0, ty);
    assert_eq!(table.contains(0), true);
    assert_eq!(table.contains(1), false);
    let res = table.look(0);
    assert!(res.is_some());
    assert_eq!(&**res.unwrap(), &NameTy(0,0));
}