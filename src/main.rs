mod lexer;

use lexer::{Lexer, Token};
use std::fs;
fn main() {
    let file_content = fs::read_to_string("./js/test.js").expect("Failed to read the file.");

    // Create a new Lexer and tokenize the JavaScript code.
    let mut lexer = Lexer::new(file_content);

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::Eof {
            break;
        }
    }
}
