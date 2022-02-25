use std::fs;
use std::io;
use std::iter::Peekable;
use std::vec::IntoIter;

use crate::compiler::fsa::{Fsa, Terminals};

type Result<T> = std::result::Result<T, String>;


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
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let table =  Fsa::new([
            [ 1,  3,  5,  6,  7,  8,  9, 10, 11, 12,  0],
            [ 1,  1,  2,  2,  2,  2,  2,  2,  2,  2,  2],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [ 4,  3,  4,  4,  4,  4,  4,  4,  4,  4,  4],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [13, 13, 13, 13, 14, 13, 13, 13, 13, 13, 13],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [14, 14, 14, 14, 15, 14, 14, 14, 14, 14, 14],
            [14, 14, 14, 14, 14, 14, 14, 14, 14,  0, 14]
        ]);

        let mut token_string = String::from("");

        // do stuff with character, append to token_string -> check next to see if token ends
        // once we have a valid "token" based on the FSA/DT we will be able to have the
        // associated symbol to append to the symbol table
        let mut curr_state: i32 = 0;
        loop {
            let character: char;

            // Handle Option<> and safely unwrap
            if let Some(c) = self.characters.next() {
                character = c;
            } else {
                break; // exit loop due to ?EOF?
                // TODO: Error handling
            }

            // Check what terminal we have
            let terminal: Terminals;
            match character {
                character if character.is_alphabetic() => {
                     terminal = Terminals::Letter;   
                }

                character if character.is_numeric() => {
                    terminal = Terminals::Digit;
                }

                '{' => terminal = Terminals::LBracket,
                '}' => terminal = Terminals::RBracket,
                ';' => terminal = Terminals::Semi,
                '+' => terminal = Terminals::Addop,
                '*' => terminal = Terminals::Mop,
                '/' => terminal = Terminals::FSlash,
                ',' => terminal = Terminals::Comma,
                '=' => terminal = Terminals::Assignment,

                character if character.is_whitespace() => {
                    terminal = Terminals::Whitespace;
                }

                _ => break // TODO: Error handling found invalid character

            }
            
            curr_state = table.state_table[curr_state as usize][terminal as usize];
            match curr_state {
                
                // Whitespace
                0 => {
                    curr_state = table.state_table[curr_state as usize][terminal as usize];
                }

                1 => {
                    token_string.push(character);
                    curr_state = table.state_table[curr_state as usize][terminal as usize];
                }

                3 => {
                    token_string.push(character);
                    curr_state = table.state_table[curr_state as usize][terminal as usize];
                }

                12 => {
                    token_string.push(character);
                    curr_state = table.state_table[curr_state as usize][terminal as usize];
                }

                14 => {
                    token_string.push(character);
                    curr_state = table.state_table[curr_state as usize][terminal as usize];
                }

                15 => {
                    token_string.push(character);
                    curr_state = table.state_table[curr_state as usize][terminal as usize];
                }

                2  => break,
                4  => break,
                5  => break,
                6  => break,
                7  => break,
                8  => break,
                9  => break,
                10 => break,
                11 => break,
                13 => break,

                
                // TODO: Error, Some how hit an unreachable state
                _ => panic!("[ ERROR ] Unreachable state, handle me better")
            }
            println!("{:?}", terminal);

        }
        println!("Final Token: {}", token_string);
        None
    }
}
