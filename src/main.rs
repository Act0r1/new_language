mod lexer;
mod parser;

use lexer::*;
fn main() {
    let s: &str = "var x = 5";
    let mut n = Lexer::new(s);

    loop {
        let ch = n.next_char();
        if ch == EOF_CHAR {
            break;
        };

        println!("{:?}", ch);
    }
}
