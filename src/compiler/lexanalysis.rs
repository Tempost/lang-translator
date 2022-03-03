use std::fs;
use std::io;
use std::iter::Peekable;
use std::vec::IntoIter;

pub type ValidTable = Vec<Vec<i32>>;

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug)]
pub enum Symbols {
    Var,
    NumLit,
    OpenBracket,
    CloseBracket,
    Mop,
    Addop,
    Assign,
    Semi,
    Comma,
    Unknown(String)
}

pub struct Token {
    pub name: String,
    pub symbol: Symbols
}

pub struct Tokenize {
    pub characters: Peekable<IntoIter<char>>,
    fsa: ValidTable,
}

impl Tokenize {
    pub fn create_scanner(filename: &str) -> io::Result<Self> {
        let table = vec![
            vec![1, 3, 5, 6, 7, 8, 9, 10, 11, 12, 0, 16],
            vec![1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![4, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![13, 13, 13, 13, 14, 13, 13, 13, 13, 13, 13, 13],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![14, 14, 14, 14, 15, 14, 14, 14, 14, 14, 14, 14],
            vec![14, 14, 14, 14, 14, 14, 14, 14, 14, 0, 14, 14],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let contents =
            &fs::read_to_string(filename).expect("[ ERROR Something went wrong reading the file]");

        Ok(Tokenize {
            characters: contents.chars().collect::<Vec<_>>().into_iter().peekable(),
            fsa: table,
            })
    }

    pub fn run_symbolizer(&mut self, filename: &str) {
        let mut peek_self = self.peekable();
        while let Some(token) = peek_self.next() {
        }
    }
}

// From input character determine what enum to relate with said character
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

impl Iterator for Tokenize {
    type Item = Token;

    // Parse tokens, using whitespace as our delimiter to denote a final token
    // Each token is constructed based on a input FSA, which was constructed when calling
    // the create_scanner function
    fn next(&mut self) -> Option<Self::Item> {
        let mut token = Token {
            name: String::from(""),
            symbol: Symbols::Unknown(String::from("Empty"))
        };

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

            curr_state = self.fsa[curr_state as usize][terminal as usize];
            match curr_state {
                // Ignoring whitespace and any comment strings
                0 | 14 | 15 => {
                    token.name.clear();
                }

                // Still parsing Letter/Digit
                1 | 3 => {
                    let peeked: &char;
                    if let Some(pc) = self.characters.peek() {
                        peeked = pc;
                    } else {
                        break; // Leave loop if nothing was found when peeking
                    }

                    token.name.push(character);

                    match parse_terminal_enum(peeked).unwrap() {
                        Terminals::Letter => continue,
                        Terminals::Digit => continue,
                        _ => {
                            if terminal == Terminals::Letter {
                                token.symbol = Symbols::Var;
                                break;
                            }

                            if terminal == Terminals::Digit {
                                token.symbol = Symbols::NumLit;
                                break;
                            }
                        },
                    }
                }
                
                // Hit final letter/digit, break, attach correct class and send out token
                2 => {
                    token.symbol = Symbols::Var;
                    break;
                }

                4 => {
                    token.symbol = Symbols::NumLit;
                    break;
                }

                // Single branch from starting state, break and send out the token
                5 => {
                    token.name.push(character);
                    token.symbol = Symbols::OpenBracket;
                    break;
                }

                6 => {
                    token.name.push(character);
                    token.symbol = Symbols::CloseBracket;
                    break;
                }
                
                7 => {
                    token.name.push(character);
                    token.symbol = Symbols::Mop;
                    break;
                }
                
                8 => {
                    token.name.push(character);
                    token.symbol = Symbols::Addop;
                    break;
                }
                
                9 => {
                    token.name.push(character);
                    token.symbol = Symbols::Assign;
                    break;
                }
                
                10 => {
                    token.name.push(character);
                    token.symbol = Symbols::Semi;
                    break;
                }
                
                11 => {
                    token.name.push(character);
                    token.symbol = Symbols::Comma;
                    break;
                }

                13 => {
                    token.name.push(character);
                    token.symbol = Symbols::Mop;
                    break;
                }

                16 => {
                    token.name.push(character);
                    token.symbol = Symbols::Addop;
                    break;
                }

                // Handling comments
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
                        Terminals::Slash => continue,
                        _ => break,
                    }
                }

                // TODO: Error, Some how hit an unreachable state... Replace later when doing
                // proper error handling
                _ => panic!("[ ERROR ] Unreachable state, handle me better"),
            }
        }

        // Send out token wrapped in option. Will return None to detonte end of Iter
        Some(token)
    }
}
