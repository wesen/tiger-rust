#![feature(plugin)]
#![plugin(plex)]

pub mod calculator4;
pub mod lexer;
pub mod tiger;
pub mod ast;

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
    let mut errors = Vec::new();
    assert_eq!(&format!("{:?}", calculator4::parse_Exprs(&mut errors, "22 * + 3").unwrap()),
    "[((22 * error) + 3)]");
    assert_eq!(&format!("{:?}", calculator4::parse_Exprs(&mut errors, "22 * 44 + 66, *3").unwrap()),
    "[((22 * 44) + 66), (error * 3)]");
    assert_eq!(&format!("{:?}", calculator4::parse_Exprs(&mut errors, "*").unwrap()),
    "[(error * error)]");

    assert_eq!(errors.len(), 4);
}

fn parse(s: &str) -> Result<Box<ast::Exp>, lalrpop_util::ParseError<usize,lexer::Token,()>> {
    let l = Lexer::new(s);
    tiger::parse_Program(l)
}

fn main() {
    println!("{:?}", parse("nil"));;
    println!("{:?}", parse("123123"));;
    println!("{:?}", parse(r#""foobar""#));
    println!("{:?}", parse(r#"int[5] of 2"#));
    println!("{:?}", parse(r#"foobar { i = 4, j = 2 }"#));
    println!("{:?}", parse(r#"foobar {  }"#));
    println!("{:?}", parse(r#"foobar"#));
    println!("{:?}", parse(r#"foobar.blabla"#));
    println!("{:?}", parse(r#"foobar.blabla.blip.blip"#));
    println!("{:?}", parse(r#"foobar.blabla.blip.blip[foobar]"#));

//    let lex = lexer::Lexer::new("function foobar() { 123; }");
//    for l in lex {
//        println!("{:?}", l);
//    }
}
