use std::fs;
use std::io;
use std::iter::Peekable;
use std::vec::IntoIter;

use crate::compiler::fsa::Fsa;

// TODO: Add more terminals
#[derive(Debug)]
pub enum Terminals {
    Letter = 0,
    Digit = 1,
    OpenBracket = 2,
    CloseBracket = 3,
    Mult = 4,
    Add = 5,
    Equal = 6,
    Semi = 7,
    Comma = 8,
    Slash = 9,
    Whitespace = 10,
    Minus = 11,
}

pub struct Tokenize<'a> {
    pub characters: Peekable<IntoIter<char>>,
    pub table: &'a Fsa<'a>,
}

struct TokenList {
    tokens: Peekable<IntoIter<String>>
}

const RESERVED_WORDS: [&str; 10] = ["CONST", "IF", "VAR", "THEN", "PROCEDURE", "WHILE", "CALL", "DO", "ODD", "CLASS"];
const SYMBOLS: [&str; 14] = ["$Class", "$Program Name", "$Openbrace", "$Closebrace", "$Const", "Constvar", "$=", "Numeric Literal", "$Semi", "$VarDeclaration", "Var", "$Comma", "$Addop", "$Mop"];
// const CHAR_DELIMITERS: [char; 13] = ['=', '.', ';', '+', '-', '*', '/', '(', ')', '<', '>', '{', '}'];
// const STR_DELIMITERS: [&str; 6] = ["==", ">=", "<=", "!=", "/*", "*/"];

impl<'a> Tokenize<'a> {
    pub fn create_scanner(filename: &str, table: &'a Fsa) -> io::Result<Self> {
        let contents =
            &fs::read_to_string(filename).expect("[ ERROR Something went wrong reading the file]");

        Ok(Tokenize {
            characters: contents.chars().collect::<Vec<_>>().into_iter().peekable(),
            table,
            })
    }

    pub fn create_symbol_table(&mut self, filename: &str) {
        let mut list: TokenList;
        while let Some(token) = self.next() {
            let test = RESERVED_WORDS.into_iter().filter(|&s| s == token).collect::<Vec<_>>();
            println!("{:?}", test);
        }
    }
}

fn parse_terminal_enum(c: &char) -> Option<Terminals> {
    match c {
        c if c.is_alphabetic() => Some(Terminals::Letter),

        c if c.is_digit(10) => Some(Terminals::Digit),

        character if character.is_whitespace() => Some(Terminals::Whitespace),

        '{' => Some(Terminals::OpenBracket),
        '}' => Some(Terminals::CloseBracket),
        ';' => Some(Terminals::Semi),
        '+' => Some(Terminals::Add),
        '*' => Some(Terminals::Mult),
        '/' => Some(Terminals::Slash),
        ',' => Some(Terminals::Comma),
        '=' => Some(Terminals::Equal),
        '-' => Some(Terminals::Minus),

        _ => None, // TODO: Error handling found invalid terminal
    }
}

impl<'a> Iterator for Tokenize<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token = String::from("");
        let mut curr_state: i32 = 0;

        loop {
            let character: char;

            // Handle Option return for next() method and safely unwrap
            if let Some(c) = self.characters.next() {
                character = c;
            } else {
                // TODO: Error handling
                return None;
            }

            // Check what terminal we have
            let terminal: Terminals;
            if let Some(t) = parse_terminal_enum(&character) {
                terminal = t;
            } else {
                // TODO: Better error handling. Recover and keep parsing but report error
                panic!("[ ERROR ] Hit unrecognized terminal.")
            }

            curr_state = self.table.table[curr_state as usize][terminal as usize];
            match curr_state {
                // Ignoring whitespace and any comment strings
                0 | 14 | 15 => {
                    token.clear();
                    continue;
                }

                // Hit final character, break and send out the token
                2 | 4 => break,

                // Single branch from starting state, break and send out the token
                5 | 6 | 7 | 8 | 9 | 10 | 11 | 13 | 16 => {
                    token.push(character);
                    break;
                }

                12 => {
                    let peeked: &char;
                    if let Some(pc) = self.characters.peek() {
                        peeked = pc;
                    } else {
                        break; // Leave loop if nothing was found when peeking
                    }

                    // TODO: This is where we will handle division later on.
                    match parse_terminal_enum(peeked).unwrap() {
                        Terminals::Mult => continue,
                        _ => break,
                    }
                }

                // Still creating a token, go to next character
                1 | 3 => {
                    let peeked: &char;
                    if let Some(pc) = self.characters.peek() {
                        peeked = pc;
                    } else {
                        break; // Leave loop if nothing was found when peeking
                    }
                    token.push(character);

                    match parse_terminal_enum(peeked).unwrap() {
                        Terminals::Letter => continue,
                        Terminals::Digit => continue,
                        Terminals::Whitespace => continue,
                        _ => break,
                    }
                }

                // TODO: Error, Some how hit an unreachable state... Replace later when doing
                // proper error handling
                _ => panic!("[ ERROR ] Unreachable state, handle me better"),
            }
        }

        Some(token)
    }
}
