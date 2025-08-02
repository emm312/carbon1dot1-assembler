pub mod assembler;
pub mod ast;
pub mod disassembler;
pub mod lower_labels;
pub mod name_mangling;
pub mod parser;
pub mod romgen;

#[macro_export]
macro_rules! instr {
    ($name:ident; $ops:expr) => {{
        use $crate::ast::*;
        Instruction {
            opcode: Opcode::$name,
            operands: $ops,
        }
    }};
}
