mod compiler;

pub use crate::compiler::lexanalysis;

fn main() {
    // NOTE: init state table in main instead?
    let mut lex = lexanalysis::Tokenize::read_file("program.java").unwrap();

    // Loop through all tokens until the Scanner returns None
    while let Some(token) = lex.next() {
        println!("{}", token.unwrap());
    }
}
