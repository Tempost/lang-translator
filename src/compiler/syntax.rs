use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::iter::Peekable;
use std::path::Path;
use std::vec::IntoIter;

// Take tokens from lex portion of the code
use crate::compiler::lexical::{Token, TokenClass, Tokenize};
use crate::compiler::precedence::{PrecedenceGrammar, OPG};
use crate::compiler::tableindex::TableIndex;

type TokenList = Vec<Token>;
pub type QuadList = Vec<Quad>;

pub struct Syntax {
    token_iter: Peekable<IntoIter<Token>>,
    token_stack: TokenList,
    pub polish: TokenList,
    pub quads: QuadList,
    p_func: PFunc,
    top_of_stack: usize,
    op_stack: TokenList,
    prev_op: Token,
}

struct PFunc {
    f: Vec<i32>,
    g: Vec<i32>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Quad {
    pub op: Token,
    pub param_one: Token,
    pub param_two: Token,
    pub temp: Token,
}

#[derive(PartialEq, Eq)]
enum Handle {
    Yields,
    Takes,
    Equal,
}

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

impl fmt::Display for Quad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{},{},{}\n",
            self.op.name, self.param_one.name, self.param_two.name, self.temp.name
        )
    }
}

impl Syntax {
    pub fn new(file: &str, flag: bool) -> Self {
        let tokens: Peekable<IntoIter<Token>>;
        if flag {
            tokens = Syntax::tokens_from_memory(file);
        } else {
            tokens = Syntax::tokens_from_file(file);
        }

        Syntax {
            top_of_stack: 0,
            token_iter: tokens,
            token_stack: Vec::new(),
            polish: Vec::new(),
            quads: Vec::new(),
            p_func: PFunc::new(),
            op_stack: Vec::new(),
            prev_op: Token::empty(),
        }
    }

    // Return a stack of iterable tokens
    pub fn tokens_from_memory(file: &str) -> Peekable<IntoIter<Token>> {
        let mut lex = Tokenize::create_scanner(file).unwrap();
        let mut stack: TokenList = Vec::new();

        // Analysis needs a "terminator" token at the start
        stack.push(Token::terminator());
        while let Some(token) = lex.next() {
            stack.push(token);
        }

        stack.into_iter().peekable()
    }

    pub fn tokens_from_file(file: &str) -> Peekable<IntoIter<Token>> {
        let token_file = File::open(file);

        match token_file {
            Ok(file) => {
                let buf = io::BufReader::new(file);
                let mut stack: TokenList = Vec::new();

                stack.push(Token::terminator());
                for line in buf.lines() {
                    Syntax::parse_token(&mut stack, line);
                }
                stack.into_iter().peekable()
            }
            Err(e) => panic!("{}", e),
        }
    }

    pub fn consume_polish(&mut self) -> Result<(), ()> {
        // loop until next op
        let mut pol_iter = self.polish.iter();
        let mut param_stack: TokenList = Vec::new();
        let mut quads: QuadList = Vec::new();
        let mut temp_id = 1;

        loop {
            if let Some(token) = pol_iter.next() {
                match token.class {
                    TokenClass::Op => {
                        if !token.name.eq(&String::from("=")) {
                            let temp = Token::temp_gen(temp_id);
                            temp_id += 1;

                            quads.push(Quad {
                                op: token.to_owned(),
                                param_one: param_stack.pop().unwrap(),
                                param_two: param_stack.pop().unwrap(),
                                temp: temp.clone(),
                            });
                            param_stack.push(temp);
                        } else {
                            quads.push(Quad {
                                op: token.to_owned(),
                                param_one: param_stack.pop().unwrap(),
                                param_two: param_stack.pop().unwrap(),
                                temp: Token::empty(),
                            });
                        }
                    }
                    TokenClass::Delimiter => continue,
                    TokenClass::ReservedWord => {
                        if token.name.eq("GET") || token.name.eq("PUT") {
                            quads.push(Quad {
                                op: token.to_owned(),
                                param_one: param_stack.pop().unwrap(),
                                param_two: Token::empty(),
                                temp: Token::empty(),
                            });
                        }
                    },
                    _ => param_stack.push(token.to_owned()),
                }
            } else {
                break;
            }
        }
        self.quads = quads;
        Ok(())
    }

    fn parse_token(stack: &mut TokenList, line: io::Result<String>) {
        match line {
            Ok(token) => {
                let mut iter = token.split_whitespace();
                let name = iter.next().unwrap();
                let class = TokenClass::from(iter.next().unwrap());

                stack.push(Token {
                    name: name.to_string(),
                    class,
                });
            }

            Err(e) => panic!("{:?}", e),
        }
    }

