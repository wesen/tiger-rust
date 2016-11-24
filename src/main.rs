#![feature(plugin)]
#![plugin(plex)]
extern crate plex;

pub mod calculator4;
pub mod lexer;
pub mod tiger;
pub mod ast;

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

fn main() {
    let program = "nil";
    let s = tiger::parse_Program(program);
    println!("{:?}", s);

    println!("{:?}", tiger::parse_Program("123"));
    println!("{:?}", tiger::parse_Program("\"foobar\""));
}
