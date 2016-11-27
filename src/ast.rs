use symbol;

pub type Symbol = symbol::SymbolId;
pub type Position = usize;

#[derive(Debug,Clone,PartialEq)]
pub enum Var {
    SimpleVar(Symbol, Position),
    FieldVar(Box<Var>, Symbol, Position),
    SubscriptVar(Box<Var>, Box<Exp>, Position),
}

#[derive(Debug,Clone,PartialEq)]
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
//    NewExp(Symbol, Position),
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
        else_: Option<Box<Exp>>,
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

#[derive(Debug,Clone,PartialEq)]
pub enum Dec {
    FunDec {
        name: Symbol,
        params: Vec<Box<Field>>,
        result: Option<(Symbol,Position)>,
        body: Box<Exp>,
        pos: Position,
    },

    VarDec {
        name: Symbol,
        escape: bool,
        typ: Option<(Symbol, Position)>,
        init: Box<Exp>,
        pos: Position,
    },
    TypeDec {
        name: Symbol,
        ty: Box<Ty>,
        pos: Position,
    }
}

#[derive(Debug,Clone,PartialEq)]
pub struct Field {
    pub name: Symbol,
    pub escape: bool,
    pub typ: Symbol,
    pub pos: Position,
}

#[derive(Debug,Clone,PartialEq)]
pub enum Ty {
    NameTy(Symbol, Position),
    RecordTy(Vec<Box<Field>>),
    ArrayTy(Symbol, Position),
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Oper {
    PlusOp,
    MinusOp,
    TimesOp,
    DivideOp,
    EqOp,
    NeqOp,
    LtOp,
    LeOp,
    GtOp,
    GeOp
}