use std::fmt;
use std::vec::IntoIter;

// Take tokens from lex portion of the code
use crate::compiler::lexical::{Token, TokenClass, Tokenize};
use crate::compiler::precedence::{PrecedenceGrammar, OPG};
use crate::compiler::tableindex::TableIndex;

type TokenList = Vec<Token>;

type Result<'a, T> = std::result::Result<T, SyntaxError<'a>>;

pub struct Syntax {
    top_of_stack: usize,
    token_iter: IntoIter<Token>,
    token_stack: TokenList,
    p_func: PFunc
}

struct PFunc {
    f: Vec<i32>,
    g: Vec<i32>
}


pub struct Quads {
    op: Token,
    ident1: Token,
    ident2: Token,
    indet3: Token,
}

enum Handle {
    Yields,
    Takes,
    Equal,
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

impl PFunc {
    fn new() -> Self {
        let funcs = PFunc::get_precedence_func(); 
        PFunc { f: funcs.0, g: funcs.1 }
    }

    // Using the handles generated in a previous step, construct the precedence function
    // using the method discussed in class to also decrease the memeory size
    fn get_precedence_func() -> (Vec<i32>, Vec<i32>) {
        let mut grammar: OPG = OPG::new();
        grammar.parse_input(false);
        grammar.shrink_precedence();

        let f: Vec<i32> = grammar.f;
        let g: Vec<i32> = grammar.g;
         
        (f,g)
    }
}

impl Syntax {
    fn new(file: &str) -> Self {
        Syntax { 
            top_of_stack: 0,
            token_iter: Syntax::tokens_from_memory(file),
            token_stack: Vec::new(),
            p_func: PFunc::new(),
        }
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


        Ok(())
    }

    fn table_lookup(&mut self, f: &String, g: &String) -> Handle  {
        let f_index = usize::from(TableIndex::from(f));    
        let g_index = usize::from(TableIndex::from(g));    

        let f_val = self.p_func.f[f_index];
        let g_val = self.p_func.g[g_index];

        if f_val > g_val {
            return Handle::Takes;
        }

        if f_val < g_val {
            return Handle::Yields;
        }

        Handle::Equal
    }

    fn program(&mut self) {
       self.block();
    }

    fn block(&mut self) {
        self.const_def_part();
    }

    fn const_def_part(&mut self) {
        self.const_list();
    }

    fn const_list(&mut self) {
        self.var_def_part();
    }

    fn var_def_part(&mut self) { 
        self.var_list();
    }
    
    fn var_list(&mut self) {
        self.proc_def_part();
    }
    
    fn proc_def_part(&mut self) {
        self.stmt();
    }
    
    fn stmt(&mut self) {
        self.s_stmt();
    }
    
    fn s_stmt(&mut self) {
        self.call_stmt()
    }
    
    fn call_stmt(&mut self) {
        self.param_list();
    }
    
    fn param_list(&mut self) {
        self.ident_list();
    }
    
    fn ident_list(&mut self) {
        self.comp_stmt();
    }
    
    fn comp_stmt(&mut self) {
        self.stmt_list();
    }
    
    fn stmt_list(&mut self) {
        self.if_stmt();
    }
    
    fn if_stmt(&mut self) {
        self.while_stmt();
    }
    
    fn while_stmt(&mut self) {
        self.bool_exp();
    }
    
    fn bool_exp(&mut self) {
        self.rel_op();
    }
    
    fn rel_op(&mut self) {
        self.expression();
    }
    
    fn expression(&mut self) {
        self.term(); 
    }

    fn term(&mut self) {
    
    }
    
    fn add_op(&mut self) {

    }
    
    fn mop(&mut self) {

    }
    
    // Handles '(' and ')' tokens
    fn factor(&mut self) {
        
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
