use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

use crate::boolean::matrix::{self, Matrix};

pub trait PrecedenceGrammar {
    fn new() -> Self;
    fn parse_input(&mut self, compute_handles: bool);
    fn shrink_precedence(&mut self);
    fn compute_handles(&mut self);
}

pub struct OPG {
    first: Matrix,
    first_term: Matrix,
    last: Matrix,
    last_term: Matrix,
    takes: Matrix,
    yields: Matrix,
    equal: Matrix,
    rtc: Matrix,
    pub f: Vec<i32>,
    pub g: Vec<i32>,
    m_dimension: usize,
}

pub struct SPG {
    first: Matrix,
    last: Matrix,
    takes: Matrix,
    yields: Matrix,
    equal: Matrix,
    rtc: Matrix,
    pub f: Vec<i32>,
    pub g: Vec<i32>,
    m_dimension: usize,
}

impl OPG {
    fn create_b_matrix(&mut self) {
        let b_matrix_len = self.m_dimension * 2;
        let mut b_matrix: Matrix = vec![vec![false; b_matrix_len]; b_matrix_len]; 

        let takes_equal = matrix::combine_matrix(&self.takes, &self.equal);
        let yields_equal = matrix::combine_matrix(&self.yields, &self.equal);

        let yields_equal = matrix::transpose(&yields_equal);

        for row in 0..b_matrix_len {
            for col in 0..b_matrix_len {
                if row < (self.m_dimension) && col >= (self.m_dimension) {
                    b_matrix[row][col] = takes_equal[row][col % (self.m_dimension)];
                }

                if row >= (self.m_dimension) && col < (self.m_dimension){
                    b_matrix[row][col] = yields_equal[row % (self.m_dimension)][col % (self.m_dimension)];
                }
            }
        }

        let identity = matrix::create_identity(b_matrix_len); 
        b_matrix = matrix::transitive_closure(&b_matrix);
        b_matrix = matrix::sum(&b_matrix, &identity); 
        self.rtc = b_matrix;
    }
}

impl PrecedenceGrammar for OPG {
    fn new() -> OPG {
        OPG {
            first: Vec::new(),
            first_term: Vec::new(),
            last: Vec::new(),
            last_term: Vec::new(),
            takes: Vec::new(),
            yields: Vec::new(),
            equal: Vec::new(),
            rtc: Vec::new(),
            f: Vec::new(),
            g: Vec::new(),
            m_dimension: 0,
        }
    }

    fn parse_input(&mut self, compute_handles: bool) {
        let path = Path::new("src/compiler/fsa_tables/handles.txt");
        let path_string = path.display();

        let in_file = match File::open(&path) {
            Err(e) => panic!("[ Error ] Trouble locating {}, {}", path_string, e),
            Ok(file) => io::BufReader::new(file),
        };

        let mut lines = in_file.lines();

        let mut matrix: Matrix = Vec::new();
        while let Some(line) = lines.next() {
            let matrix_dim = line.unwrap();

            // Skip any blank lines in between the matrices
            if matrix_dim.is_empty() {
                continue
            }

            let mut m_size = matrix_dim.split(' ');

            // Prase the dimensions of the matricies
            let (x, y) = (
                m_size.next().unwrap().parse::<i32>().ok().unwrap(),
                m_size.next().unwrap().parse::<i32>().ok().unwrap(),
            );

            if x != y {
                panic!("[ Error ] Please supply a valid square matrix.");
            }

            self.m_dimension = x as usize;

            let matrix_name: &str = &lines.next().unwrap().ok().unwrap().to_lowercase();

            for _ in 0..x {
                matrix.push(
                    lines
                        .next()
                        .unwrap()
                        .unwrap()
                        .trim()
                        .split_whitespace()
                        .map(|x| -> bool {
                            match x.parse::<u32>().ok().unwrap() {
                                0 => false,
                                1 => true,
                                _ => panic!("[ Error ] Ouch found something not allowed."),
                            }
                        })
                        .collect::<Vec<bool>>(),
                );
            }
           
            if compute_handles {
                match matrix_name {
                    "First" => self.first = matrix.clone(),
                    "First Term" => self.first_term = matrix.clone(),
                    "Last" => self.last = matrix.clone(),
                    "Last Term" => self.last_term = matrix.clone(),
                    "Equal" => self.equal = matrix.clone(),
                    _ => panic!("[ Error ] Included too many matrices in input."),
                }
            } else {
                match matrix_name {
                    "yields" => self.yields = matrix.clone(),
                    "takes" => self.takes = matrix.clone(),
                    "equals" => self.equal = matrix.clone(),
                    v => panic!("[ Error ] Included too many matrices in input. {}", v),
                }
            }

            matrix.clear();
        }
    }

    fn shrink_precedence(&mut self) {
        self.create_b_matrix();
        let length = self.rtc.len();

        let mut count = 0;
        for row in 0..length {
            if row < (length / 2) {
                for col in 0..length {
                    if self.rtc[row][col] {
                        count += 1;
                    }
                }
                self.f.push(count);
            }

            if row >= (length / 2) {
                for col in 0..length {
                    if self.rtc[row][col] {
                        count += 1;
                    }
                }
                self.g.push(count);
            }
            count = 0;
        }
    }

