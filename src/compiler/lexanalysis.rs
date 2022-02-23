use std::fs;
use std::io;
use std::iter::Peekable;
use std::vec::IntoIter;

use crate::compiler::fsa::Fsa;

type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Identifier(String),
    Keyword(Keywords),
    Operator(Operators),
    Literal(Literals),
    Seperator(Seperators)
}

// Required for later? to futher identify a symbol
#[derive(Debug, Clone, PartialEq)]
pub enum Keywords {
    Const(String),
    Var(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literals {
    Int(i32)
}

// Required for later? to futher identify a symbol
#[derive(Debug, Clone, PartialEq)]
pub enum Operators {
    Mop(String),
    AddOp(String),
}

// Required for later? to futher identify a symbol
#[derive(Debug, Clone, PartialEq)]
pub enum Seperators {
    LB(String),
    RB(String),
    Comma(String),
    Semi(String)
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

// Here we need to use the FSA/Desicion table to cunstruct our tokens
// Each call of next will return a Option<Result<Tokens>> aka a singular valid token
// eventually use box or something else for error handling
impl Iterator for Tokenize {
    type Item = Result<Tokens>;

    fn next(&mut self) -> Option<Self::Item> {
        // NOTE: what if instead of [0, 1, 2, 3, 4] i used my States enum?
        // TODO: Move this out of the function
        let table =  Fsa::new([
        //   L  D  b
            [1, 3, 0], // State 0 -> Start
            [1, 1, 2], // State 1
            [0, 0, 0], // State 2 <Identifier> -> Finish
            [0, 3, 4], // State 3
            [0, 0, 0]  // State 4 <Literal> -> Finish
        ],
        [
            String::from("<Identifier>"),
            String::from("<Literal>"),
            String::from("<PlaceHolder>")
        ]);

        let mut curr_state = States::Start;
        let mut token_string = String::from("");

        // do stuff with character, append to token_string -> check next to see if token ends
        // once we have a valid "token" based on the FSA/DT we will be able to have the
        // associated symbol to append to the symbol table
        loop {
            println!("{:?}", curr_state);
            match self.characters.next() {
                /*
                Letter => goto state 1 from state 0, peek, goto state 1 from state 1, peek
                Digit  => goto state 3 from state 0, peek, goto state 1 from state 1, peek, goto state 3 from state 3, peek 
                space  => peek, goto state 0 from state 0, goto state 2 from state 1, goto state 4 from state 3 => append token
                */
                Some(c) if c.is_whitespace() => break,

                Some(c) if c.is_alphabetic() => {
                    curr_state = States::Letter;
                    token_string.push(c);
                }

                Some(c) if c.is_numeric() => {
                    token_string.push(c);
                    if curr_state.eq(&States::Letter) {
                        curr_state = States::LetterDigit;
                    }
                    else {
                        curr_state = States::Digit; 
                    }
                }

                // Add a match for Seperators

                // do something with whitespace
                _ => unimplemented!()
            }
        }
        println!("Final Token: {}", token_string);
        None
    }
}
