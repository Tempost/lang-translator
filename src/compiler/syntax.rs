use std::error::Error;
use std::fmt;

// Take tokens from lex portion of the code
use crate::compiler::lexical::{Token, TokenClass, Tokenize};
use crate::compiler::precedence::{Precedence, TableIndex, GrammarTable };

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
    token_stack: Vec<Token>
}


struct Quads {
    op: String,
    var_one: String,
    var_two: String,
    var_three: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SyntaxError<'a>(&'a str, &'a TokenClass);

impl<'a> fmt::Display for SyntaxError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ Syntax Error ] at token name: {} class: {:?}",
            self.0, self.1
        )
    }
}

impl Syntax {
    fn new() -> Self {
        Syntax { 
            tokens: Vec::new(),
            token_stack: Vec::new()
        }
    }

    pub fn stack_from_file(&mut self, file: &str) {
        // Parse name -- TokenClass into token struct, push into TokenList
        unimplemented!();
    }

    pub fn stack_from_memory(&mut self, file: &str) {
        let mut lex = Tokenize::create_scanner(file).unwrap();
        while let Some(token) = lex.next() {
            self.tokens.push(token);
        }
    }

    pub fn complete_analysis(&mut self) -> Result<()> {
        let grammar_rules = GrammarTable::new("table");

        // push token into PDA stack until a operator or reserved word is hit
        // Compare last operator/RW with next from the token stack to determine precendece
        // Determine what to do based on output of the compare, IE contuine pushing to the PDA
        // stack, or reduce to a new handle(syntax class). Contuine until no more tokens

        let mut iter = self.tokens.iter();
        let mut prev_op = TableIndex::Nil;

        let end_token = Token {
            name: String::from("Terminator"),
            class: TokenClass::Delimiter 
        };

        self.token_stack.push(end_token);

        while let Some(token) = iter.next() {
            match token.class {
                TokenClass::Delimiter | TokenClass::Op | TokenClass::ReservedWord => {
                    println!("comparing precedence of {:?} and {:?}", prev_op, TableIndex::from(&token.name));

                    match grammar_rules.lookup_precedence(prev_op, &token.name) {
                        Precedence::Yields => {
                            // Push into stack
                            println!("Yields... Pushing {:?} to the stack.\n", TableIndex::from(&token.name));
                            self.token_stack.push(token.clone()); 
                        }

                        Precedence::Takes => {
                            // Pop up to last yields and reduce/generate quads
                            // Create function that will loop through and pop until a yields was
                            // hit. Return all items popped as a new vec
                            println!("Takes... Pushing {:?} to the stack.\n", TableIndex::from(&token.name));
                            self.token_stack.push(token.clone()); 
                        },
                        
                        Precedence::Equal => {
                            // Just push into stack
                            println!("Equal... Pushing {:?} to the stack.\n", TableIndex::from(&token.name));
                            self.token_stack.push(token.clone()); 
                        }

                        Precedence::Nil => return Err(SyntaxError(token.name.as_str(), &token.class)),
                    }
                    prev_op = TableIndex::from(&token.name);
                }

                TokenClass::Identifier | TokenClass::Literal => {
                    self.token_stack.push(token.clone()); 
                }

                TokenClass::Unknown => return Err(SyntaxError(token.name.as_str(), &token.class))
            }
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
        syn.stack_from_memory("program.java");

        let name = syn.tokens.first();
        assert_eq!(name.unwrap().name, String::from("CLASS"));

        let name = syn.tokens.last();
        assert_eq!(name.unwrap().name, String::from("}"));
    }

    #[test]
    #[ignore]
    fn from_file_tokens_work() {
        let mut syn = Syntax::new();
        syn.stack_from_file("symbols");

        let name = syn.tokens.first();
        assert_eq!(name.unwrap().name, String::from("CLASS"));
    }

    #[test]
    fn syntax_works() {
        let mut syn = Syntax::new();
        syn.stack_from_memory("program.java");

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
    }
}
