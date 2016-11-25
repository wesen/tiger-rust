#![feature(plugin)]
#![plugin(plex)]

pub mod calculator4;
pub mod lexer;
pub mod tiger;
pub mod ast;
pub mod symbol;

extern crate lalrpop_util;

use lexer::Lexer;

#[test]
fn calculator4() {
    assert_eq!(&format!("{:?}", calculator4::parse_Expr("22 * 44 + 66").unwrap()),
    "((22 * 44) + 66)");
    assert_eq!(&format!("{:?}", calculator4::parse_Exprs("").unwrap()),
    "[]");
    assert_eq!(&format!("{:?}", calculator4::parse_Exprs("22 * 44 + 66").unwrap()),
    "[((22 * 44) + 66)]");
    assert_eq!(&format!("{:?}", calculator4::parse_Exprs("22 * 44 + 66,").unwrap()),
    "[((22 * 44) + 66)]");
    assert_eq!(&format!("{:?}", calculator4::parse_Exprs("22 * 44 + 66, 13*3").unwrap()),
    "[((22 * 44) + 66), (13 * 3)]");
    assert_eq!(&format!("{:?}", calculator4::parse_Exprs("22 * 44 + 66, 13*3,").unwrap()),
    "[((22 * 44) + 66), (13 * 3)]");
}

fn parse(s: &str) {
    let mut st = symbol::SymbolTable::new();

    let l = Lexer::new(s, &mut st);
    let p = tiger::parse_Program(l);
    println!("{}", s);
    println!("{:?}", p);
    println!("");
}

fn main() {
    parse("nil");;
    parse("123123");;
    parse(r#""foobar""#);
    parse(r#"int[5] of 2"#);
    parse(r#"foobar { i = 4, j = 2 }"#);
    parse(r#"foobar {  }"#);
    parse(r#"foobar"#);
    parse(r#"foobar[2]"#);
    parse(r#"foobar[2].foobar"#);
    parse(r#"foobar[2][3]"#);
    parse(r#"foobar.blabla"#);
    parse(r#"foobar.blabla[2]"#);
    parse(r#"foobar.blabla[2] of 4"#);
    parse(r#"foobar.blabla.blip.blip"#);
    parse(r#"foobar.blabla.blip.blip[foobar]"#);
    parse(r#"foobar("bla", angry.foobar, foo[0])"#);
    parse(r#"foo[34] of 34 + 36 + 89 * 89"#);
    parse(r#"if foobar.symbol then 23"#);

//    let lex = lexer::Lexer::new("function foobar() { 123; }");
//    for l in lex {
//        println!("{:?}", l);
//    }
}
