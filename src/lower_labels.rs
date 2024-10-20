use std::collections::HashMap;

use crate::ast::{FuncBody, Opcode, Operand};

pub fn lower_labels(module: Vec<FuncBody>) -> Vec<FuncBody> {
    let mut label_map = HashMap::new();
    let mut pc = 0;
    for instr in module.iter() {
        match instr {
            FuncBody::Label(l) => {
                label_map.insert(l, pc);
            }
            FuncBody::Instruction(i) => {
                if i.opcode == Opcode::Brc {
                    pc += 3;
                    continue;
                }
                pc += i.operands.iter().fold(1, |acc, op| match op {
                    Operand::Label(_) | Operand::Immediate(_) | Operand::Address(_) => acc + 1,
                    _ => acc,
                });
            }
            FuncBody::Data(d) => {
                pc += d.len() as u8;
            }
        }
    }

    let mut ret = Vec::new();

    for instr in &module {
        match &instr {
            FuncBody::Label(_) => (),
            FuncBody::Instruction(i) => {
                let mut new_i = i.clone();
                for operand in new_i.operands.iter_mut() {
                    if let Operand::Label(l) = operand {
                        *operand = Operand::Immediate(
                            *label_map.get(l).expect("used a label that doesn't exist"),
                        );
                    }
                }
                ret.push(FuncBody::Instruction(new_i));
            }
            FuncBody::Data(_) => ret.push(instr.clone()),
        }
    }

    ret
}
