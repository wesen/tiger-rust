pub mod calculator1;
pub mod calculator2;
pub mod calculator3;
pub mod calculator4;

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

#[test]
fn calculator3() {
    assert!(calculator3::parse_Expr("22 + 23").is_ok());
    assert_eq!(calculator3::parse_Expr("22 + 23").unwrap(), 45);
    assert_eq!(calculator3::parse_Expr("2 + (3 * (4 + 5))").unwrap(), 29);
}

#[test]
fn calculator1() {
    assert!(calculator1::parse_Term("22").is_ok());
    assert!(calculator1::parse_Term("(22)").is_ok());
    assert!(calculator1::parse_Term("((((22))))").is_ok());
    assert!(calculator1::parse_Term("((22)").is_err());
}

#[test]
fn calculator2() {
    assert!(calculator2::parse_Term("22").is_ok());
    assert!(calculator2::parse_Term("(22)").is_ok());
    assert!(calculator2::parse_Term("((((22))))").is_ok());
    assert!(calculator2::parse_Term("((22)").is_err());
}

fn main() {
    println!("Hello, world!");
}
