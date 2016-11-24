use std::io::BufReader;

use self::Token::TokA;

#[derive(PartialEq,Debug)]
pub enum Token {
    TokA(String)
}

#[derive(PartialEq,Debug)]
pub enum TokenB {
    TokB(String)
}
