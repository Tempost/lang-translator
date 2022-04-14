use std::fmt;
use std::vec::IntoIter;

// Take tokens from lex portion of the code
use crate::compiler::lexical::{Token, TokenClass, Tokenize};
use crate::compiler::precedence::{PrecedenceGrammar, OPG};
use crate::compiler::tableindex::TableIndex;

type TokenList = Vec<Token>;

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
    top_of_stack: usize,
    token_iter: IntoIter<Token>,
    token_stack: TokenList,
    p_func: Vec<Vec<i32>>
}

pub struct Quads {
    op: SyntaxClass,
    ident1: SyntaxClass,
    ident2: SyntaxClass,
    indet3: SyntaxClass,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SyntaxError<'a>(&'a str, TokenClass);

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
    fn new(file: &str) -> Self {
        Syntax { 
            top_of_stack: 0,
            token_iter: Syntax::tokens_from_memory(file),
            token_stack: Vec::new(),
            p_func: Syntax::get_precedence_func(),
        }
    }

    // Using the handles generated in a previous step, construct the precedence function
    // using the method discussed in class to also decrease the memeory size
    fn get_precedence_func() -> Vec<Vec<i32>> {
        let mut grammar: OPG = OPG::new();
        grammar.parse_input(false);
        grammar.shrink_precedence();

        let mut matrix: Vec<Vec<i32>> = Vec::new();

        matrix.push(grammar.f);
        matrix.push(grammar.g);
         
        matrix
    }

    // Return a stack of iterable tokens
    fn tokens_from_memory(file: &str) -> IntoIter<Token> {
        let mut lex = Tokenize::create_scanner(file).unwrap();
        let mut stack: TokenList = Vec::new();

        // Stack requires to be wrapped in a starting/end dummy token
        let terminator = Token {
            name: String::from("Terminator"),
            class: TokenClass::Delimiter
        };
        
        // Analysis needs a "terminator" token on both the start and end of the stack
        stack.push(terminator.clone());
        while let Some(token) = lex.next() {
            stack.push(token);
        }

        stack.push(terminator);

        stack.into_iter()
    }

    pub fn complete_analysis(&mut self) -> Result<()> {
        // Consume first token, pushing it to the stack
        self.token_stack.push(self.token_iter.next().unwrap().to_owned());
        self.top_of_stack += 1;

        while let Some(token) = self.token_iter.next() {
            match token.class {
                TokenClass::Identifier | TokenClass::Literal => {
                }

                TokenClass::ReservedWord | TokenClass::Op | TokenClass::Delimiter => {
                }

                TokenClass::Unknown => return Err(SyntaxError("Something went wrong parsing at: ", token.class)),
            }
        }
        Ok(())
    }
}



#[cfg(test)]
mod test {

    use super::*;

    #[test]
    #[should_panic]
    fn syntax_error() {
        let mut syn = Syntax::new("program.java");

        assert!(syn.complete_analysis().is_err());
    }
    
    #[test]
    fn syntax_ok() {
        let mut syn = Syntax::new("program.java");
        assert!(syn.complete_analysis().is_ok());
    }
}
