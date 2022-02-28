use std::fs;
use std::io;
use std::vec::IntoIter;

type Result<T> = std::result::Result<T, String>;

type Validtable = [ [i32; 11]; 16];

#[derive(Debug, Clone, Copy)]
pub enum Terminals {
    Letter = 0,
    Digit = 1,
    LBracket = 2,
    RBracket = 3,
    Mop = 4,
    Addop = 5,
    Assignment = 6,
    Semi = 7,
    Comma = 8,
    FSlash = 9,
    Whitespace = 10
}

#[derive(Debug, PartialEq, Eq)]
struct Fsa {
    state_table: Validtable,
}

const TABLE: Fsa =  Fsa {
    state_table: 
   [[ 1,  3,  5,  6,  7,  8,  9, 10, 11, 12,  0],
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
    [14, 14, 14, 14, 14, 14, 14, 14, 14,  0, 14]]
};

pub struct Tokenize {
    pub characters: IntoIter<char>,
}

impl Tokenize {
    pub fn to_struct(data: &str) -> Self {
        Tokenize { 
            characters: data.chars().collect::<Vec<_>>()
                .into_iter(),
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
                // TODO: Error handling
                return None
            }

            // Check what terminal we have
            let terminal: Terminals;
            match character {
                character if character.is_alphabetic() => {
                     terminal = Terminals::Letter;   
                }

                // TODO: Find a better method for finding digits please
                character if character.is_digit(10)  => {
                    terminal = Terminals::Digit;
                }

                character if character.is_whitespace() => {
                    terminal = Terminals::Whitespace;
                }

                '{' => terminal = Terminals::LBracket,
                '}' => terminal = Terminals::RBracket,
                ';' => terminal = Terminals::Semi,
                '+' => terminal = Terminals::Addop,
                '*' => terminal = Terminals::Mop,
                '/' => terminal = Terminals::FSlash,
                ',' => terminal = Terminals::Comma,
                '=' => terminal = Terminals::Assignment,

                _ => break // TODO: Error handling found invalid character
            }
            
            // TODO: Turn self.characters back into a peekable iter and use that to peek at the
            // next character to see if we need to keep making a token or break and report token
            // back to caller
            curr_state = TABLE.state_table[curr_state as usize][terminal as usize];
            match curr_state {
                
                // Ignoring whitespace and any comment strings
                0 | 14 | 15 => {
                    token_string.clear();
                    continue;
                }

                // Still creating a token, go to next character
                1 | 3 | 12 => {
                    token_string.push(character);
                }

                // Hit final character, break and send out the token
                2 | 4  => break,

                // Single branch from starting state, break and send out the token 
                5 | 6 | 7 | 8 | 9 | 10 | 11 | 13  => {
                    token_string.push(character);
                    break;
                }

                // TODO: Error, Some how hit an unreachable state
                _ => panic!("[ ERROR ] Unreachable state, handle me better")
            }

        }

        Some(Ok(token_string))
    }
}
