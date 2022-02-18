use std::fs;
use std::io;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, Clone, PartialEq)]
enum Tokens {
    Identifier(String),
    Keyword(Keywords),
    Operator(Operators),
    Literal(Literals),
    Seperator(Seperators)
}

#[derive(Debug, Clone, PartialEq)]
enum Keywords {
    Const(String),
    Var(String)
}

#[derive(Debug, Clone, PartialEq)]
enum Literals {
    Integer(i32)
}

#[derive(Debug, Clone, PartialEq)]
enum Operators {
    Mop(String),
    AddOp(String),
}

#[derive(Debug, Clone, PartialEq)]
enum Seperators {
    LB(String),
    RB(String),
    Comma(String),
    Semi(String)
}

struct Tokenize {
    character: Peekable<IntoIter<char>>
}

impl Tokenize {
    fn to_struct(data: &str) -> Self {
        Tokenize { 
            character: data.chars().collect::<Vec<_>>()
                .into_iter().peekable()
        }
    }

    fn read_file(filename: &str) -> io::Result<Self> {
        Ok(Self::to_struct(&fs::read_to_string(filename)
                           .expect("[ ERROR ] Something went wrong reading the file")))
    }

}
type Result<T> = std::result::Result<T, String>;

impl Iterator for Tokenize {
    type Item = Result<Tokens>;

    fn next(&mut self) -> Option<Self::Item> {
        // Loop through, ignoring whitespace, and create tokens
        unimplemented!()
    }
}

fn main() {
}