    fn compute_handles(&mut self) {
        let mut final_handle: Matrix;

        let first_p = matrix::transitive_closure(&self.first);
        let identity = matrix::create_identity(10); 
        let first_s = matrix::sum(&identity, &first_p);
        
        final_handle = matrix::product(&self.equal, &first_s);
        final_handle = matrix::product(&final_handle, &self.first_term);
        self.yields = final_handle.clone();
        final_handle.clear();

        let last_p = matrix::transitive_closure(&self.last);
        let last_s = matrix::sum(&identity, &last_p);
        final_handle = matrix::product(&last_s, &self.last_term);
        final_handle = matrix::transpose(&final_handle);
        final_handle = matrix::product(&final_handle, &self.equal);
        self.takes = final_handle.clone();
    }
}

impl SPG {
    fn create_b_matrix(&mut self) {
        let b_matrix_len = self.m_dimension * 2;
        let mut b_matrix: Matrix = vec![vec![false; b_matrix_len]; b_matrix_len]; 

        let takes_equal = matrix::combine_matrix(&self.takes, &self.equal);
        let yields_equal = matrix::combine_matrix(&self.yields, &self.equal);

        let yields_equal = matrix::transpose(&yields_equal);

        for row in 0..b_matrix_len {
            for col in 0..b_matrix_len {
                if row < (b_matrix_len / 2) && col >= (b_matrix_len / 2) {
                    b_matrix[row][col] = takes_equal[row][col % (self.m_dimension)];
                }

                if row >= (b_matrix_len / 2) && col < (b_matrix_len / 2){
                    b_matrix[row][col] = yields_equal[row % (self.m_dimension)][col % (self.m_dimension)];
                }
            }
        }

        let identity = matrix::create_identity(b_matrix_len); 
        b_matrix = matrix::transitive_closure(&b_matrix);
        b_matrix = matrix::sum(&b_matrix, &identity); 
        self.rtc = b_matrix;
    }
}

impl PrecedenceGrammar for SPG {
    fn new() -> SPG {
        SPG {
            first: Vec::new(),
            last: Vec::new(),
            takes: Vec::new(),
            yields: Vec::new(),
            equal: Vec::new(),
            rtc: Vec::new(),
            f: Vec::new(),
            g: Vec::new(),
            m_dimension: 0
        }
    }

    fn parse_input(&mut self, compute_handles: bool) {
        let in_handle = io::stdin();
        let mut lines = in_handle.lock().lines();

        let mut matrix: Matrix = Vec::new();
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
                panic!("[ Error ] Please supply a valid square matrix.");
            }

            self.m_dimension = x as usize;

            let matrix_name: &str = &lines.next().unwrap().ok().unwrap().to_lowercase();

            for _ in 0..x {
                matrix.push(
                    lines
                        .next()
                        .unwrap()
                        .unwrap()
                        .trim()
                        .split_whitespace()
                        .map(|x| -> bool {
                            match x.parse::<u32>().ok().unwrap() {
                                0 => false,
                                1 => true,
                                _ => panic!("[ Error ] Ouch found something not allowed."),
                            }
                        })
                        .collect::<Vec<bool>>(),
                );
            }
           
            if compute_handles {
                match matrix_name {
                    "first" => self.first = matrix.clone(),
                    "last" => self.last = matrix.clone(),
                    "equal" => self.equal = matrix.clone(),
                    _ => panic!("[ Error ] Included too many matrices in input."),
                }
            } else {
                match matrix_name {
                    "yields" => self.yields = matrix.clone(),
                    "takes" => self.takes = matrix.clone(),
                    "equal" => self.equal = matrix.clone(),
                    _ => panic!("[ Error ] Included too many matrices in input."),
                }
            }

            matrix.clear();
        }
    } 

    fn shrink_precedence(&mut self) {
        self.create_b_matrix();
        let length = self.rtc.len();

        let mut count = 0;
        for row in 0..length {

            if row < (length / 2) {
                for col in 0..length {
                    if self.rtc[row][col] {
                        count += 1;
                    }
                }
                self.f.push(count);
                count = 0;
            }

            if row >= (length / 2) {
                for col in 0..length {
                    if self.rtc[row][col] {
                        count += 1;
                    }
                }
                self.g.push(count);
                count = 0;
            }
        }
    }

    fn compute_handles(&mut self) {
        let mut final_handle: Matrix;

        let first_p = matrix::transitive_closure(&self.first);

        final_handle = matrix::product(&self.equal, &first_p);

        self.yields = final_handle.clone();
        final_handle.clear();


        let last_p = matrix::transitive_closure(&self.last);
        let identity = matrix::create_identity(10);

        let first_s = matrix::sum(&identity, &first_p);
        let transposed = matrix::transpose(&last_p);

        final_handle = matrix::product(&transposed, &self.equal);
        final_handle = matrix::product(&final_handle, &first_s);

        self.takes = final_handle.clone();
    }
}
