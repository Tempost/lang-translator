mod compiler;

pub use crate::compiler::lexanalysis;

fn main() {
    let lex = lexanalysis::Tokenize::read_file("program.java").unwrap();
    
    lex.characters.for_each(|c|
        println!("{:?}", c)
    )
}
