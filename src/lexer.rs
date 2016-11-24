use std::io::BufReader;

#[derive(PartialEq, Debug)]
pub enum Token {
    Ident(String),
    String(String),

    While,
    For,
    To,
    Break,
    Let,
    In,
    End,
    Function,
    Var,
    Type,
    Array,
    If,
    Then,
    Else,
    Do,
    Of,
    Nil,

    Integer(i32),
    Colon,
    Comma,
    SemiColon,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    NotEqual,
    Lower,
    LowerEqual,
    Greater,
    GreaterEqual,
    Ampersand,
    Pipe,
    Assign,

    Whitespace,
    Comment,
}

lexer! {
    fn next_token(text: 'a) -> (Token, &'a str);

    r#"[ \t\r\n]+"# => (Token::Whitespace, text),
    // "C-style" comments (/* .. */) - can't contain "*/"
    r#"/[*](~(.*[*]/.*))[*]/"# => (Token::Comment, text),
    // "C++-style" comments (// ...)
    r#"//[^\n]*"# => (Token::Comment, text),

    r#"while"# => (Token::While, text),
    r#"for"# => (Token::For, text),
    r#"to"# => (Token::To, text),
    r#"break"# => (Token::Break, text),
    r#"let"# => (Token::Let, text),
    r#"in"# => (Token::In, text),
    r#"end"# => (Token::End, text),
    r#"function"# => (Token::Function, text),
    r#"var"# => (Token::Var, text),
    r#"type"# => (Token::Type, text),
    r#"array"# => (Token::Array, text),
    r#"if"# => (Token::If, text),
    r#"then"# => (Token::Then, text),
    r#"else"# => (Token::Else, text),
    r#"do"# => (Token::Do, text),
    r#"of"# => (Token::Of, text),
    r#"nil"# => (Token::Nil, text),

        r#"[0-9]+"# => {
        (if let Ok(i) = text.parse() {
            Token::Integer(i)
        } else {
            panic!("integer {} is out of range", text)
        }, text)
    },

    r#""[a-zA-Z_][a-zA-Z0-9_]*""# => { let len = text.len();
               (Token::String(text[1..len-1].to_owned()), text)
    },
    r#"[a-zA-Z_][a-zA-Z0-9_]*"# => (Token::Ident(text.to_owned()), text),

    r#":"# => (Token::Colon, text),
    r#","# => (Token::Comma, text),
    r#";"# => (Token::SemiColon, text),
    r#"\("# => (Token::LParen, text),
    r#"\)"# => (Token::RParen, text),
    r#"\["# => (Token::LBracket, text),
    r#"\]"# => (Token::RBracket, text),
    r#"\{"# => (Token::LBrace, text),
    r#"\}"# => (Token::RBrace, text),
    r#"\."# => (Token::Dot, text),
    r#"\&"# => (Token::Ampersand, text),
    r#"\|"# => (Token::Pipe, text),

    r#"\+"# => (Token::Plus, text),
    r#"-"# => (Token::Minus, text),
    r#"\*"# => (Token::Star, text),
    r#"/"# => (Token::Slash, text),

    r#"="# => (Token::Equal, text),
    r#"<>"# => (Token::NotEqual, text),
    r#"<"# => (Token::Lower, text),
    r#"<="# => (Token::LowerEqual, text),
    r#">"# => (Token::Greater, text),
    r#">="# => (Token::GreaterEqual, text),
    r#":="# => (Token::Assign, text),

    r#"."# => panic!("unexpected character: {}", text),
}

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer { original: s, remaining: s }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

fn span_in(s: &str, t: &str) -> Span {
    let lo = s.as_ptr() as usize - t.as_ptr() as usize;
    Span {
        lo: lo,
        hi: lo + s.len(),
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (usize, Token, usize);
    fn next(&mut self) -> Option<(usize, Token, usize)> {
        loop {
            let tok = if let Some(tok) = next_token(&mut self.remaining) {
                tok
            } else {
                return None
            };
            match tok {
                (Token::Whitespace, _) | (Token::Comment, _) => {
                    continue;
                }
                (tok, span) => {
                    let s = span_in(span, self.original);
                    return Some((s.lo, tok, s.hi));
                }
            }
        }
    }
}

