// TODO: Refactor Validtable to hold terminal enum
type Validtable = [ [i32; 11]; 16];

#[derive(Debug, Clone, Copy)]
pub enum Terminals {
    Letter = 0,
    Digit = 1,
    LBracket = 2,
    RBracket = 3,
    Mop = 4,
    Addop = 5,
    Assignment = 6,
    Semi = 7,
    Comma = 8,
    FSlash = 9,
    Whitespace = 10
}

#[derive(Debug, PartialEq, Eq)]
pub struct Fsa {
    pub state_table: Validtable,
}

impl Fsa {
    pub fn new(table: Validtable) -> Self {
        return Fsa {
            state_table: table,
        }
    }
}
// table[state][terminal]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_loop() {
        // NOTE: What do I do for end states?
        let table =  Fsa::new([
            [ 1,  3,  5,  6,  7,  8,  9, 10, 11, 12,  0],
            [ 1,  1,  2,  2,  2,  2,  2,  2,  2,  2,  2],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [ 4,  3,  4,  4,  4,  4,  4,  4,  4,  4,  4],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [13, 13, 13, 13, 14, 13, 13, 13, 13, 13, 13],
            [ 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0],
            [14, 14, 14, 14, 15, 14, 14, 14, 14, 14, 14],
            [14, 14, 14, 14, 14, 14, 14, 14, 14,  0, 14]
        ]);

        let mut curr = 0;
        for state in table.state_table {
            print!("current index: {} == ", curr);
            for to_state in state {
                print!("{:?},", to_state);
            }
            curr += 1;
            println!("\n");
        }
    }
}
