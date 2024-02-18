use crate::ast::{FuncBody, Operand};

pub fn assemble(instrs: Vec<FuncBody>) -> Vec<u8> {
    let mut ret = Vec::new();
    for b in instrs {
        match b {
            FuncBody::Instruction(instr) => {
                let mut word = instr.opcode as u8;
                let mut pushed_opword = false;
                for operand in instr.operands {
                    match operand {
                        Operand::Address(a) | Operand::Immediate(a) => {
                            if !pushed_opword {
                                ret.push(word);
                            }
                            pushed_opword = true;
                            ret.push(a);
                        }
                        Operand::Condition(c) => {
                            let w = (c as u8) << 5;
                            word |= w;
                            pushed_opword = true;
                            ret.push(word);
                        }
                        Operand::Register(r) => {
                            let w = r << 5;
                            word |= w;
                            pushed_opword = true;
                            ret.push(word);
                        }
                        _ => unreachable!(),
                    }
                }
                if !pushed_opword {
                    ret.push(word);
                }
            }
            FuncBody::Data(d) => {
                ret.extend(d);
            }
            _ => unreachable!()
        }
    }
    ret
}
