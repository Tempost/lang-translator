use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::vec::IntoIter;

use crate::compiler::lexical::TokenClass;
use crate::compiler::syntax::{Quad, QuadList};

type Result<T> = std::result::Result<T, GeneratorErr>;

#[derive(Debug, PartialEq, Eq)]
pub struct GeneratorErr(Quad);

struct Generator {
    quads: IntoIter<Quad>,
    asm_file: File,
}

impl<'a> fmt::Display for GeneratorErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Generator {
    fn new(quads: QuadList) -> Self {
        if Path::new("code.asm").exists() {
            fs::remove_file("code.asm").unwrap();
        }

        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open("code.asm")
            .unwrap();

        Generator {
            quads: quads.into_iter(),
            asm_file: file,
        }
    }

    fn consume_quads(&mut self) -> Result<()> {
        // let label_loc = 0;
        // let fix_up: Vec<(i32, &str)> = Vec::new();

        // Match on the different operators
        // output the assembly to a file
        while let Some(quad) = self.quads.next() {
            match quad.op.class {
                TokenClass::ReservedWord => todo!(),
                TokenClass::Op => match quad.op.name.as_str() {
                    "+" => {
                        let res = self.asm_file.write_fmt(format_args!(
                            "mov ax,[{}]\nadd ax,[{}]\nmov [{}],ax\n",
                            quad.param_two.name, quad.param_one.name, quad.temp.name
                        ));

                        if res.is_err() {
                            return Err(GeneratorErr(quad));
                        }
                    }

                    "-" => {
                        let res = self.asm_file.write_fmt(format_args!(
                            "mov ax,[{}]\nsub ax,[{}]\nmov [{}],ax\n",
                            quad.param_two.name, quad.param_one.name, quad.temp.name
                        ));

                        if res.is_err() {
                            return Err(GeneratorErr(quad));
                        }
                    }

                    "/" => {
                        let res = self.asm_file.write_fmt(format_args!(
                            "mov dx,0\nmov ax,[{}]\nmov bx,[{}]\ndiv bx\nmov [{}],ax\n",
                            quad.param_two.name, quad.param_one.name, quad.temp.name
                        ));

                        if res.is_err() {
                            return Err(GeneratorErr(quad));
                        }
                    }

                    "*" => {
                        let res = self.asm_file.write_fmt(format_args!(
                            "mov ax,[{}]\nmov bx,[{}]\nmul bx\nmov [{}],ax\n",
                            quad.param_two.name, quad.param_one.name, quad.temp.name
                        ));

                        if res.is_err() {
                            return Err(GeneratorErr(quad));
                        }
                    }

                    "=" => {
                        let res = self.asm_file.write_fmt(format_args!(
                            "mov ax,[{}]\nmov [{}],ax\n",
                            quad.param_one.name, quad.param_two.name
                        ));

                        if res.is_err() {
                            return Err(GeneratorErr(quad));
                        }
                    }

                    e => {
                        if Path::new("code.asm").exists() {
                            fs::remove_file("code.asm").unwrap();
                        }
                        panic!("[ Error ] Not a valid operator, {}", e)
                    }
                },
                TokenClass::BoolExp => todo!(),
                TokenClass::RelationOp => todo!(),
                TokenClass::Unknown => todo!(),
                _ => {
                    if Path::new("code.asm").exists() {
                        fs::remove_file("code.asm").unwrap();
                    }
                    panic!("[ Error ] Some how this made it past syntax analysis?")
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::compiler::syntax::Syntax;
    use std::any::type_name;

    #[test]
    fn new_file() {
        fn type_of<T>(_: T) -> &'static str {
            type_name::<T>()
        }

        let file = File::open("code.asm").unwrap();
        let gen = Generator::new(Vec::new());
        assert_eq!(type_of(file), type_of(gen.asm_file));
    }

    #[test]
    fn getting_data() {
        let mut syn = Syntax::new("test2.java", true);
        syn.complete_analysis();
        syn.consume_polish().unwrap();

        let mut gen = Generator::new(syn.quads);
        let res = gen.consume_quads();
        let check_res = res.is_ok();

        assert!(check_res);
    }
}
