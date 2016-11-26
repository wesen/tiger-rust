use ast;
use lalrpop_util;
use lexer;
use symbol;
use tiger;

pub fn parse(s: &str) -> Result<(Box<ast::Exp>, Box<symbol::SymbolTable>),
    lalrpop_util::ParseError<usize, lexer::Token, ()>> {
    let mut st = Box::new(symbol::SymbolTable::new());

    let p;
    {
        let l = lexer::Lexer::new(s, &mut st);
        p = tiger::parse_Program(l)?;
    }
    Ok((p, st))
}
