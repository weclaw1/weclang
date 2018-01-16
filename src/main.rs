#![feature(ascii_ctype)]

mod lexer;
mod token;
mod repl;

fn main() {
    println!("Hello, this is the Weclang programming language!");
    println!("Feel free to type commands!");
    repl::start();
}
