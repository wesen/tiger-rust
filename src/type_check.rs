use ast;
use types::{Ty, ValueEnv, TypeEnv, EnvEntry};
use symbol::SymbolTable;

use std::cell::RefCell;

type AstTy = ast::Ty;
type AstEx = ast::Exp;

type Exp = ();

#[derive(Debug)]
pub struct ExpTy {
    exp: Exp,
    // this should be Rc<Ty>
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
    venv: &'a ValueEnv<'a>,
    tenv: &'a TypeEnv<'a>,
    unique_gen: &'a RefCell<UniqueGenerator>,
}

impl<'a> TypeChecker<'a> {
    fn new(symbol_table: &'a SymbolTable,
           venv: &'a ValueEnv<'a>,
           tenv: &'a TypeEnv<'a>,
           unique_gen: &'a RefCell<UniqueGenerator>) -> TypeChecker<'a> {
        TypeChecker {
            symbol_table: symbol_table,
            venv: venv,
            tenv: tenv,
            unique_gen: unique_gen
        }
    }

    fn new_with_venv(&self, venv: &'a ValueEnv<'a>) -> TypeChecker<'a> {
        TypeChecker {
            symbol_table: self.symbol_table,
            venv: venv,
            tenv: self.tenv,
            unique_gen: self.unique_gen,
        }
    }

    fn new_with_tenv(&self, tenv: &'a TypeEnv<'a>) -> TypeChecker<'a> {
        TypeChecker {
            symbol_table: self.symbol_table,
            venv: self.venv,
            tenv: tenv,
            unique_gen: self.unique_gen,
        }
    }

    fn trans_var(&self, var: &ast::Var) -> Result<ExpTy, String> {
        match var {
            &ast::Var::SimpleVar(symbol, pos) => {
                match self.venv.look(symbol) {
                    Some(rc_ty) => match rc_ty.as_ref() {
                        &EnvEntry::VarEntry(ref ty) => Ok(ExpTy { exp: (), ty: ty.as_ref().clone() }),
                        _ => {
                            let name = self.symbol_table.name(&symbol);
                            Err(format!("Unknown variable {} at pos {}", name, pos))
                        }
                    },
                    _ => {
                        let name = self.symbol_table.name(&symbol);
                        Err(format!("Unknown variable {} at pos {}", name, pos))
                    }
                }
            },
            &ast::Var::FieldVar(ref var, symbol, pos) => {
                // var must be of type RecordTy, and have a field matching symbol
                Err("unimplemented".to_string())
            },
            &ast::Var::SubscriptVar(ref var, ref exp, pos) => {
                Err("unimplemented".to_string())
            }
        }
    }

    pub fn trans_exp(&self, exp: &ast::Exp) -> Result<ExpTy, String> {
        use ast::Oper::*;

        match exp {
            &ast::Exp::VarExp(ref var) => self.trans_var(var),

            &ast::Exp::IntExp(_) => Ok(ExpTy { exp: (), ty: Ty::Int }),
            &ast::Exp::StringExp(_, _) => Ok(ExpTy { exp: (), ty: Ty::String }),
            &ast::Exp::NilExp => Ok(ExpTy { exp: (), ty: Ty::Nil }),

            &ast::Exp::CallExp { func, ref args, pos } => {
                Err("unimplemented".to_string())
            },

            &ast::Exp::OpExp { ref left, op, ref right, pos } => {
                let ExpTy { ty: left_ty, .. } = self.trans_exp(left)?;
                let ExpTy { ty: right_ty, .. } = self.trans_exp(right)?;

                match op {
                    PlusOp | MinusOp |
                    TimesOp | DivideOp |
                    LtOp | LeOp |
                    GtOp | GeOp => {
                        match (left_ty, right_ty) {
                            (Ty::Int, Ty::Int) => Ok(ExpTy { exp: (), ty: Ty::Int }),
                            _ => Err(format!("Integer required at {}", pos)),
                        }
                    },

                    EqOp | NeqOp => {
                        match (left_ty, right_ty) {
                            (Ty::Int, Ty::Int) => Ok(ExpTy { exp: (), ty: Ty::Int }),
                            _ => Err(format!("Integer required at {}", pos)),
                        }
                    }
                }
            },

            &ast::Exp::SeqExp(ref v) => {
                if v.len() == 0 {
                    Ok(ExpTy { exp: (), ty: Ty::Nil })
                } else {
                    self.trans_exp(&v[v.len() - 1])
                }
            },

            &ast::Exp::AssignExp { .. } => Ok(ExpTy { exp: (), ty: Ty::Unit }),

            &ast::Exp::IfExp { ref test, ref then_, ref else_, pos } => {
                let ExpTy { ty: test_ty, .. } = self.trans_exp(test)?;
                let then_ty = self.trans_exp(then_)?;

                if test_ty == Ty::Int {
                    if else_.is_some() {
                        let else_ty = self.trans_exp(else_.as_ref().unwrap())?;
                        if else_ty.ty == then_ty.ty {
                            Ok(ExpTy { exp: (), ty: else_ty.ty })
                        } else {
                            Err(format!("then ({:?} and else {:?} branch are not of the same type",
                                        then_ty.ty, else_ty.ty))
                        }
                    } else {
                        Ok(then_ty)
                    }
                } else {
                    Err(format!("integer required for test at {}", pos))
                }
            },

            &ast::Exp::WhileExp { ref test, ref body, pos } => {
                let ExpTy { ty: test_ty, .. } = self.trans_exp(test)?;
                let _ = self.trans_exp(body)?;
                if test_ty == Ty::Int {
                    Ok(ExpTy { ty: Ty::Unit, exp: () })
                } else {
                    Err(format!("integer required for test at {}", pos))
                }
            },

            &ast::Exp::ForExp { var, ref lo, ref hi, ref body, pos, .. } => {
                let ExpTy { ty: lo_ty, .. } = self.trans_exp(lo)?;
                let ExpTy { ty: hi_ty, .. } = self.trans_exp(hi)?;
                // add var to environment
                let _ = self.trans_exp(body)?;
                Err("unimplemented".to_string())
            }

            &ast::Exp::BreakExp(pos) => Ok(ExpTy { ty: Ty::Unit, exp: () }),

            &ast::Exp::RecordExp { ref fields, typ, pos } => {
                Err("unimplemented".to_string())
            },

            _ => {
                Err("unimplemented".to_string())
            },
        }
    }
}

#[test]
fn test_trans_exp() {
    use parser::parse;

    let (p, symbol_table) = parse("2 + 2").unwrap();
    let venv = ValueEnv::new(None);
    let tenv = TypeEnv::new(None);
    let unique_gen = RefCell::new(UniqueGenerator::new());
    let mut type_checker = TypeChecker::new(&symbol_table, &venv, &tenv, &unique_gen);

    let venv2 = ValueEnv::new(Some(&venv));
    let mut tcheck2 = TypeChecker::new_with_venv(&mut type_checker, &venv2);
    let ExpTy { ty, .. } = tcheck2.trans_exp(&*p).unwrap();
    println!("{:?}", ty);
}