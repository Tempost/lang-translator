// use std::io;
// use std::io::prelude::*;

mod compiler;

use crate::compiler::lexanalysis::Tokenize;
use crate::compiler::fsa::Fsa;

fn main() {
    let scanner_table = vec![
        vec![1, 3, 5, 6, 7, 8, 9, 10, 11, 12, 0, 16],
        vec![1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![4, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![13, 13, 13, 13, 14, 13, 13, 13, 13, 13, 13, 13],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![14, 14, 14, 14, 15, 14, 14, 14, 14, 14, 14, 14],
        vec![14, 14, 14, 14, 14, 14, 14, 14, 14, 0, 14, 14],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];


    let scanner_fsa = Fsa::define_table(&scanner_table);
    let mut lex = Tokenize::create_scanner("program.java", &scanner_fsa).unwrap();
    lex.run_symbolizer("program.symbols");
    // let mut stdin = io::stdin();
    // let mut stdout = io::stdout();

    // while let Some(token) = lex.next() {
    //     println!("Token: {} Symbol: {:?}", token.name, token.symbol);
        // write!(stdout, "Press any key to continue to next token...").unwrap();
        // stdout.flush().unwrap();

        // let _ = stdin.read(&mut [0u8]).unwrap();
    // }
}
