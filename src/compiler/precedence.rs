use std::fs::File;
use std::io::{BufRead, BufReader};

pub type RelMatrix = Vec<Vec<Precedence>>;

#[derive(Debug, Clone, Copy)]
pub enum Precedence {
    Yields,
    Takes,
    Equal,
    Nil,
}

#[derive(Debug, Clone, Copy)]
enum TableIndex {
    Nil,
    Assignment,
    Plus,
    Minus,
    LParen,
    RParen,
    Mop,
    Div,
    IF,
    Then,
    Odd,
    Equal,
    NEqual,
    GreaterThan,
    LessThan,
    GEqual,
    LEqual,
    LBracket,
    RBracket,
    Class,
    Var,
    Const,
    Semi,
    Comma,
}

impl From<Precedence> for String {
    fn from(p: Precedence) -> String {
        match p {
            Precedence::Yields => "<".to_string(),
            Precedence::Takes => ">".to_string(),
            Precedence::Equal => "=".to_string(),
            Precedence::Nil => "0".to_string(),
        }
    }
}

impl From<TableIndex> for usize {
    fn from(t: TableIndex) -> usize {
        match t {
            TableIndex::Nil => 0,
            TableIndex::Assignment => 1,
            TableIndex::Plus => 2,
            TableIndex::Minus => 3,
            TableIndex::LParen => 4,
            TableIndex::RParen => 5,
            TableIndex::Mop => 6,
            TableIndex::Div => 7,
            TableIndex::IF => 8,
            TableIndex::Then => 9,
            TableIndex::Odd => 10,
            TableIndex::Equal => 11,
            TableIndex::NEqual => 12,
            TableIndex::GreaterThan => 13,
            TableIndex::LessThan => 14,
            TableIndex::GEqual => 15,
            TableIndex::LEqual => 16,
            TableIndex::LBracket => 17,
            TableIndex::RBracket => 18,
            TableIndex::Class => 19,
            TableIndex::Var => 20,
            TableIndex::Const => 21,
            TableIndex::Semi => 22,
            TableIndex::Comma => 23,
        }
    }
}

impl From<&String> for TableIndex {
    fn from(s: &String) -> TableIndex {
        match s.as_str() {
            "=" => TableIndex::Assignment,
            "+" => TableIndex::Plus,
            "-" => TableIndex::Minus,
            "(" => TableIndex::LParen,
            ")" => TableIndex::RParen,
            "*" => TableIndex::Mop,
            "/" => TableIndex::Div,
            "IF" => TableIndex::IF,
            "THEN" => TableIndex::Then,
            "ODD" => TableIndex::Odd,
            "==" => TableIndex::Equal,
            "!=" => TableIndex::NEqual,
            ">" => TableIndex::GreaterThan,
            "<" => TableIndex::LessThan,
            ">=" => TableIndex::GEqual,
            "<=" => TableIndex::LEqual,
            "{" => TableIndex::LBracket,
            "}" => TableIndex::RBracket,
            "CLASS" => TableIndex::Class,
            "VAR" => TableIndex::Var,
            "CONST" => TableIndex::Const,
            ";" => TableIndex::Semi,
            "," => TableIndex::Comma,
            _ => TableIndex::Nil,
        }
    }
}

pub struct GrammarTable {
    pub table: RelMatrix,
    pub dimension: usize,
}

impl GrammarTable {
    pub fn new(filename: &str) -> Self {
        let file = File::open(filename).expect("No such file.");
        let file = BufReader::new(file);

        let mut lines = file.lines();

        let mut matrix: RelMatrix = Vec::new();

        let mut grammar_table = GrammarTable {
            table: Vec::new(),
            dimension: 0,
        };

        while let Some(line) = lines.next() {
            let matrix_dim = line.unwrap();

            if matrix_dim.is_empty() {
                continue;
            }

            let mut m_size = matrix_dim.split(' ');

            let (x, y) = (
                m_size.next().unwrap().parse::<i32>().ok().unwrap(),
                m_size.next().unwrap().parse::<i32>().ok().unwrap(),
            );

            if x != y {
                panic!("Please supply a valid square matrix.");
            }

            grammar_table.dimension = x as usize;

            for _ in 0..x {
                matrix.push(
                    lines
                        .next()
                        .unwrap()
                        .unwrap()
                        .trim()
                        .split_whitespace()
                        .map(|x| -> Precedence {
                            match x.to_string().as_str() {
                                "0" => Precedence::Nil,
                                "<" => Precedence::Yields,
                                ">" => Precedence::Takes,
                                "=" => Precedence::Equal,
                                _ => panic!("[ Error ] Ouch found something not allowed."),
                            }
                        })
                        .collect::<Vec<Precedence>>(),
                );
            }

            grammar_table.table = matrix.clone();
            matrix.clear();
        }

        grammar_table
    }

    pub fn lookup_precedence(&self, op_one: &String, op_two: &String) -> Precedence {
        let op_one_class = TableIndex::from(op_one);
        let op_two_class = TableIndex::from(op_two);

        self.table[usize::from(op_one_class)][usize::from(op_two_class)]
    }

    pub fn print_table(&self) {
        let mut iter = self.table.iter();

        while let Some(row) = iter.next() {
            let parsed = row
                .iter()
                .map(|x| String::from(*x))
                .collect::<Vec<String>>();

            println!("{:?}", parsed);
        }
    }
}
