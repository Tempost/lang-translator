mod compiler;

use crate::compiler::lexanalysis::Tokenize;

fn main() {

    let mut lex = Tokenize::create_scanner("program.java").unwrap();
    lex.create_symbol_table("symbols");

    // while let Some(token) = lex.next() {
    //     let format = format!("Token: {:<10} Symbol: {:?}", token.name, token.class.unwrap());
    //     println!("{}", format);
    // }
}
