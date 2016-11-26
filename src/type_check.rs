use ast;
use types::{Ty, ValueEnv, TypeEnv};
use symbol::SymbolTable;

type AstTy = ast::Ty;
type AstEx = ast::Exp;

type Exp = ();

#[derive(Debug)]
pub struct ExpTy {
    exp: Exp,
    ty: Ty,
}

struct UniqueGenerator {
    unique: u32,
}

impl UniqueGenerator {
    fn new() -> UniqueGenerator {
        UniqueGenerator { unique: 0 }
    }

    fn next(&mut self) -> u32 {
        let ret = self.unique;
        self.unique += 1;
        ret
    }
}

pub struct TypeChecker<'a> {
    symbol_table: &'a SymbolTable,
    initial_venv: ValueEnv<'a>,
    initial_tenv: TypeEnv<'a>,
    unique_gen: UniqueGenerator,
}

impl<'a> TypeChecker<'a> {
    fn new(symbol_table: &'a SymbolTable) -> TypeChecker {
        let venv = ValueEnv::new(None);
        let tenv = TypeEnv::new(None);

        TypeChecker {
            symbol_table: symbol_table,
            initial_venv: venv,
            initial_tenv: tenv,
            unique_gen: UniqueGenerator::new()
        }
    }

    pub fn trans_exp(&self, exp: &ast::Exp) -> ExpTy {
        match exp {
            &ast::Exp::OpExp { ref left, op, ref right, pos } => {
                let ExpTy { ty: left_ty, .. } = self.trans_exp(left);
                let ExpTy { ty: right_ty, .. } = self.trans_exp(right);

                match (left_ty, right_ty) {
                    (Ty::Int, Ty::Int) => ExpTy { exp: (), ty: Ty::Int },
                    _ => panic!("Integer required at {}", pos),
                }
            },
            &ast::Exp::IntExp(_) => ExpTy { exp: (), ty: Ty::Int },
            _ => ExpTy { exp: (), ty: Ty::Unit },
        }
    }
}

#[test]
fn test_trans_exp() {
    use parser::parse;

    let (p, symbol_table) = parse("2 + 2").unwrap();
    let type_checker = TypeChecker::new(&symbol_table);
    let ExpTy { ty, .. } = type_checker.trans_exp(&*p);
    println!("{:?}", ty);
}