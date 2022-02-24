type Validtable = [ [States; 3]; 3];
type Symbols    = [String; 3];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum States {
    Start,
    Letter,
    Digit,
    LetterDigit,
    Finish
}

#[derive(Debug, PartialEq, Eq)]
pub struct Fsa {
    pub state_table: Validtable,
    pub symbols:     Symbols 
}

impl Fsa {
    pub fn new(table: Validtable, symbol_table: Symbols) -> Self {
        return Fsa {
            state_table: table,
            symbols:     symbol_table,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_just_works() {
        let table =  Fsa::new([
            [States::Letter, States::Letter, States::Letter],
            [States::Letter, States::Letter, States::Letter],
            [States::Letter, States::Letter, States::Letter],
        ],
        [
            String::from("var"),
            String::from("int"),
            String::from("double")
        ]);

        let valid_table: Validtable = [
            [States::Letter, States::Letter, States::Letter],
            [States::Letter, States::Letter, States::Letter],
            [States::Letter, States::Letter, States::Letter],
        ];

        let valid_symbols: Symbols = [String::from("var"), String::from("int"), String::from("double")];

        assert_eq!(table.state_table, valid_table);
        assert_eq!(table.symbols, valid_symbols);
    }

    #[test]
    fn state_loop() {
        // always passing test
        let table =  Fsa::new([
            [States::Letter, States::Digit, States::Start], // State 0
            [States::Letter, States::Letter, States::Finish], // State 1
            [States::Start, States::Digit, States::Finish], // State 2
        ],
        [
            String::from("<Identifier>"),
            String::from("<Literal>"),
            String::from("<PlaceHolder>")
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
