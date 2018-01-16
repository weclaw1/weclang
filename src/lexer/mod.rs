use std::str::Chars;
use std::iter::Peekable;

use token::{Token, Type};

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer { input: input.chars().peekable() }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn peek_char_eq(&mut self, ch: char) -> bool {
        match self.peek_char() {
            Some(&peek_ch) => peek_ch == ch,
            None => false,
        }
    }

    fn peek_is_alphanumeric(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => ch.is_ascii_alphanumeric() || ch == '_',
            None => false,
        }
    }

    fn peek_is_numeric(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => ch.is_ascii_digit() || ch == '.',
            None => false,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if !c.is_ascii_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn read_operator_or_delimiter(&mut self, ch: char) -> Token {
        match ch {
            '=' => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            ';' => Token::Semicolon,
            ':' => Token::Colon,
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => {
                if self.peek_char_eq('>') {
                    self.read_char();
                    Token::RightArrow
                } else {
                    Token::Minus
                }
            }
            '!' => {
                if self.peek_char_eq('=') {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            _ => Token::Illegal(ch.to_string()),
        }
    }

    fn read_identifier(&mut self, ch: char) -> Token {
        let mut ident = String::new();
        ident.push(ch);

        while self.peek_is_alphanumeric() {
            ident.push(self.read_char().unwrap());
        }

        match ident.as_ref() {
            "fn" => Token::Function,
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            "return" => Token::Return,
            "int" => Token::Type(Type::Integer),
            "float" => Token::Type(Type::Float),
            "bool" => Token::Type(Type::Boolean),
            "string" => Token::Type(Type::String),
            _ => Token::Ident(ident.to_string()),
        }

    }

    fn read_number(&mut self, ch: char) -> Token {
        let mut float_num = false;
        let mut number = String::new();
        number.push(ch);

        while self.peek_is_numeric() {
            let ch = self.read_char().unwrap();
            if ch == '.' {
                float_num = true;
            } 
            number.push(ch);
        }

        if float_num {
            if let Ok(float_num) = number.parse::<f64>() {
                return Token::Float(float_num)
            }
        } else {
            if let Ok(int_num) = number.parse::<i64>() {
                return Token::Integer(int_num)
            }
        }

        Token::Illegal(number)
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.read_char() {
            Some(ch @ _) => {
                if ch.is_ascii_punctuation() {
                    self.read_operator_or_delimiter(ch)
                } else if ch.is_ascii_alphabetic() {
                    self.read_identifier(ch)
                } else if ch.is_ascii_digit() {
                    self.read_number(ch)
                } else {
                    Token::Illegal(ch.to_string())
                } 
            }
            None => Token::EndOfFile,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() { 
        let input = "
            let five = 5;
            let ten = 10;

            let add = fn(x: int, y: int) -> int {
                return x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
        ";
        let tests = vec![
            Token::Let, Token::Ident("five".to_string()), Token::Assign, Token::Integer(5), Token::Semicolon,

            Token::Let, Token::Ident("ten".to_string()), Token::Assign, Token::Integer(10), Token::Semicolon,

            Token::Let, Token::Ident("add".to_string()), Token::Assign, Token::Function, Token::LeftParenthesis, Token::Ident("x".to_string()), Token::Colon, 
            Token::Type(Type::Integer), Token::Comma, Token::Ident("y".to_string()), Token::Colon, Token::Type(Type::Integer), Token::RightParenthesis,
            Token::RightArrow, Token::Type(Type::Integer), Token::LeftBrace,

            Token::Return, Token::Ident("x".to_string()), Token::Plus, Token::Ident("y".to_string()), Token::Semicolon,

            Token::RightBrace, Token::Semicolon,

            Token::Let, Token::Ident("result".to_string()), Token::Assign, Token::Ident("add".to_string()), Token::LeftParenthesis, Token::Ident("five".to_string()),
            Token::Comma, Token::Ident("ten".to_string()), Token::RightParenthesis, Token::Semicolon,

            Token::Bang, Token::Minus, Token::Slash, Token::Asterisk, Token::Integer(5), Token::Semicolon,

            Token::Integer(5), Token::LessThan, Token::Integer(10), Token::GreaterThan, Token::Integer(5), Token::Semicolon,

            Token::If, Token::LeftParenthesis, Token::Integer(5), Token::LessThan, Token::Integer(10), Token::RightParenthesis, Token::LeftBrace,

            Token::Return, Token::Boolean(true), Token::Semicolon,

            Token::RightBrace, Token::Else, Token::LeftBrace,

            Token::Return, Token::Boolean(false), Token::Semicolon,

            Token::RightBrace,

            Token::Integer(10), Token::Equal, Token::Integer(10), Token::Semicolon,

            Token::Integer(10), Token::NotEqual, Token::Integer(9), Token::Semicolon,

        ];
        let mut lexer = Lexer::new(input);
        for test in tests {
            let token = lexer.next_token();
            assert_eq!(token, test);
        }
    }

}