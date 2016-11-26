use symbol::SymbolId;
use ast::Ty;

use std::collections::HashMap;

struct Table<'a, T: 'a> {
    parent: Option<&'a Table<'a, T>>,
    map: HashMap<SymbolId, Box<T>>,
}

impl<'a, T: 'a> Table<'a, T> {
    fn new(parent: Option<&'a Table<'a, T>>) -> Table<'a, T> {
        Table {
            map: HashMap::new(),
            parent: parent
        }
    }

    fn enter(&mut self, s: SymbolId, v: Box<T>) {
        self.map.insert(s, v);
    }

    fn look(&self, s: SymbolId) -> Option<&Box<T>> {
        let v = self.map.get(&s);
        match (v, self.parent.as_ref()) {
            (Some(_), _) => v,
            (None, Some(parent)) => parent.look(s),
            (None, None) => None,
        }
    }

    fn contains(&self, s: SymbolId) -> bool {
        match (self.map.contains_key(&s), self.parent.as_ref()) {
            (true, _) => true,
            (false, None) => false,
            (false, Some(parent)) => parent.contains(s)
        }
    }
}

#[test]
fn test_table() {
    use ast::Ty::*;

    let ty = Box::new(NameTy(0, 0));
    let ty2 = Box::new(NameTy(1, 1));

    let mut table: Table<Ty> = Table::new(None);
    table.enter(0, ty);
    assert_eq!(table.contains(0), true);
    assert_eq!(table.contains(1), false);
    let res = table.look(0);
    assert!(res.is_some());
    assert_eq!(&**res.unwrap(), &NameTy(0, 0));

    let mut t2: Table<Ty> = Table::new(Some(&table));
    t2.enter(1, ty2);
    assert_eq!(t2.contains(0), true);
    assert_eq!(t2.contains(1), true);
    assert_eq!(t2.contains(2), false);
    assert_eq!(&**t2.look(0).unwrap(), &NameTy(0,0));
    assert_eq!(&**t2.look(1).unwrap(), &NameTy(1,1));
}