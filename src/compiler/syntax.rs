use std::error::Error;
use std::fmt;

// Take tokens from lex portion of the code
use crate::compiler::lexical::{Token, TokenClass, Tokenize};
use crate::compiler::precedence::{Precedence, GrammarTable };

type TokenList = Vec<Token>;
type TokenListRef<'a> = Vec<&'a Token>;

type Result<'a, T> = std::result::Result<T, SyntaxError<'a>>;

enum SyntaxClass {
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
    AddoOp,
    Term,
    Mop,
    Fac,
}

pub struct Syntax {
    tokens: TokenList,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SyntaxError<'a>(&'a str, &'a TokenClass);

impl<'a> Error for SyntaxError<'a> {}

impl<'a> fmt::Display for SyntaxError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Syntax Error! at token name: {} class: {:?}",
            self.0, self.1
        )
    }
}

const RESERVED_WORDS: [&str; 10] = [
    "CONST",
    "IF",
    "VAR",
    "THEN",
    "PROCEDURE",
    "WHILE",
    "CALL",
    "DO",
    "ODD",
    "CLASS",
];

impl Syntax {
    fn new() -> Self {
        Syntax { tokens: Vec::new() }
    }

    pub fn pda_stack_from_file(&mut self, file: &str) {
        // Parse name -- TokenClass into token struct, push into TokenList
        unimplemented!();
    }

    pub fn pda_stack_from_memory(&mut self, file: &str) {
        let mut lex = Tokenize::create_scanner(file).unwrap();
        while let Some(token) = lex.next() {
            self.tokens.push(token);
        }
    }

    pub fn complete_analysis(&self) -> Result<()> {
        // Err(SyntaxError(item.name.to_string(), &item.class.as_ref().unwrap()));
        let mut pda: Vec<&Token> = Vec::new();
        let grammar_rules = GrammarTable::new("table");

        // push token into PDA stack until a operator or reserved word is hit
        // Compare last operator/RW with next from the token stack to determine precendece
        // Determine what to do based on output of the compare, IE contuine pushing to the PDA
        // stack, or reduce to a new handle(syntax class). Contuine until no more tokens

        let mut iter = self.tokens.iter().peekable();
        let mut curr_index = 0;
        let mut prev_op_index = 0;

        let empty_token = Token {
            name: String::from("Empty"),
            class: TokenClass::Delimiter 
        };

        pda.push(&empty_token);

        while let Some(token) = iter.next() {
            curr_index += 1;
            match token.class {
                TokenClass::Identifier if RESERVED_WORDS.contains(&token.name.as_str()) => {
                    
                    println!("Comparing precedence of {} and {}", pda[prev_op_index].name, token.name);

                    match grammar_rules.lookup_precedence(&pda[prev_op_index].name, &token.name) {
                        Precedence::Yields => {
                            println!("Yields... Pushing {} to the stack.\n", token.name);
                            pda.push(token); 
                        }

                        Precedence::Takes => println!("Reduction"),
                        
                        Precedence::Equal => {
                            println!("Equal... Pushing {} to the stack.\n", token.name);
                            pda.push(token)
                        }

                        Precedence::Nil => return Err(SyntaxError(token.name.as_str(), &token.class)),
                    }
                    prev_op_index = curr_index;
                }

                TokenClass::Delimiter => {

                    println!("Comparing precedence of {} and {}", pda[prev_op_index].name, token.name);

                    match grammar_rules.lookup_precedence(&pda[prev_op_index].name, &token.name) {
                        Precedence::Yields => {
                            println!("Yields... Pushing {} to the stack.\n", token.name);
                            pda.push(token); 
                        }

                        Precedence::Takes => println!(""),
                        Precedence::Equal => {
                            println!("Equal... Pushing {} to the stack.\n", token.name);
                            pda.push(token)
                        }
                        Precedence::Nil => return Err(SyntaxError(token.name.as_str(), &token.class)),
                    }
                    prev_op_index = curr_index;
                }

                TokenClass::Op => {
                    println!("Comparing precedence of {} and {}", pda[prev_op_index].name, token.name);

                    match grammar_rules.lookup_precedence(&pda[prev_op_index].name, &token.name) {
                        Precedence::Yields => {
                            println!("Yields... Pushing {} to the stack.\n", token.name);
                            pda.push(token); 
                        }

                        Precedence::Takes => println!("Reduction"),
                        Precedence::Equal => {
                            println!("Equal... Pushing {} to the stack.\n", token.name);
                            pda.push(token)
                        }
                        Precedence::Nil => return Err(SyntaxError(token.name.as_str(), &token.class)),
                    }
                    prev_op_index = curr_index;
            
                }

                TokenClass::Identifier  => {
                    println!("Pushing {} to the stack.\n", token.name);
                    pda.push(token)
                }
                TokenClass::Literal => {
                    println!("Pushing {} to the stack.\n", token.name);
                    pda.push(token)
                }

                TokenClass::Unknown => return Err(SyntaxError(token.name.as_str(), &token.class))
            }
            println!("Curr Index: {}, Last Op Index: {}", curr_index, prev_op_index);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn in_memory_tokens_work() {
        let mut syn = Syntax::new();
        syn.pda_stack_from_memory("program.java");

        let name = syn.tokens.first();
        assert_eq!(name.unwrap().name, String::from("CLASS"));

        let name = syn.tokens.last();
        assert_eq!(name.unwrap().name, String::from("}"));
    }

    #[test]
    #[ignore]
    fn from_file_tokens_work() {
        let mut syn = Syntax::new();
        syn.pda_stack_from_file("symbols");

        let name = syn.tokens.first();
        assert_eq!(name.unwrap().name, String::from("CLASS"));
    }

    #[test]
    fn error_reporting() {
        let mut syn = Syntax::new();
        syn.pda_stack_from_memory("program.java");

        let good = syn.complete_analysis();
        match good {
            Ok(_) => println!("Make sure this errors out."),
            Err(e) => println!("{}", e),
        }
    }

    #[test]
    fn syntax_works() {
        let mut syn = Syntax::new();
        syn.pda_stack_from_memory("program.java");

        let good = syn.complete_analysis();
        match good {
            Ok(_) => println!("Finished"),
            Err(e) => println!("{}", e),
        }
    }

    #[test]
    fn parse_works() {
        let grammar = GrammarTable::new("table");
        grammar.print_table();

        let prez = grammar.lookup_precedence(&String::from("+"), &String::from("+"));
        println!("{:?}", prez);
    }
}
