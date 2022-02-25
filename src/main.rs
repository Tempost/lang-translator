mod compiler;

pub use crate::compiler::lexanalysis;

fn main() {
    // NOTE: init state table in main instead?
    let mut lex = lexanalysis::Tokenize::read_file("program.java").unwrap();
    
    println!("{:?}",lex.next());
    println!("{:?}",lex.next());
    println!("{:?}",lex.next());
    println!("{:?}",lex.next());
}
