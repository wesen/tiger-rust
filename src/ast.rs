use std::fmt::{Debug, Formatter, Error};

pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Error
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Error => write!(fmt, "error"),
        }
    }
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}

pub type Position = usize;
pub type Symbol = String;

#[derive(Debug)]
pub enum Var {
    SimpleVar(Symbol, Position),
    FieldVar(Box<Var>, Symbol, Position),
    SubscriptVar(Box<Var>, Box<Exp>, Position),
}

#[derive(Debug)]
pub enum Exp {
    VarExp(Box<Var>),
    NilExp,
    IntExp(i32),
    StringExp(String, Position),
    CallExp {
        func: Symbol,
        args: Vec<Box<Exp>>,
        pos: Position,
    },
    NewExp(Symbol, Position),
    OpExp {
        left: Box<Exp>,
        op: Oper,
        right: Box<Exp>,
        pos: Position,
    },
    RecordExp {
        fields: Vec<(Symbol, Box<Exp>, Position)>,
        typ: Symbol,
        pos: Position,
    },
    SeqExp(Vec<Box<Exp>>),
    AssignExp {
        var: Box<Var>,
        exp: Box<Exp>,
        pos: Position,
    },
    IfExp {
        test: Box<Exp>,
        then_: Box<Exp>,
        else_: Box<Exp>,
        pos: Position,
    },
    WhileExp {
        test: Box<Exp>,
        body: Box<Exp>,
        pos: Position,
    },
    ForExp {
        var: Symbol,
        escape: bool,
        lo: Box<Exp>,
        hi: Box<Exp>,
        body: Box<Exp>,
        pos: Position
    },
    BreakExp(Position),
    LetExp {
        decs: Vec<Box<Dec>>,
        body: Box<Exp>,
        pos: Position,
    },
    ArrayExp {
        typ: Symbol,
        size: Box<Exp>,
        init: Box<Exp>,
        pos: Position,
    },
}

#[derive(Debug)]
pub struct FunDecl {
    name: Symbol,
    ty: Box<Ty>,
    pos: Position,
}

#[derive(Debug)]
pub struct TypeDecl {
    name: Symbol,
    ty: Box<Ty>,
    pos: Position,
}

#[derive(Debug)]
pub enum Dec {
    FunctionDec(Vec<Box<FunDecl>>),
    VarDec {
        name: Symbol,
        escape: bool,
        typ: Option<(Symbol, Position)>,
        init: Box<Exp>,
        pos: Position,
    },
    TypeDec(Vec<Box<TypeDecl>>),
}

#[derive(Debug)]
pub struct Field {
    name: Symbol,
    escape: bool,
    typ: Symbol,
    pos: Position,
}

#[derive(Debug)]
pub enum Ty {
    NameTy(Symbol, Position),
    RecordTy(Vec<Box<Field>>),
    ArrayTy(Symbol, Position),
}

#[derive(Debug)]
pub enum Oper {
    PlusOp, MinusOp, TimesOp, DivideOp, EqOp, NeqOp, LtOp, LeOp, GtOp, GeOp
}