use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
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

        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open("code.asm")
            .unwrap();

        file.write_fmt(format_args!(
            "sys_exit equ 1\nsys_read equ 3\nsys_write equ 4\nstdin equ 0\nstdout equ 1\n.DATA\n"
        ))
        .unwrap();
        Generator::init_asm_file(&mut file).unwrap();

        Generator {
            quads: quads.into_iter(),
            asm_file: file,
        }
    }

    fn init_asm_file(asm_file: &mut File) -> Result<()> {
        // open symbol table
        // init variables
        // Table driven?
        let sym_file = File::open("symbols");
        match sym_file {
            Ok(file) => {
                let buf = BufReader::new(file);
                let mut lines = buf.lines();

                while let Some(result) = lines.next() {
                    let mut line_vec: Vec<&str>;
                    if let Ok(line) = result {
                        line_vec = Vec::from_iter(line.split_whitespace());
                        if line_vec[1].eq("Identifier") {
                            asm_file
                                .write_fmt(format_args!("{:<5} DW {}\n", line_vec[0], line_vec[2]))
                                .unwrap();
                        }
                    }
                }
                asm_file
                    .write_fmt(format_args!(
                        "section .bss\n\tblen equ 6\n\tbuffer resb blen\nsection .text\n\tglobal _start\n_start: nop\n"
                    ))
                    .unwrap();
                Ok(())
            }
            Err(e) => panic!("{}", e),
        }
    }

    pub fn consume_quads(&mut self) -> Result<()> {
        // let label_loc = 0;
        // let fix_up: Vec<(i32, &str)> = Vec::new();

        // Match on the different operators
        // output the assembly to a file
        let mut io_flag = false;
        while let Some(quad) = self.quads.next() {
            match quad.op.class {
                TokenClass::ReservedWord => match quad.op.name.as_str() {
                    "GET" => {
                        io_flag = true;
                        let res = self.asm_file.write_fmt(format_args!("call GetInput\n"));

                        if res.is_err() {
                            return Err(GeneratorErr(quad));
                        }
                    }

                    "PUT" => {
                        io_flag = true;
                        let res = self.asm_file.write_fmt(format_args!("call Print\n"));

                        if res.is_err() {
                            return Err(GeneratorErr(quad));
                        }
                    }

                    _ => todo!(),
                },

                TokenClass::Op => match quad.op.name.as_str() {
                    "+" => {
                        let res = self.asm_file.write_fmt(format_args!(
                            "\tmov ax,[{}]\n\tadd ax,[{}]\n\tmov [{}],ax\n",
                            quad.param_two.name, quad.param_one.name, quad.temp.name
                        ));

                        if res.is_err() {
                            return Err(GeneratorErr(quad));
                        }
                    }

                    "-" => {
                        let res = self.asm_file.write_fmt(format_args!(
                            "\tmov ax,[{}]\n\tsub ax,[{}]\n\tmov [{}],ax\n",
                            quad.param_two.name, quad.param_one.name, quad.temp.name
                        ));

                        if res.is_err() {
                            return Err(GeneratorErr(quad));
                        }
                    }

                    "/" => {
                        let res = self.asm_file.write_fmt(format_args!(
                            "\tmov dx,0\n\tmov ax,[{}]\n\tmov bx,[{}]\n\tdiv bx\n\tmov [{}],ax\n",
                            quad.param_two.name, quad.param_one.name, quad.temp.name
                        ));

                        if res.is_err() {
                            return Err(GeneratorErr(quad));
                        }
                    }

                    "*" => {
                        let res = self.asm_file.write_fmt(format_args!(
                            "\tmov ax,[{}]\n\tmov bx,[{}]\n\tmul bx\n\tmov [{}],ax\n",
                            quad.param_two.name, quad.param_one.name, quad.temp.name
                        ));

                        if res.is_err() {
                            return Err(GeneratorErr(quad));
                        }
                    }

                    "=" => {
                        let res = self.asm_file.write_fmt(format_args!(
                            "\tmov ax,[{}]\n\tmov [{}],ax\n",
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

        if io_flag {
            self.write_io();
        }

        Ok(())
    }

    fn write_io(&mut self) {
        self.asm_file
            .write_fmt(format_args!(
                "GetInput:\nmov eax, 3\nmov ebx, 2\nmov ecx, buffer\nmov edx, blen\nint 80h\n"
            ))
            .unwrap();

        self.asm_file.write_fmt(format_args!("Print:\npush ax\npush dx\nmov eax, 4\nmov ebx, 1\nmov ecx")).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::compiler::syntax::Syntax;

    #[test]
    fn test_program1() {
        let mut syn = Syntax::new("test1.java", true);
        syn.create_symbol_table("symbols1");
        syn.complete_analysis();
        syn.consume_polish().unwrap();

        let mut gen = Generator::new(syn.quads);
        let res = gen.consume_quads();
        let check_res = res.is_ok();

        assert!(check_res);
    }

    #[test]
    fn test_program2() {
        let mut syn = Syntax::new("test2.java", true);
        syn.create_symbol_table("symbols2");
        syn.complete_analysis();
        syn.consume_polish().unwrap();

        let mut gen = Generator::new(syn.quads);
        let res = gen.consume_quads();
        let check_res = res.is_ok();

        assert!(check_res);
    }
}
