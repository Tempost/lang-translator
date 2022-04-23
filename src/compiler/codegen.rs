use std::fmt;
use std::fs::{ self, File, OpenOptions };
use std::io::{self, BufRead, Write, BufReader};
use std::path::Path;

use crate::compiler::syntax::QuadList;

type Result<'a, T> = std::result::Result<T, GeneratorErr<'a>>;
type AsmSnippet = Vec<String>;

trait Snippet {
    fn new(quads: QuadList) -> Self;
}

#[derive(Debug, PartialEq, Eq)]
pub struct GeneratorErr<'a>(&'a str, AsmSnippet);

struct Generator {
    assembly: Vec<AsmSnippet>,
    quads: QuadList,
    asm_file: File,
}

impl<'a> fmt::Display for GeneratorErr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ Error ] {} : {:?}", self.0, self.1)
    }
}

impl Snippet for Generator {
    fn new(quads: QuadList) -> Self {
        if Path::new("code.asm").exists() {
            fs::remove_file("code.asm").unwrap();
        }

        let file = OpenOptions::new().create_new(true)
            .write(true)
            .open("code.asm")
            .unwrap();

        Generator { 
            assembly: Vec::new(),
            quads,
            asm_file: file
        }
    }
}

impl Generator {
    fn consume_quads(&mut self, file: &str) -> Result<AsmSnippet> {
        let snippet: AsmSnippet = Vec::new();
        Ok(snippet)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::compiler::syntax::Syntax;
    use std::any::type_name;

    #[test]
    fn new_file() {

        fn type_of<T>(_: T) -> &'static str {
            type_name::<T>()
        }

        let file = File::open("code.asm").unwrap();
        let gen = Generator::new(Vec::new()); 
        assert_eq!(type_of(file), type_of(gen.asm_file));
    }

    #[test]
    fn getting_data() {
        let mut syn = Syntax::new("test2.java", true);
        syn.complete_analysis();
        syn.consume_polish().unwrap();
        let gen = Generator::new(syn.quads);
        gen.quads.iter().for_each(|x| println!("{}", x));
    }
}
