use ast;
use types::{Ty, ValueEnv, TypeEnv};

type AstTy = ast::Ty;

type Exp = ();

struct ExpTy {
    exp: Exp,
    ty: AstTy
}

pub fn trans_exp(venv: &ValueEnv, tenv: &TypeEnv, exp: &ast::Exp) -> Exp {
    ()
}

#[test]
fn test_trans_exp() {
    let venv = ValueEnv::new(None);
    let tenv = TypeEnv::new(None);
    let exp = ast::Exp::NilExp;
    assert_eq!(trans_exp(&venv, &tenv, &exp), ());
}