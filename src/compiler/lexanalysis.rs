use std::fs;
use std::io;
use std::iter::Peekable;
use std::vec::IntoIter;
type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Identifier(String),
    Keyword(Keywords),
    Operator(Operators),
    Literal(Literals),
    Seperator(Seperators)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keywords {
    Const(String),
    Var(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literals {
    Int(i32)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operators {
    Mop(String),
    AddOp(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Seperators {
    LB(String),
    RB(String),
    Comma(String),
    Semi(String)
}

pub struct Tokenize {
    characters: Peekable<IntoIter<char>>
}

impl Tokenize {
    pub fn to_struct(data: &str) -> Self {
        Tokenize { 
            characters: data.chars().collect::<Vec<_>>()
                .into_iter().peekable()
        }
    }

    pub fn read_file(filename: &str) -> io::Result<Self> {
        Ok(Self::to_struct(&fs::read_to_string(filename)
            .expect("[ ERROR ] Something went wrong reading the file")))
    }

}

// Here we need to use the FSA/Desicion table to cunstruct our tokens/symbol table
impl Iterator for Tokenize {
    type Item = Result<Tokens>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!("Work on this (:");
        // let character: char;

        // // Loop through, ignoring whitespace, and create tokens
        // loop {
        //     match self.characters.next() {
        //         Some(c) if c.is_ascii_whitespace() => continue,
        //         Some(c) => {
        //            character = c;
        //            println!("{}", character);
        //            break;
        //         }
        //         None => return None, 
        //     }
        // }
        // None
    }
}
