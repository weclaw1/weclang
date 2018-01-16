use std::io::{self, Write};

use lexer::Lexer;
use token::Token;

const PROMPT: &'static str = "=> ";

pub fn start() {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut lexer = Lexer::new(&input);
        loop {
            let token = lexer.next_token();
            if token != Token::EndOfFile {
                println!("{:?}", token);
            } else {
                break;
            }
        }
    }
}