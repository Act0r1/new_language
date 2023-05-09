use std::str::Chars;

#[derive(Debug)]
pub struct Lexer<'a> {
    src: Chars<'a>,
    len_remain: usize,
    curs: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(inp: &'a str) -> Self {
        Self {
            src: inp.chars(),
            len_remain: inp.len(),
            curs: 0,
        }
    }
    pub fn next_char(&mut self) -> Option<char> {
        self.curs += 1;
        self.len_remain -= 1;
        self.src.next()
    }
    pub fn get_len_remain(&self) -> usize {
        self.len_remain
    }
    pub fn is_eof(&self) -> bool {
        self.src.as_str().is_empty()
    }
    pub fn bump(&mut self) -> Option<char> {
        let c = self.src.next()?;


        Some(c)
    }
}

#[derive(Debug)]
enum KindToken {
    Num,
    Bool,
    Str,
    Greater,
    Equal,
    Less,
    Comma,
    Comment,
    Mul,
    Div,
    Add,
    Sub,
    Func,
    OBracket,
    CBracket,
}
