type Validtable = [ [i16; 3]; 5];
type Symbols    = [String; 3];

#[derive(Debug, PartialEq, Eq)]
pub struct Fsa {
    state_table: Validtable,
    symbols: Symbols 
}

impl Fsa {
    pub fn new(table: Validtable, symbol_table: Symbols) -> Self {
        return Fsa {
            state_table: table,
            symbols: symbol_table,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_just_works() {
        let table =  Fsa::new([
            [1, 1, 1],
            [1, 1, 1],
            [1, 1, 1],
            [1, 1, 1],
            [1, 1, 1]
        ],
        [
            String::from("var"),
            String::from("int"),
            String::from("double")
        ]);

        let valid_table: Validtable = [[1,1,1],[1,1,1],[1,1,1], [1, 1, 1], [1, 1, 1]];
        let valid_symbols: Symbols = [String::from("var"), String::from("int"), String::from("double")];

        assert_eq!(table.state_table, valid_table);
        assert_eq!(table.symbols, valid_symbols);
    }
}
