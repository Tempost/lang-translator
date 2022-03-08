use std::fs;
use std::io;
use std::io::Write;
use std::iter::Peekable;
use std::vec::IntoIter;

pub type ValidTable = Vec<Vec<i32>>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Terminals {
    Letter,
    Digit,
    OpenBracket,
    CloseBracket,
    Mult,
    Add,
    Equal,
    Semi,
    Comma,
    Slash,
    Whitespace,
    Minus,
    Unknown
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenClass {
    ReservedWord(ReservedWords),
    Identifier(Identifiers),
    Literal(Literals),
    Delimiter(Delimiters),
    Op(Ops),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReservedWords {
    Class,
    Const,
    Var
}

#[derive(Debug, PartialEq, Eq)]
pub enum Identifiers {
    Identifier
}

#[derive(Debug, PartialEq, Eq)]
pub enum Literals {
    Integer
}


#[derive(Debug, PartialEq, Eq)]
pub enum Delimiters {
    OpenBracket,
    CloseBracket,
    Semi,
    Comma,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Ops {
    Mop,
    Addop,
    Assignment,
}

pub struct Token {
    pub name: String,
    pub class: Option<TokenClass>
}

pub struct Tokenize {
    pub characters: Peekable<IntoIter<char>>,
    fsa: ValidTable,
}

impl Tokenize {
    pub fn create_scanner(filename: &str) -> io::Result<Self> {
        let table = vec![
            vec![1, 3, 5, 6, 7, 8, 9, 10, 11, 12, 0, 16],         // State 0
            vec![1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],             // State 1
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],             // State 2
            vec![4, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 0],             // State 3
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],             // State 4
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],             // State 5
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],             // State 6
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],             // State 7
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],             // State 8
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],             // State 9
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],             // State 10
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],             // State 11
            vec![13, 13, 13, 13, 14, 13, 13, 13, 13, 13, 13, 13], // State 12
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],             // State 13
            vec![14, 14, 14, 14, 15, 14, 14, 14, 14, 14, 14, 14], // State 14
            vec![14, 14, 14, 14, 14, 14, 14, 14, 14, 0, 14, 14],  // State 15
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],             // State 16
        ];

        let contents =
            &fs::read_to_string(filename).expect("[ ERROR ] Something went wrong reading the file]");

        Ok(Tokenize {
            characters: contents.chars().collect::<Vec<_>>().into_iter().peekable(),
            fsa: table,
            })
    }

    // NOTE: Unfinished symbol table construction
    pub fn create_symbol_table(&mut self, filename: &str) {
        let table = vec![
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],              // State 0
            vec![0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0],              // State 1
            vec![0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0],              // State 2
            vec![0, 4, 8, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0],             // State 3
            vec![0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0],              // State 4
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0],              // State 5
            vec![0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0],              // State 6
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 0],              // State 7
            vec![0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0],              // State 8
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 8, 0],              // State 9
            vec![10, 10, 10, 10, 11, 10, 10, 10, 10, 10, 10, 10, 12], // State 10
            vec![10, 10, 10, 10, 11, 10, 10, 10, 10, 10, 10, 10, 12], // State 11
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];             // State 12

        let mut file = fs::File::create(filename).expect("[ ERROR ] Something went wrong creating the file.");

        let mut peek_self = self.peekable();
        let mut curr_state: i32 = 0;
        let mut new_state: i32 = 0;

        while let Some(token) = peek_self.next() {
            let token_index = usize::from(token.class.unwrap());
            let mut symbol_index = 1;
            new_state = table[curr_state as usize][token_index];

            match curr_state {
                0 | 2 | 3 => {},
                1 => {
                    file.write_fmt(format_args!("{:>3} {:>5} {:>10?} {:>5} {:>5} {:>2}", 
                                      symbol_index, token.name, "$program name", "", 0, "CS")).ok();
                    symbol_index += 1;
                }
                _ => panic!("[ ERROR ] Unreachable state, handle me better"),
            }
            curr_state = new_state;
        }
    }
}

impl From<TokenClass> for usize {
    fn from(class: TokenClass) -> usize {
        match class {
            TokenClass::ReservedWord(r) => {
                match r {
                    ReservedWords::Class => 0,
                    ReservedWords::Const => 1,
                    ReservedWords::Var => 2,
                }
            }
            TokenClass::Identifier(i) => {
                match i {
                    Identifiers::Identifier => 3,
                }
            }
            TokenClass::Literal(l) => {
                match l {
                    Literals::Integer => 4,
                }
            }
            TokenClass::Op(o) => {
                match o {
                    Ops::Mop => 7,
                    Ops::Addop => 8,
                    Ops::Assignment => 9,
                }
            }
            TokenClass::Delimiter(d) => {
                match d {
                    Delimiters::OpenBracket => 5,
                    Delimiters::CloseBracket => 6,
                    Delimiters::Semi => 10,
                    Delimiters::Comma => 11,
                }
            }
        }
    }
}

