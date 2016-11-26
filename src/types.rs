use symbol::SymbolId;

pub type Unique = u32;

#[derive(Debug,PartialEq,Clone)]
pub enum Ty<'a> {
    Int,
    String,
    Nil,
    Record {
        unique: Unique,
        fields: Vec<(SymbolId, &'a Ty<'a>)>,
    },
    Array {
        typ: Box<Ty<'a>>,
        unique: Unique,
    },
    Unit,
    Name(SymbolId, Option<&'a Box<Ty<'a>>>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum EnvEntry<'a> {
    VarEntry(Ty<'a>),
    FunEntry {
        formals: Vec<Ty<'a>>,
        result: Ty<'a>
    }
}

use std::collections::BTreeMap;

pub struct Table<'a, T: 'a> {
    parent: Option<&'a Table<'a, T>>,
    map: BTreeMap<SymbolId, Box<T>>,
}

impl<'a, T: 'a> Table<'a, T> {
    pub fn new(parent: Option<&'a Table<'a, T>>) -> Table<'a, T> {
        Table {
            map: BTreeMap::new(),
            parent: parent
        }
    }

    pub fn enter(&mut self, s: SymbolId, v: Box<T>) {
        self.map.insert(s, v);
    }

    pub fn look(&self, s: SymbolId) -> Option<&Box<T>> {
        let v = self.map.get(&s);
        match (v, self.parent.as_ref()) {
            (Some(_), _) => v,
            (None, Some(parent)) => parent.look(s),
            (None, None) => None,
        }
    }

    pub fn contains(&self, s: SymbolId) -> bool {
        match (self.map.contains_key(&s), self.parent.as_ref()) {
            (true, _) => true,
            (false, None) => false,
            (false, Some(parent)) => parent.contains(s)
        }
    }
}

#[test]
fn test_table() {
    let ty = Box::new(Ty::Int);
    let ty2 = Box::new(Ty::String);

    let mut table: Table<Ty> = Table::new(None);
    table.enter(0, ty);
    assert_eq!(table.contains(0), true);
    assert_eq!(table.contains(1), false);
    let res = table.look(0);
    assert!(res.is_some());
    assert_eq!(&**res.unwrap(), &Ty::Int);

    let mut t2: Table<Ty> = Table::new(Some(&table));
    t2.enter(1, ty2);
    assert_eq!(t2.contains(0), true);
    assert_eq!(t2.contains(1), true);
    assert_eq!(t2.contains(2), false);
    assert_eq!(&**t2.look(0).unwrap(), &Ty::Int);
    assert_eq!(&**t2.look(1).unwrap(), &Ty::String);
}

pub type TypeEnv<'a> = Table<'a, Ty<'a>>;
pub type ValueEnv<'a> = Table<'a, EnvEntry<'a>>;

