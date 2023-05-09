use crate::parser::KindOp;
use std::str::Chars;

#[derive(Debug)]
pub struct Lexer<'a> {
    src: Chars<'a>,
    len_remain: usize,
    curs: usize,
}
const EOF_CHAR: char = '\0';

impl<'a> Lexer<'a> {
    pub fn new(inp: &'a str) -> Self {
        Self {
            src: inp.chars(),
            len_remain: inp.len(),
            curs: 0,
        }
    }
    pub fn next_char(&mut self) -> char {
        self.curs += 1;
        self.len_remain -= 1;
        self.src.next().unwrap_or(EOF_CHAR)
    }
    pub fn get_len_remain(&self) -> usize {
        self.len_remain
    }
    pub fn is_eof(&self) -> bool {
        self.src.as_str().is_empty()
    }
    pub fn second(&self) -> char {
        let mut iter = self.src.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }
    pub fn bump(&mut self) -> Option<char> {
        Some(self.src.next()?)
    }
    pub fn return_ty_token(&mut self) -> KindOp {
        let kind = self.next_char();
        let ty = match kind {
            '(' => KindOp::OBracket,
            ')' => KindOp::CBracket,
            '+' => KindOp::Add,
            '-' => KindOp::Sub,
            ':' => KindOp::Sub,
            '*' => KindOp::Mul,
            '>' => KindOp::Greater,
            '<' => KindOp::Less,
            '/' => KindOp::Comment,
            ',' => KindOp::Comma,
            _ => KindOp::NotFound
        };
        ty
    }
}