    fn table_lookup(&self, f: &Token, g: &Token) -> Handle {
        let f_index = usize::from(TableIndex::from(&f.name));
        let g_index = usize::from(TableIndex::from(&g.name));

        let f_val = &self.p_func.f[f_index];
        let g_val = &self.p_func.g[g_index];

        if f_val > g_val {
            return Handle::Takes;
        }

        if f_val < g_val {
            return Handle::Yields;
        }

        Handle::Equal
    }

    // Advance iterator to the next operator, adding variables and literals to the stacks
    fn next_op(&mut self) -> Option<Token> {
        while let Some(token) = self.token_iter.next() {
            match token.class {
                TokenClass::ReservedWord | TokenClass::Delimiter | TokenClass::Op => {
                    self.token_stack.push(token.clone());
                    return Some(token);
                }
                TokenClass::Unknown => panic!("[ Error ] Invalid next operator."),

                // Add Ident or literal to stack
                _ => {
                    self.top_of_stack += 1;
                    self.polish.push(token.clone());
                    self.token_stack.push(token);
                }
            }
        }
        None
    }

    // Search for the last operator from the processed tokens
    fn last_op(&mut self) -> Option<Token> {
        let mut iter = self.token_stack.iter();

        while let Some(token) = iter.next_back() {
            match token.class {
                TokenClass::ReservedWord | TokenClass::Delimiter | TokenClass::Op => {
                    return Some(token.to_owned());
                }
                _ => return None,
            }
        }
        None
    }

    // Advance through the input tokens
    fn next_token(&mut self) -> Option<Token> {
        if let Some(token) = self.token_iter.next() {
            self.token_stack.push(token.clone());
            return Some(token);
        } else {
            None
        }
    }

    pub fn complete_analysis(&mut self) {
        // Consume first token pushing it to the stack, always a indicator to the start of input
        self.token_stack.push(self.token_iter.next().unwrap());

        self.s_stmt();
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
        self.prev_op = self.last_op().unwrap();
        while let Some(oper) = self.next_op() {
            match self.table_lookup(&self.prev_op, &oper) {
                Handle::Yields => {
                    self.prev_op = oper.clone();
                    self.op_stack.push(oper);
                    self.expression();
                    self.polish.push(self.op_stack.pop().unwrap());
                }

                Handle::Takes => break,
                Handle::Equal => todo!(),
            }

            while let Some(op) = self.op_stack.pop() {
                self.polish.push(op);
            }
        }
    }

    fn expression(&mut self) {
        self.term();
        while let Some(oper) = self.next_token() {
            match self.table_lookup(&self.prev_op, &oper) {
                Handle::Yields => {
                    self.op_stack.push(oper.clone());
                    self.term();
                    self.polish.push(self.op_stack.pop().unwrap());
                }
                Handle::Takes => break,
                Handle::Equal => self.polish.push(oper.clone()),
            }
            self.prev_op = oper;
        }
    }

    fn term(&mut self) {
        self.factor();
        while let Some(oper) = self.next_token() {
            match self.table_lookup(&self.prev_op, &oper) {
                Handle::Yields => {
                    self.op_stack.push(oper.clone());
                }

                Handle::Takes => {
                    self.polish.push(self.op_stack.pop().unwrap());
                    self.op_stack.push(oper.clone());
                }

                Handle::Equal => continue,
            }
            self.factor();
            self.prev_op = oper;
        }
    }

    fn factor(&mut self) {
        if let Some(oper) = self.next_token() {
            if oper.class != TokenClass::Delimiter {
                self.polish.push(oper);
            } else {
                self.expression();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn syntax_test1() {
        let mut syn = Syntax::new("test1.java", true);
        syn.complete_analysis();
        syn.consume_polish().unwrap();
    }

    #[test]
    fn syntax_test2() {
        let mut syn = Syntax::new("test2.java", true);
        syn.complete_analysis();
        syn.consume_polish().unwrap();
    }

    #[test]
    fn syntax_test3() {
        let mut syn = Syntax::new("test5.java", true);
        syn.complete_analysis();
        syn.consume_polish().unwrap();

        let mut iter = syn.quads.iter();

        while let Some(quad) = iter.next() {
            println!("{}", quad);
        }
    }

    #[test]
    fn syntax_test4() {
        let mut lex = Tokenize::create_scanner("test3.java").unwrap();

        while let Some(token) = lex.next() {
            Tokenize::token_to_file(token);
        }

        let mut syn = Syntax::new("tokens", false);
        syn.complete_analysis();
    }
}
