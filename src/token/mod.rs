#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(String),
    EndOfFile,

    //Literals
    Ident(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Type(Type),

    //Keywords
    If,
    Else,
    Function,
    Let,
    Return,

    //Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    GreaterThan,
    LessThan,
    Equal,
    NotEqual,

    //Delimiters
    Comma,
    Semicolon,
    Colon,

    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,

    RightArrow,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Integer,
    Float,
    Boolean,
    String,
}