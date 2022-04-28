use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::iter::Peekable;
use std::path::Path;
use std::vec::IntoIter;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Terminal {
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
    Unknown,
    OpenParan,
    CloseParan,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenClass {
    Identifier,
    ReservedWord,
    Literal,
    Delimiter,
    Op,
    Program,
    Block,
    ConstDefPart,
    ConstList,
    VariableDefPart,
    VarList,
    ProcedureDefPart,
    Statment,
    SimpleStatement,
    CallStatement,
    ParamList,
    IdentList,
    CompoundStatement,
    StatmentList,
    IfStatement,
    WhileStatement,
    BoolExp,
    RelationOp,
    Expression,
    AddOp,
    Term,
    Mop,
    Fac,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub name: String,
    pub class: TokenClass,
}

impl From<&str> for TokenClass {
    fn from(class: &str) -> Self {
        match class {
            "Identifier" => TokenClass::Identifier,
            "ReservedWord" => TokenClass::ReservedWord,
            "Literal" => TokenClass::Literal,
            "Delimiter" => TokenClass::Delimiter,
            "Op" => TokenClass::Op,
            "RelationOp" => TokenClass::RelationOp,
            "Mop" => TokenClass::Mop,
            e => panic!("[ Error ] Could not parse: {}", e),
        }
    }
}

pub struct Tokenize {
    pub characters: Peekable<IntoIter<char>>,
}

const RESERVED_WORDS: [&str; 12] = [
    "CONST",
    "IF",
    "VAR",
    "THEN",
    "PROCEDURE",
    "GET",
    "PUT",
    "WHILE",
    "CALL",
    "DO",
    "ODD",
    "CLASS",
];

fn debug_print_kek(curr_state: &usize, name: &str, flag: bool) {
    let added: &str = if flag { "ADDED" } else { "" };

    println!(
        "State: {} Found: {} {}",
        curr_state,
        format_args!("{:<7}", name),
        added
    )
}

impl Token {
    pub fn empty() -> Self {
        Token {
            name: String::from("Empty"),
            class: TokenClass::Unknown,
        }
    }

    pub fn terminator() -> Self {
        Token {
            name: String::from("Terminator"),
            class: TokenClass::Delimiter,
        }
    }

    pub fn temp_gen(id: i32) -> Self {
        let string = String::from("temp") + &id.to_string();
        Token {
            name: string,
            class: TokenClass::Identifier,
        }
    }
}

impl Tokenize {
    pub fn create_scanner(filename: &str) -> io::Result<Self> {
        let contents = &fs::read_to_string(filename)
            .expect("[ ERROR ] Something went wrong reading the file]");

        Ok(Tokenize {
            characters: contents.chars().collect::<Vec<_>>().into_iter().peekable(),
        })
    }

    // NOTE: Unfinished symbol table construction
    pub fn create_symbol_table(&mut self, filename: &str) {
        let mut file =
            fs::File::create(filename).expect("[ ERROR ] Something went wrong creating the file.");

        // Make our token iterator peekable
        let mut curr_state: usize = 0;
        let mut goto_state: usize;
        let mut addr: u32 = 0;

        file.write_fmt(format_args!(
            "{:<6} {:<10} {:<5} {:<7} {}\n",
            "Symbol", "Type", "Value", "Address", "Segment"
        ))
        .ok();

        while let Some(token) = self.next() {
            goto_state = Tokenize::table_lookup(
                curr_state,
                usize::from(token.class),
                "fsa_tables/symbol_fsa",
            );

            match goto_state {
                0 => continue,
                1 => {
                    Tokenize::token_to_table(&mut file, &token.name, "Literal", &addr);

                    addr += 2;
                    debug_print_kek(&curr_state, &token.name, true)
                }

                2 => {
                    Tokenize::token_to_table(&mut file, &token.name, "Identifier", &addr);

                    addr += 2;
                    debug_print_kek(&curr_state, &token.name, true)
                }

                3 => break,

                _ => panic!("[ ERROR ] Unreachable state, handle me better"),
            }
            curr_state = goto_state;
        }
    }

    fn token_to_table(mut file: &File, name: &str, class: &str, addr: &u32) {
        file.write_fmt(format_args!(
            "{:<6} {:<10} {:<5} {:<7} {}\n",
            name, class, "", addr, "DS"
        ))
        .ok();
    }

    pub fn token_to_file(token: Token) {
        // let file_res: io::Result<File>;
        let path = Path::new("tokens");
        if !path.exists() {
            File::create("tokens").expect("[ Error ] Something went wrong creating file.");
        }

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("tokens")
            .unwrap();

        if let Err(e) = writeln!(file, "{} {:?}", token.name, token.class) {
            eprintln!("{}, could not write to file.", e);
        }
    }

    // Using a predefined state table located in a file to perform row col look up
    // determining our current state
    fn table_lookup(state: usize, col: usize, fsa: &str) -> usize {
        let table = fs::File::open("src/compiler/".to_owned() + fsa)
            .expect("[ ERROR ] Something went wrong reading the file.");

        let file_reader = io::BufReader::new(table).lines().nth(state).unwrap();
        if let Ok(line) = file_reader {
            return line
                .split(' ')
                .collect::<Vec<&str>>()
                .into_iter()
                .nth(col)
                .unwrap()
                .parse::<usize>()
                .unwrap();
        }
        999
    }
}

// NOTE: The great thing about the From<T> for U trait is that we get the opposite type conversion "for
// free" as well. IE Into<U> for T
impl From<TokenClass> for usize {
    fn from(class: TokenClass) -> usize {
        match class {
            TokenClass::Literal => 0,
            TokenClass::Identifier | TokenClass::ReservedWord => 1,
            TokenClass::Op => 2,
            TokenClass::Delimiter => 3,
            TokenClass::Unknown => panic!("[ Error ] Cannot index Unknown Token Class."),
            _ => panic!("[ Error ] Cannot index using Non-Terminal classes."),
        }
    }
}

impl From<Terminal> for usize {
    fn from(t: Terminal) -> usize {
        match t {
            Terminal::Letter => 0,
            Terminal::Digit => 1,
            Terminal::OpenBracket => 2,
            Terminal::CloseBracket => 3,
            Terminal::Mult => 4,
            Terminal::Add => 5,
            Terminal::Equal => 6,
            Terminal::Semi => 7,
            Terminal::Comma => 8,
            Terminal::Slash => 9,
            Terminal::Whitespace => 10,
            Terminal::Minus => 11,
            Terminal::OpenParan => 12,
            Terminal::CloseParan => 13,
            Terminal::Unknown => 999,
        }
    }
}

impl From<&char> for Terminal {
    fn from(ch: &char) -> Terminal {
        match ch {
            c if c.is_alphabetic() => Terminal::Letter,

            c if c.is_digit(10) => Terminal::Digit,

            character if character.is_whitespace() => Terminal::Whitespace,

            '{' => Terminal::OpenBracket,
            '}' => Terminal::CloseBracket,
            ';' => Terminal::Semi,
            '+' => Terminal::Add,
            '*' => Terminal::Mult,
            '/' => Terminal::Slash,
            ',' => Terminal::Comma,
            '=' => Terminal::Equal,
            '-' => Terminal::Minus,
            '(' => Terminal::OpenParan,
            ')' => Terminal::CloseParan,

            _ => Terminal::Unknown,
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
            class: TokenClass::Unknown,
        };

        let mut curr_state: usize = 0;

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
            let terminal = Terminal::from(&character);

            curr_state =
                Tokenize::table_lookup(curr_state, usize::from(terminal), "fsa_tables/scanner_fsa");
            match curr_state {
                // Ignoring whitespace and any comment strings
                0 | 14 | 15 => {
                    token.name.clear();
                }

                1 => {
                    token.name.push(character);

                    // Handling the case where we find a delimiter after a letter
                    let peeked = self.characters.peek().unwrap();
                    match Terminal::from(peeked) {
                        Terminal::Letter => continue,
                        Terminal::Digit => continue,
                        _ => {
                            if terminal == Terminal::Letter {
                                token.class = TokenClass::Identifier;
                                break;
                            }
                        }
                    }
                }

                3 => {
                    token.name.push(character);

                    // Handling the case where we find a delimiter after a digit
                    let peeked = self.characters.peek().unwrap();
                    match Terminal::from(peeked) {
                        Terminal::Letter => continue,
                        Terminal::Digit => continue,
                        _ => {
                            if terminal == Terminal::Digit {
                                token.class = TokenClass::Literal;
                                break;
                            }
                        }
                    }
                }

                // Hit final letter/digit, break, attach correct class and send out token
                2 => {
                    token.class = TokenClass::Identifier;
                    break;
                }

                4 => {
                    token.class = TokenClass::Literal;
                    break;
                }

                // Single branch from starting state, break and send out the token
                5 => {
                    token.name.push(character);
                    token.class = TokenClass::Delimiter;
                    break;
                }

                6 => {
                    token.name.push(character);
                    token.class = TokenClass::Delimiter;
                    break;
                }

                7 => {
                    token.name.push(character);
                    token.class = TokenClass::Op;
                    break;
                }

                8 => {
                    token.name.push(character);
                    token.class = TokenClass::Op;
                    break;
                }

                9 => {
                    token.name.push(character);
                    token.class = TokenClass::Op;
                    break;
                }

                10 => {
                    token.name.push(character);
                    token.class = TokenClass::Delimiter;
                    break;
                }

                11 => {
                    token.name.push(character);
                    token.class = TokenClass::Delimiter;
                    break;
                }

                13 => {
                    token.name.push(character);
                    token.class = TokenClass::Op;
                    break;
                }

                16 => {
                    token.name.push(character);
                    token.class = TokenClass::Op;
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
                    match Terminal::from(peeked) {
                        Terminal::Mult => continue,
                        Terminal::Slash => continue,
                        Terminal::Whitespace => {
                            token.name.push(character);
                            token.class = TokenClass::Op;
                            break;
                        }
                        _ => break,
                    }
                }

                // TODO: Error, Some how hit an unreachable state... Replace later when doing
                // proper error handling
                _ => panic!("[ ERROR ] Unreachable state, handle me better"),
            }
        }

        if RESERVED_WORDS.contains(&token.name.as_str()) {
            token.class = TokenClass::ReservedWord;
        }

        // Send out token wrapped in option. Will return None to detonte end of Iter
        Some(token)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sym_table() {
        let mut lex = Tokenize::create_scanner("test2.java").unwrap();
        lex.create_symbol_table("symbols");
    }

    #[test]
    fn lex_tokens() {
        let mut lex = Tokenize::create_scanner("test5.java").unwrap();
        while let Some(token) = lex.next() {
            println!("{:?}", token);
        }
    }
}
