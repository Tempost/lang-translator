use std::fs;
use std::io;
use std::iter::Peekable;
use std::vec::IntoIter;

use crate::compiler::fsa::{Fsa, Terminals};

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
    LB,
    RB,
    Comma,
    Semi
}

pub struct Tokenize {
    pub characters: Peekable<IntoIter<char>>
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

fn valid_seperator(c: &char) -> bool {
    let list = ['{', '}', ',', ';'];
    if list.contains(c) {
        return true;
    }
    false
}

// Here we need to use the FSA/Desicion table to cunstruct our tokens
// Each call of next will return a Option<Result<Tokens>> aka a singular valid token
// eventually use box or something else for error handling
impl Iterator for Tokenize {
    type Item = Result<Tokens>;

    fn next(&mut self) -> Option<Self::Item> {

        let mut curr_state = 
        let mut token_string = String::from("");
        let mut character: char;

        // do stuff with character, append to token_string -> check next to see if token ends
        // once we have a valid "token" based on the FSA/DT we will be able to have the
        // associated symbol to append to the symbol table
        loop {
            println!("{:?}", curr_state);
            if let Some(c) = self.characters.next() {
                character = c;
            } else {
                break; // exit loop due to ?EOF?
                // TODO: Error handling
            }

            // TODO: Determine if character is L, D, or other terminal.

            match table[terminal][curr_state] {
                // TODO: Set new state or finish token
            }

        }
        println!("Final Token: {}", token_string);
        None
    }
}

