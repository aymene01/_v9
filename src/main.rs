mod lexer;

use lexer::{Lexer, Token};
fn main() {
    let input = "function add(a, b) { return a + b; }".to_string();
    let mut lexer = Lexer::new(input);

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::Eof {
            break;
        }
    }
}
