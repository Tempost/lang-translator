// use std::io;
// use std::io::prelude::*;

mod compiler;

use crate::compiler::lexanalysis::Tokenize;

fn main() {

    let mut lex = Tokenize::create_scanner("program.java").unwrap();
    // lex.run_symbolizer("program.symbols");
    // let mut stdin = io::stdin();
    // let mut stdout = io::stdout();

    while let Some(token) = lex.next() {
        let format = format!("Token: {:<10} Symbol: {:?}", token.name, token.class);
        println!("{}", format);
        // write!(stdout, "Press any key to continue to next token...").unwrap();
        // stdout.flush().unwrap();

        // let _ = stdin.read(&mut [0u8]).unwrap();
    }
}
