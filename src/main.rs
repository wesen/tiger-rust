#![feature(plugin)]
#![plugin(plex)]

pub mod lexer;
pub mod tiger;
pub mod ast;
pub mod symbol;
pub mod types;
pub mod type_check;
pub mod parser;

extern crate lalrpop_util;

use parser::parse;


fn main() {
    let _ = parse("nil");
    let _ = parse("123123");;
    let _ = parse(r#""foobar""#);
    let _ = parse(r#"int[5] of 2"#);
    let _ = parse(r#"foobar { i = 4, j = 2 }"#);
    let _ = parse(r#"foobar {  }"#);
    let _ = parse(r#"foobar"#);
    let _ = parse(r#"foobar[2]"#);
    let _ = parse(r#"foobar[2].foobar"#);
    let _ = parse(r#"foobar[2][3]"#);
    let _ = parse(r#"foobar.blabla"#);
    let _ = parse(r#"foobar.blabla[2]"#);
    let _ = parse(r#"foobar.blabla[2] of 4"#);
    let _ = parse(r#"foobar.blabla.blip.blip"#);
    let _ = parse(r#"foobar.blabla.blip.blip[foobar]"#);
    let _ = parse(r#"foobar("bla", angry.foobar, foo[0])"#);
    let _ = parse(r#"foo[34] of 34 + 36 + 89 * 89"#);
    let _ = parse(r#"if foobar.symbol then 23"#);

//    let lex = lexer::Lexer::new("function foobar() { 123; }");
//    for l in lex {
//        println!("{:?}", l);
//    }
}
