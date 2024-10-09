use std::time::Instant;

use crate::{
    bytecode::{Bytecode, Constant, Instruction, OpCode, OpMode, Proto},
    util::util::format_time_taken,
};

// TODO: make this similar to Luau (have locals & not just a string; then we can use this for both register and upvalue assignments?) ?

struct RegisterMove {
    destination: u8,
    source: u8,
}
struct RegisterAssignment {
    register: u8,
    value: Constant,
    is_first: bool,
}

enum Statement {
    Unknown(OpCode),
    RegisterMove(RegisterMove),
    RegisterAssignment(RegisterAssignment),
}

pub struct Decompiler {
    bytecode: Bytecode,
    indent: u8,
}
pub fn build_decompiler(bytecode: Bytecode) -> Decompiler {
    Decompiler {
        bytecode,
        indent: 0,
    }
}

impl Decompiler {
    fn format_text(&self, text: String) -> String {
        format!("{}{}", ("    ").repeat(self.indent.into()), text)
    }

    pub fn decompile(&mut self) -> String {
        let start_instant: Instant = Instant::now();

        let time_taken = start_instant.elapsed();

        format!(
            "-- decompiled by inu in {}\n\n{}",
            format_time_taken!(time_taken),
            self.decompile_proto(self.bytecode.main_proto.clone())
        )
    }

    fn decompile_proto(&mut self, proto: Proto) -> String {
        if !proto.is_main {
            self.indent += 1;
        }

        let code_len: usize = proto.code.len();
        let mut statement_list: Vec<Statement> = Vec::with_capacity(code_len);

        for i in 0..code_len {
            let inst: &Instruction = &proto.code[i];

            // FIXME: keep track of registers for a proper RegisterAssigment is_first
            let statement = match inst.op {
                OpCode::OpMove(OpMode::ABC(a, b, _c)) => Statement::RegisterMove(RegisterMove {
                    destination: a as u8,
                    source: b as u8,
                }),
                OpCode::OpLoadK(OpMode::ABX(a, bx)) => {
                    Statement::RegisterAssignment(RegisterAssignment {
                        register: a as u8,
                        value: proto.constants[bx as usize].clone(),
                        is_first: true,
                    })
                }
                _ => Statement::Unknown(inst.op.clone()),
            };
            statement_list.push(statement);
        }

        let mut result: String = String::new();

        // convert statements
        for statement in statement_list {
            result += match statement {
                Statement::Unknown(op) => format!("-- failed to generate statement for {:?}", op),
                Statement::RegisterMove(statement) => {
                    format!("r_{} = r_{}", statement.destination, statement.source)
                }
                Statement::RegisterAssignment(statement) => {
                    format!(
                        "{}r_{} = {}",
                        if statement.is_first { "local " } else { "" },
                        statement.register,
                        statement.value.format()
                    )
                }
            }
            .as_str();
            result.push('\n');
        }

        if !proto.is_main {
            self.indent -= 1;
        }

        result
    }
}
