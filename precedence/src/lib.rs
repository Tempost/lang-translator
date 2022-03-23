use std::io::{self, BufRead};

mod boolean;

use crate::boolean::matrix::{self, Matrix};

trait PrecedenceGrammar {
    fn new() -> Self;
    fn parse_input(&mut self, compute_handles: bool);
    fn shrink_precedence(&mut self);
    fn compute_handles(&mut self);
}

struct OPG {
    first: Matrix,
    first_term: Matrix,
    last: Matrix,
    last_term: Matrix,
    takes: Matrix,
    yields: Matrix,
    equal: Matrix,
    rtc: Matrix,
    f: Vec<i32>,
    g: Vec<i32>,
    m_dimension: usize,
}

struct SPG {
    first: Matrix,
    last: Matrix,
    takes: Matrix,
    yields: Matrix,
    equal: Matrix,
    rtc: Matrix,
    f: Vec<i32>,
    g: Vec<i32>,
    m_dimension: usize,
}

impl OPG {
    // NOTE: take the (<=) matrix and add to pos [0][7] through pos [7][15]
    // also take (>=)T matrix and add to pos [8][0] through pos [15][7]
    // to create the B matrix
    fn create_b_matrix(&self) -> Matrix{
        let b_matrix_len = self.m_dimension * 2;
        let mut b_matrix: Matrix = vec![vec![false; b_matrix_len]; b_matrix_len]; 

        let takes_equal = matrix::combine_matrix(&self.takes, &self.equal);
        let yields_equal = matrix::combine_matrix(&self.yields, &self.equal);

        // Shadowing takes_equal to perform the transpose
        let takes_equal = matrix::transpose(&takes_equal);

        for row in 0..b_matrix_len {
            for col in 0..b_matrix_len {
                if row < (b_matrix_len / 2) && col >= (b_matrix_len / 2) {
                    b_matrix[row][col] = yields_equal[row][col % (self.m_dimension)];
                }

                if row >= (b_matrix_len / 2) && col < (b_matrix_len / 2){
                    b_matrix[row][col] = takes_equal[row % (self.m_dimension)][col % (self.m_dimension)];
                }
            }
        }

        b_matrix
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
        let in_handle = io::stdin();
        let mut lines = in_handle.lock().lines();

        let mut matrix: Matrix = Vec::new();
        let mut count: i32 = 0;
        while let Some(line) = lines.next() {
            let line_ref = line.unwrap();

            if line_ref.is_empty() {
                continue;
            }

            let mut m_size = line_ref.split(' ');

            let (x, y) = (
                m_size.next().unwrap().parse::<i32>().ok().unwrap(),
                m_size.next().unwrap().parse::<i32>().ok().unwrap(),
            );

            if x != y {
                panic!("Please supply a valid square matrix.");
            }

            self.m_dimension = x as usize;

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
                match count {
                    0 => self.first = matrix.clone(),
                    1 => self.first_term = matrix.clone(),
                    2 => self.last = matrix.clone(),
                    3 => self.last_term = matrix.clone(),
                    4 => self.equal = matrix.clone(),
                    _ => panic!("[ ERROR ] Included too many matrices in input."),
                }
            } else {
                match count {
                    0 => self.takes = matrix.clone(),
                    1 => self.yields = matrix.clone(),
                    2 => self.equal = matrix.clone(),
                    _ => panic!("[ ERROR ] Included too many matrices in input."),
                }
            }

            matrix.clear();
            count += 1;
        }
    }

    fn shrink_precedence(&mut self) {
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
        let identity = matrix::create_identity(10); 
        let first_s = matrix::sum(&identity, &first_p);
        
        final_handle = matrix::product(&self.equal, &first_s);
        final_handle = matrix::product(&final_handle, &self.first_term);
        self.takes = final_handle.clone();
        final_handle.clear();

        let last_p = matrix::transitive_closure(&self.last);
        let last_s = matrix::sum(&identity, &last_p);
        final_handle = matrix::product(&last_s, &self.last_term);
        final_handle = matrix::transpose(&final_handle);
        final_handle = matrix::product(&final_handle, &self.equal);
        self.yields = final_handle.clone();
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
        let mut count: i32 = 0;
        while let Some(line) = lines.next() {
            let line_ref = line.unwrap();

            if line_ref.is_empty() {
                continue;
            }

            let mut m_size = line_ref.split(' ');

            let (x, y) = (
                m_size.next().unwrap().parse::<i32>().ok().unwrap(),
                m_size.next().unwrap().parse::<i32>().ok().unwrap(),
            );

            if x != y {
                panic!("Please supply a valid square matrix.");
            }

            self.m_dimension = x as usize;

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
                match count {
                    0 => self.first = matrix.clone(),
                    1 => self.last = matrix.clone(),
                    2 => self.equal = matrix.clone(),
                    _ => panic!("[ ERROR ] Included too many matrices in input."),
                }
            } else {
                match count {
                    0 => self.takes = matrix.clone(),
                    1 => self.yields = matrix.clone(),
                    2 => self.equal = matrix.clone(),
                    _ => panic!("[ ERROR ] Included too many matrices in input."),
                }
            }
            matrix.clear();
            count += 1;
        }
    } 

    fn shrink_precedence(&mut self) {
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

        self.takes = final_handle.clone();
        final_handle.clear();


        let last_p = matrix::transitive_closure(&self.last);
        let identity = matrix::create_identity(10);

        let first_s = matrix::sum(&identity, &first_p);
        let transpose = matrix::transpose(&last_p);

        final_handle = matrix::product(&transpose, &self.equal);
        final_handle = matrix::product(&final_handle, &first_s);

        self.yields = final_handle.clone();
    }
}
