pub type Matrix = Vec<Vec<bool>>;

pub fn product(m1: &Matrix, m2: &Matrix) -> Matrix {
    if m1.len() != m2.len() {
        panic!("[ ERROR ] Trying to multiply non-square matrices.");
    }

    let length = m1.len();
    let mut product: Matrix = vec![vec![false; length]; length];

    for row in 0..length {
        for col in 0..length {
            for k in 0..length {
                product[row][col] |= m1[row][k] && m2[k][col];
            }
        }
    }
    product
}

pub fn sum(m1: &Matrix, m2: &Matrix) -> Matrix {
    if m1.len() != m2.len() {
        panic!("[ ERROR ] Trying to sum non-square matrices.");
    }

    let length = m1.len();
    let mut sum: Matrix = vec![vec![false; length]; length];

    for row in 0..length {
        for col in 0..length {
            sum[row][col] = m1[row][col] || m2[row][col]
        }
    }
    sum
}

pub fn transitive_closure(m: &Matrix) -> Matrix {
    let length = m.len();
    let mut r_plus = m.clone();

    for row in 0..length {
        for col in 0..length {
            if r_plus[col][row] {
                for k in 0..length {
                    r_plus[col][k] = r_plus[col][k] || r_plus[row][k];
                }
            }
        }
    }
    r_plus
}

pub fn transpose(m: &Matrix) -> Matrix {
    let length = m.len();
    let mut final_matrix: Matrix = vec![vec![false; length]; length];

    for row in 0..length {
        for col in 0..length {
            final_matrix[row][col] = m[col][row];
        }
    }
    final_matrix
}

pub fn combine_matrix(m1: &Matrix, m2: &Matrix) -> Matrix {
    if m1.len() != m2.len() {
        panic!("[ ERROR ] Trying to combine non-square matrices.");
    }

    let length = m1.len();
    let mut final_matrix: Matrix = vec![vec![false; length]; length];

    for row in 0..length {
        for col in 0..length {
            final_matrix[row][col] = m1[row][col] | m2[row][col];
        }
    }

    final_matrix
}

pub fn print_matrix(m: &Matrix) {
    let mut iter = m.iter();

    while let Some(row) = iter.next() {
        let parsed = row
            .iter()
            .map(|x| match x {
                true => 1,
                false => 0,
            })
            .collect::<Vec<i32>>();

        println!("{:?}", parsed);
    }
}

pub fn create_identity(size: usize) -> Matrix{
    let mut identity: Matrix = vec![vec![false; size]; size];
    
    for row in 0..size {
        for col in 0..size {
            if row == col {
                identity[row][col] = true;
            }
        }
    }

    identity
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transpose_works() {
        let matrix = vec![
            vec![false, true, true],
            vec![true, true, true],
            vec![false, false, false],
        ];

        let t_matrix = vec! [
            vec![false, true, false],
            vec![true, true, false],
            vec![true, true, false]
        ];

        let transposed = transpose(&matrix);
        assert_eq!(t_matrix, transposed);

    }

    #[test]
    fn identity_works() {
        print_matrix(&create_identity(16));
    }

    #[test]
    fn sum_works() {
        let m1 = vec![
            vec![false, false],
            vec![true, true],
        ];

        let m2  = vec! [
            vec![true, false],
            vec![true, false],
        ];

        print_matrix(&sum(&m1, &m2));
    }
}
