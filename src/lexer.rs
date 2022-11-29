use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::TokenKind::QuotedString;

#[derive(Clone, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub file: (usize, Rc<String>), // FileId, File Contents
}

impl Span {
    pub fn new(start: usize, end: usize, file: (usize, Rc<String>)) -> Self {
        Self {
            start,
            end,
            file,
        }
    }
    pub fn inside(&self, other: &Span) -> bool {
        return self.file.0 == other.file.0 && self.start >= other.start && self.end <= other.end;
    }
}

#[derive(Clone)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)?;
        Ok(())
    }
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize, file: (usize, Rc<String>)) -> Self {
        Self {
            span: Span::new(start, end, file),
            kind,
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum NumericConstant {
    Integer(i32),
    Float(f32),
}

#[derive(Clone, Debug)]
pub enum TokenKind {
    Name(String),
    Number(NumericConstant),
    QuotedString(String),

    LParen,
    RParen,

    LCurly,
    RCurly,

    LBracket,
    RBracket,

    Plus,
    Minus,
    Divide,
    Multiply,

    Equals,

    Comma,
    Period,
    Semicolon,
    Colon,

    Proc,
    Var,
    Return,
    If,
    Begin,
    End,
}

pub fn lex(file: String) -> Vec<Token> {
    let mut index: usize = 0;
    let mut tokens = vec![];
    let mut acc = String::from("");

    //TODO: implement fileid
    let fileId = 0;
    let fileRc = Rc::new(file.clone());

    //TODO: RIGHT NOW THIS DOESNT SUPPORT FLOATING POINT NUMBERS
    let parse_numeric = |s: &String, index: usize| -> (NumericConstant, usize) { // (number, length)

        let mut size = 0usize;
        let mut acc = String::from("");

        for c in s.as_bytes().iter().skip(index) {
            let c = (*c) as char;
            if !c.is_numeric() {
                break;
            }

            acc.push(c);
            size += 1;
        }


        (NumericConstant::Integer(acc.parse::<i32>().unwrap()), size)
    };
    let parse_name = |s: &String, index: usize| -> (TokenKind, usize) {
        let mut size = 0usize;
        let mut acc = String::from("");

        for c in s.as_bytes().iter().skip(index) {
            let c = (*c) as char;
            if !c.is_alphabetic() && !c.is_numeric() {
                break;
            }

            acc.push(c);
            size += 1;
        }

        let token_kind = match acc.as_str() {
            "proc" => TokenKind::Proc,
            "var" => TokenKind::Var,
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "begin" => TokenKind::Begin,
            "end" => TokenKind::End,
            _ => TokenKind::Name(acc.clone())
        };

        (token_kind, size)
    };
    let parse_single = |s: &String, index: usize| {

        //['{', '}', '+', '-', '/', '*', '[', ']', '(', ')', ',', '.', ';', ':', '=']
        //FIXME: How did I even come up with this
        let token_kind = match *(s.as_bytes().iter().skip(index).take(1).collect::<Vec<_>>()[0]) {
            b'{' => TokenKind::LCurly,
            b'}' => TokenKind::RCurly,
            b'+' => TokenKind::Plus,
            b'-' => TokenKind::Minus,
            b'/' => TokenKind::Divide,
            b'*' => TokenKind::Multiply,
            b'[' => TokenKind::LBracket,
            b']' => TokenKind::RBracket,
            b'(' => TokenKind::LParen,
            b')' => TokenKind::RParen,
            b',' => TokenKind::Comma,
            b'.' => TokenKind::Period,
            b';' => TokenKind::Semicolon,
            b':' => TokenKind::Colon,
            b'=' => TokenKind::Equals,
            _ => unimplemented!()
        };

        token_kind
    };

    let parse_quoted = |s: &String, index: usize| -> (String, usize) { // (String inside quotes, size including quotes)

        let mut size = 1usize; // Including the left quote
        let mut acc = String::from("");

        for c in s.as_bytes().iter().skip(index + 1) { // + 1 to skip past the quote
            let c = (*c) as char;

            if c == '\"' {
                size += 1;
                break;
            }

            acc.push(c);
            size += 1;
        }
        (acc, size)
    };

    while index < file.len() {
        let c = file.as_bytes()[index] as char;

        if c.is_numeric() {
            let (numeric_constant, size) = parse_numeric(&file, index);
            tokens.push(
                Token::new(TokenKind::Number(numeric_constant),
                           index,
                           index + size,
                           (fileId, Rc::clone(&fileRc))));
            index += size;
        } else if c.is_alphabetic() {
            let (token_kind, size) = parse_name(&file, index);
            tokens.push(
                Token::new(token_kind, index, index + size, (fileId, Rc::clone(&fileRc)))
            );
            index += size;
        } else if ['{', '}', '+', '-', '/', '*', '[', ']', '(', ')', ',', '.', ';', ':', '='].contains(&c) {
            let token_kind = parse_single(&file, index);
            tokens.push(
                Token::new(token_kind, index, index + 1, (fileId, Rc::clone(&fileRc)))
            );
            index += 1;
        } else if c == '\"' {
            let (string, size) = parse_quoted(&file, index);

            tokens.push(
                Token::new(QuotedString(string), index, size, (fileId, Rc::clone(&fileRc)))
            );

            index += size;
        } else {
            //Whitespace
            index += 1;
        }
    }


    tokens
}