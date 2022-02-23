mod compiler;

pub use crate::compiler::lexanalysis;

fn main() {
    let mut lex = lexanalysis::Tokenize::read_file("program.java").unwrap();
    
    println!("{:?}",lex.next());
    println!("{:?}",lex.next());
    println!("{:?}",lex.next());
}
