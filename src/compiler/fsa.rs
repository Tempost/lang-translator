pub type ValidTable = Vec<Vec<i32>>;

pub struct Fsa<'a> {
    pub table: &'a ValidTable,
}

impl<'a> Fsa<'a> {
    pub fn define_table(table: &'a ValidTable) -> Self {
        Fsa { table }
    }
}
