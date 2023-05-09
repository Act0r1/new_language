mod parser;
mod lexer;

use lexer::*;

fn main() {
    let s:&str = "var x = 5";
    let mut n = Lexer::new(s);
    while let Some(ch) = n.next_char() {
        println!("{:?}", ch);
        
    }
    println!("{:?}", n);
}
