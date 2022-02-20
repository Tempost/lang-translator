use std::fs;
use std::io;
use std::iter::Peekable;
use std::vec::IntoIter;
type Result<T> = std::result::Result<T, String>;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum Tokens {
    Identifier(String),
    Keyword(Keywords),
    Operator(Operators),
    Literal(Literals),
    Seperator(Seperators)
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum Keywords {
    Const(String),
    Var(String)
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum Literals {
    Int(i32)
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum Operators {
    Mop(String),
    AddOp(String),
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum Seperators {
    LB(String),
    RB(String),
    Comma(String),
    Semi(String)
}

struct Tokenize {
    characters: Peekable<IntoIter<char>>
}

impl Tokenize {
    fn to_struct(data: &str) -> Self {
        Tokenize { 
            characters: data.chars().collect::<Vec<_>>()
                .into_iter().peekable()
        }
    }

    fn read_file(filename: &str) -> io::Result<Self> {
        Ok(Self::to_struct(&fs::read_to_string(filename)
            .expect("[ ERROR ] Something went wrong reading the file")))
    }

}

// Here we need to use the FSA/Desicion table to cunstruct our tokens/symbol table
impl Iterator for Tokenize {
    type Item = Result<Tokens>;

    fn next(&mut self) -> Option<Self::Item> {
        let character: char;

        // Loop through, ignoring whitespace, and create tokens
        loop {
            match self.characters.next() {
                Some(c) if c.is_ascii_whitespace() => continue,
                Some(c) => {
                   character = c;
                   println!("{}", character);
                   break;
                }
                None => return None, 
            }
        }
        None
    }
}

fn main() {
    let lex = Tokenize::read_file("program.java").unwrap();
    lex.for_each(|c|
        println!("{:?}", c)
    )
}
