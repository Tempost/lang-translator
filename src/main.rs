use std::io;
use std::io::prelude::*;

mod compiler;

pub use crate::compiler::lexanalysis;

fn main() {
    // NOTE: init state table in main instead?
    let mut lex = lexanalysis::Tokenize::read_file("program.java").unwrap();

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // Loop through all tokens until the Scanner returns None
    while let Some(token) = lex.next() {
        println!("Token: {}", token.unwrap());
        write!(stdout, "Press any key to continue to next token...").unwrap();
        stdout.flush().unwrap();

        let _ = stdin.read(&mut [0u8]).unwrap();
    }
}
