use std::iter::Peekable;
use std::vec::IntoIter;

// Take tokens from lex portion of the code
use crate::compiler::lexical::{Token, TokenClass, Tokenize};
use crate::compiler::precedence::{PrecedenceGrammar, OPG};
use crate::compiler::tableindex::TableIndex;

type TokenList = Vec<Token>;
type QuadList = Vec<Quads>;

pub struct Syntax {
    top_of_stack: usize,
    token_iter: Peekable<IntoIter<Token>>,
    token_stack: TokenList,
    polish: TokenList,
    quad_stack: QuadList,
    p_func: PFunc,
}

struct PFunc {
    f: Vec<i32>,
    g: Vec<i32>,
}

pub struct Quads {
    op: Token,
    ident1: Token,
    ident2: Token,
    indet3: Token,
}

#[derive(Debug, PartialEq, Eq)]
enum Handle {
    Yields,
    Takes,
    Equal,
}

// type Result<'a, T> = std::result::Result<T, SyntaxError<'a>>;

// #[derive(Debug, PartialEq, Eq)]
// pub struct SyntaxError<'a>(&'a str, TokenClass);

// impl<'a> fmt::Display for SyntaxError<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "[ Syntax Error ] at token name: {} class: {:?}",
//             self.0, self.1
//         )
//     }
// }

impl PFunc {
    fn new() -> Self {
        let funcs = PFunc::get_precedence_func();
        PFunc {
            f: funcs.0,
            g: funcs.1,
        }
    }

    // Using the handles generated in a previous step, construct the precedence function
    // using the method discussed in class to also decrease the memeory size
    fn get_precedence_func() -> (Vec<i32>, Vec<i32>) {
        let mut grammar: OPG = OPG::new();
        grammar.parse_input(false);
        grammar.shrink_precedence();

        let f: Vec<i32> = grammar.f;
        let g: Vec<i32> = grammar.g;

        (f, g)
    }
}

impl Syntax {
    fn new(file: &str) -> Self {
        Syntax {
            top_of_stack: 0,
            token_iter: Syntax::tokens_from_memory(file),
            token_stack: Vec::new(),
            polish: Vec::new(),
            quad_stack: Vec::new(),
            p_func: PFunc::new(),
        }
    }

    // Return a stack of iterable tokens
    fn tokens_from_memory(file: &str) -> Peekable<IntoIter<Token>> {
        let mut lex = Tokenize::create_scanner(file).unwrap();
        let mut stack: TokenList = Vec::new();

        // Analysis needs a "terminator" token at the start
        stack.push(Token::terminator());
        while let Some(token) = lex.next() {
            stack.push(token);
        }

        stack.into_iter().peekable()
    }

    fn table_lookup(&self, f: &Token, g: &Token) -> Handle {
        let f_index = usize::from(TableIndex::from(&f.name));
        let g_index = usize::from(TableIndex::from(&g.name));

        let f_val = &self.p_func.f[f_index];
        let g_val = &self.p_func.g[g_index];

        println!("F:{} G:{}", f_val, g_val);
        if f_val > g_val {
            println!("Takes\n");
            return Handle::Takes;
        }

        if f_val < g_val {
            println!("Yields\n");
            return Handle::Yields;
        }

        Handle::Equal
    }

    pub fn complete_analysis(&mut self) {
        // Consume first token pushing it to the stack, always a indicator to the start of input
        self.token_stack
            .push(self.token_iter.next().unwrap());

        // Peek next value, if we have a token to parse we will contuine parsing.
        // Handles multiple functions
        // while let Some(_) = self.token_iter.peek() {
        self.s_stmt();
        // }

        self.polish.iter().for_each(|x| println!("{}", x.name));

        println!("Finished analysis!");
    }

    // Advance iterator to the next operator, skiping variables and literals
    fn next_op(&mut self) -> Option<Token> {
        while let Some(token) = self.token_iter.next() {
            match token.class {
                TokenClass::ReservedWord | TokenClass::Delimiter | TokenClass::Op => {
                    self.token_stack.push(token.clone());
                    return Some(token)
                }
                TokenClass::Unknown => panic!("[ Error ] Invalid next operator."),
                _ => {
                    self.top_of_stack += 1;
                    self.polish.push(token.clone());
                    self.token_stack.push(token);
                }
            }
        }
        None
    }

    fn last_op(&mut self) -> Option<Token> {
        let mut r_iter = self.token_stack.iter().rev(); 
        while let Some(token) = r_iter.next() {
            match token.class {
                TokenClass::ReservedWord | TokenClass::Delimiter | TokenClass::Op => {
                    return Some(token.to_owned())
                }
                _ => continue,
            }
        }
        None
    }

    fn next_token(&mut self) -> Option<Token> {
        if let Some(token) = self.token_iter.next() {
            return Some(token);
        } else {
            None
        }
    }

    fn program(&mut self) {
        println!("Analysis of program.");
        let mut token = self.next_token().unwrap();

        // NOTE: any way to just get just the ref to this value?
        let mut prev_op = self.token_stack[self.top_of_stack - 1].clone();
        println!("Comparing: {:?} and {:?}", prev_op, token);

        if self.table_lookup(&prev_op, &token) == Handle::Yields {
            prev_op = token;
            token = self.next_op().unwrap();

            println!("Comparing: {:?} and {:?}\n", prev_op, token);
            if self.table_lookup(&prev_op, &token) == Handle::Yields {
                self.block();
            } else {
                panic!(
                    "[ Error ] Syntax error at {} -- {}",
                    prev_op.name, token.name
                );
            }
        } else {
            panic!(
                "[ Error ] Syntax error at {} -- {}",
                prev_op.name, token.name
            );
        }
    }

    fn block(&mut self) {
        println!("Analysis of block.");
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

    fn s_stmt(&mut self) {
        let mut prev_op = self.last_op().unwrap();
        while let Some(oper) = self.next_op() {
            
            println!("Analysis of simple statement.");
            println!("Comparing: {:?} and {:?}", prev_op, oper);

            match self.table_lookup(&prev_op, &oper) {
                Handle::Yields => {
                    self.expression();
                    self.polish.push(oper);
                },

                Handle::Takes => break,
                Handle::Equal => todo!(),
            }
            prev_op = self.last_op().unwrap();
        }
    }

    fn expression(&mut self) {
        println!("Analysis of expression.");

        self.term();
        let mut prev_op = self.last_op().unwrap();
        while let Some(oper) = self.next_op() {

            println!("Returning to expression.");
            println!("Comparing: {:?} and {:?}", &prev_op, oper);

            match self.table_lookup(&prev_op, &oper) {
                Handle::Yields => {
                    self.term();
                    self.polish.push(oper);
                },
                Handle::Takes => break,
                Handle::Equal => todo!(),
            }
            prev_op = self.last_op().unwrap();
        }
    }

    fn term(&mut self) {
        println!("Analysis of term.");
        
        let prev_op = self.last_op().unwrap();
        let oper = self.next_op().unwrap();
        println!("Comparing: {:?} and {:?}", prev_op, oper);

        match self.table_lookup(&prev_op, &oper) {
            Handle::Yields => {
                self.term();
                self.polish.push(oper);
            }

            Handle::Takes => {
                self.polish.push(oper);
            },

            Handle::Equal => todo!(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn syntax_ok() {
        let mut syn = Syntax::new("test2.java");
        syn.complete_analysis();
    }
}
