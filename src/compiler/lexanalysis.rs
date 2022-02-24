use std::fs;
use std::io;
use std::iter::Peekable;
use std::vec::IntoIter;

use crate::compiler::fsa::{Fsa, States};

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
        // TODO: Move this out of the function
        let table =  Fsa::new([
        //   L               D               b
            [States::Letter, States::Digit , States::Start], // State 0
            [States::Letter, States::Letter, States::Finish], // State 1
            [States::Start,  States::Digit,  States::Finish], // State 2
        ],
        [
            String::from("<Identifier>"),
            String::from("<Literal>"),
            String::from("<PlaceHolder>")
        ]);

        let mut curr_state = States::Start;
        let mut token_string = String::from("");
        let mut character: char;

        // do stuff with character, append to token_string -> check next to see if token ends
        // once we have a valid "token" based on the FSA/DT we will be able to have the
        // associated symbol to append to the symbol table
        while curr_state != States::Finish {
            println!("{:?}", curr_state);
            if let Some(c) = self.characters.next() {
                character = c;
            } else {
                break;
            }

            match curr_state {
                /*
                Letter => goto state 1 from state 0, peek, goto state 1 from state 1, peek
                Digit  => goto state 3 from state 0, peek, goto state 1 from state 1, peek, goto state 3 from state 3, peek 
                space  => peek, goto state 0 from state 0, goto state 2 from state 1, goto state 4 from state 3 => append token
                */
                States::Start => continue,
                States::Letter => continue,
                States::Digit => continue,
                _ => continue,
            }
        }
        println!("Final Token: {}", token_string);
        None
    }
}