impl From<Terminals> for usize {
    fn from(t: Terminals) -> usize {
        match t {
            Terminals::Letter => 0,
            Terminals::Digit => 1,
            Terminals::OpenBracket => 2,
            Terminals::CloseBracket => 3,
            Terminals::Mult => 4,
            Terminals::Add => 5,
            Terminals::Equal => 6,
            Terminals::Semi => 7,
            Terminals::Comma => 8,
            Terminals::Slash => 9,
            Terminals::Whitespace => 10,
            Terminals::Minus => 11,
            Terminals::Unknown => 12,
        }
    }
}

impl From<&char> for Terminals {
    fn from(ch: &char) -> Terminals {
        match ch {
            c if c.is_alphabetic() => Terminals::Letter,

            c if c.is_digit(10) => Terminals::Digit,

            character if character.is_whitespace() => Terminals::Whitespace,

            '{' => Terminals::OpenBracket,
            '}' => Terminals::CloseBracket,
            ';' => Terminals::Semi,
            '+' => Terminals::Add,
            '*' => Terminals::Mult,
            '/' => Terminals::Slash,
            ',' => Terminals::Comma,
            '=' => Terminals::Equal,
            '-' => Terminals::Minus,

            _ => Terminals::Unknown
        }
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
            class: None 
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
            let terminal: Terminals = Terminals::from(&character);

            curr_state = self.fsa[curr_state as usize][usize::from(terminal)];
            match curr_state {
                // Ignoring whitespace and any comment strings
                0 | 14 | 15 => {
                    token.name.clear();
                }

                1  => {
                    token.name.push(character);

                    // Handling the case where we find a delimiter after a letter
                    let peeked = self.characters.peek().unwrap(); 
                    match Terminals::from(peeked) {
                        Terminals::Letter => continue,
                        Terminals::Digit => continue,
                        _ => {
                            if terminal == Terminals::Letter {
                                token.class = Some(TokenClass::Identifier(Identifiers::Identifier));
                                break;
                            }
                        },
                    }
                }

                3 => {
                    token.name.push(character);

                    // Handling the case where we find a delimiter after a digit
                    let peeked = self.characters.peek().unwrap(); 
                    match Terminals::from(peeked){
                        Terminals::Letter => continue,
                        Terminals::Digit => continue,
                        _ => {
                            if terminal == Terminals::Digit {
                                token.class = Some(TokenClass::Literal(Literals::Integer));
                                break;
                            }
                        },
                    }
                }
                
                // Hit final letter/digit, break, attach correct class and send out token
                2 => {
                    token.class = Some(TokenClass::Identifier(Identifiers::Identifier));
                    break;
                }

                4 => {
                    token.class = Some(TokenClass::Literal(Literals::Integer));
                    break;
                }

                // Single branch from starting state, break and send out the token
                5 => {
                    token.name.push(character);
                    token.class = Some(TokenClass::Delimiter(Delimiters::OpenBracket));
                    break;
                }

                6 => {
                    token.name.push(character);
                    token.class = Some(TokenClass::Delimiter(Delimiters::CloseBracket));
                    break;
                }
                
                7 => {
                    token.name.push(character);
                    token.class = Some(TokenClass::Op(Ops::Mop));
                    break;
                }
                
                8 => {
                    token.name.push(character);
                    token.class = Some(TokenClass::Op(Ops::Addop));
                    break;
                }
                
                9 => {
                    token.name.push(character);
                    token.class = Some(TokenClass::Op(Ops::Assignment));
                    break;
                }
                
                10 => {
                    token.name.push(character);
                    token.class = Some(TokenClass::Delimiter(Delimiters::Semi));
                    break;
                }
                
                11 => {
                    token.name.push(character);
                    token.class = Some(TokenClass::Delimiter(Delimiters::Comma));
                    break;
                }

                13 => {
                    token.name.push(character);
                    token.class = Some(TokenClass::Op(Ops::Mop));
                    break;
                }

                16 => {
                    token.name.push(character);
                    token.class = Some(TokenClass::Op(Ops::Addop));
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
                    match Terminals::from(peeked) {
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

        // Checking if our current token is one of our reserved words and changing its token class
        // to match before sending the token out
        match token.name.as_str() {
                "CLASS" => token.class = Some(TokenClass::ReservedWord(ReservedWords::Class)),
                "CONST" => token.class = Some(TokenClass::ReservedWord(ReservedWords::Const)),
                "VAR" => token.class = Some(TokenClass::ReservedWord(ReservedWords::Var)),
                _ => {},
        }

        // Send out token wrapped in option. Will return None to detonte end of Iter
        Some(token)
    }
}
