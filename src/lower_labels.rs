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
                
                // Start with 1 byte for the instruction word
                let mut size = 1;
                
                // Check if any operand requires additional bytes
                for operand in &i.operands {
                    match operand {
                        Operand::Immediate(_) | Operand::Label(_) => {
                            // Immediate values and labels (which become immediates) add 1 byte
                            // but only for non-BRC instructions
                            size += 1;
                        }
                        Operand::Register(_) | Operand::Condition(_) | Operand::Address(_) => {
                            // These are encoded in the instruction word bits, no extra bytes
                        }
                    }
                }
                pc += size;
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
