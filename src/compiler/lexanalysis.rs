use std::fs;
use std::io;
use std::vec::IntoIter;
use std::iter::Peekable;

type Result<T> = std::result::Result<T, String>;

type Validtable = [ [i32; 12]; 17];

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
    Whitespace = 10,
    Subtraction = 11
}

#[derive(Debug, PartialEq, Eq)]
struct Fsa {
    state_table: Validtable,
}

const TABLE: Fsa =  Fsa {
    state_table: 
   [[ 1,  3,  5,  6,  7,  8,  9, 10, 11, 12,  0, 16],
    [ 1,  1,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2],
    [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 4,  3,  4,  4,  4,  4,  4,  4,  4,  4,  4,  0],
    [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [13, 13, 13, 13, 14, 13, 13, 13, 13, 13, 13, 13],
    [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
    [14, 14, 14, 14, 15, 14, 14, 14, 14, 14, 14, 14],
    [14, 14, 14, 14, 14, 14, 14, 14, 14,  0, 14, 14],
    [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0]]
};

pub struct Tokenize {
    pub characters: Peekable<IntoIter<char>>,
}

impl Tokenize {
    pub fn to_struct(data: &str) -> Self {
        Tokenize { 
            characters: data.chars().collect::<Vec<_>>()
                .into_iter().peekable(),
        }
    }

    pub fn read_file(filename: &str) -> io::Result<Self> {
        Ok(Self::to_struct(&fs::read_to_string(filename)
            .expect("[ ERROR ] Something went wrong reading the file")))
    }
}

fn get_terminal_enum(c: &char) -> Option<Terminals> {
    match c {
            c if c.is_alphabetic() => {
                Some(Terminals::Letter)
            }

            // TODO: Find a better method for finding digits please
            c if c.is_digit(10)  => {
                Some(Terminals::Digit)
            }

            character if character.is_whitespace() => {
                Some(Terminals::Whitespace)
            }

            '{' => Some(Terminals::LBracket),
            '}' => Some(Terminals::RBracket),
            ';' => Some(Terminals::Semi),
            '+' => Some(Terminals::Addop),
            '*' => Some(Terminals::Mop),
            '/' => Some(Terminals::FSlash),
            ',' => Some(Terminals::Comma),
            '=' => Some(Terminals::Assignment),
            '-' => Some(Terminals::Subtraction),

            _ => None // TODO: Error handling found invalid character
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

            // Handle Option return for next() method and safely unwrap
            if let Some(c) = self.characters.next() {
                character = c;
            } else {
                // TODO: Error handling
                return None
            }

            // Check what terminal we have
            let terminal: Terminals;
            if let Some(t) = get_terminal_enum(&character) {
                terminal = t;
            } else {
                // TODO: Better error handling. Recover and keep parsing but report error
                panic!("[ ERROR ] Hit unrecognized token.")
            }
                        
            curr_state = TABLE.state_table[curr_state as usize][terminal as usize];
            match curr_state {
                
                // Ignoring whitespace and any comment strings
                0 | 14 | 15 => {
                    token_string.clear();
                    continue;
                }

                // Hit final character, break and send out the token
                2 | 4  => break,

                // Single branch from starting state, break and send out the token 
                5 | 6 | 7 | 8 | 9 | 10 | 11 | 13 | 16  => {
                    token_string.push(character);
                    break;
                }

                12 => {
                    let peeked: &char;
                    if let Some(pc) = self.characters.peek() {
                        peeked = pc;
                    } else {
                        // TODO: Do something if nothing peekable?
                        break
                    }

                    // TODO: This is were we will handle division later on.
                   match get_terminal_enum(peeked).unwrap() {
                       Terminals::Mop => continue,
                       _ => break
                   }

                }

                // Still creating a token, go to next character
                1 | 3 => {
                    let peeked: &char;
                    if let Some(pc) = self.characters.peek() {
                        peeked = pc;
                    } else {
                        // TODO: Do something if nothing peekable?
                        break
                    }
                    token_string.push(character);

                    // If the return value is anything but a letter or digit break and report
                    // final token
                    match get_terminal_enum(peeked).unwrap() {
                        Terminals::Letter => continue, 
                        Terminals::Digit  => continue,
                        _ => break
                    }
                }

                // TODO: Error, Some how hit an unreachable state
                _ => panic!("[ ERROR ] Unreachable state, handle me better")
            }

        }

        Some(Ok(token_string))
    }
}
