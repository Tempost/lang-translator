use std::fmt;

type Result<'a, T> = std::result::Result<T, GeneratorErr<'a>>;

#[derive(Debug, PartialEq, Eq)]
pub struct GeneratorErr<'a>(&'a str, AsmSnippet);

struct Generator {}

#[derive(Debug, PartialEq, Eq)]
struct AsmSnippet {}

impl<'a> fmt::Display for GeneratorErr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ Error ] {} : {:?}", self.0, self.1)
    }
}

impl Generator {
    fn consume_polish(&mut self, file: &str) -> Result<AsmSnippet> {
        Ok(AsmSnippet {})
    }
}
