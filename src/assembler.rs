use crate::ast::{FuncBody, Opcode, Operand};

pub fn assemble(instrs: Vec<FuncBody>) -> Vec<u8> {
    let mut ret = Vec::new();
    for b in instrs {
        match b {
            FuncBody::Instruction(instr) => {
                let mut word = (instr.opcode as u8) << 3;
                let mut pushed_opword = false;
                for operand in instr.operands {
                    match operand {
                        Operand::Immediate(a) => {
                            match instr.opcode {
                                Opcode::Brc => {
                                    ret.push(a >> 7);
                                    ret.push(a & 0b0111_1111);
                                }
                                Opcode::Bsl | Opcode::Bsr => {
                                    word |= a;
                                    pushed_opword = true;
                                    ret.push(word);
                                }
                                _ => {
                                    if !pushed_opword {
                                        ret.push(word);
                                    }
                                    pushed_opword = true;
                                    ret.push(a);
                                }
                            }
                        }
                        Operand::Condition(c) => {
                            word |= c as u8;
                            pushed_opword = true;
                            ret.push(word);
                        }
                        Operand::Register(r) | Operand::Address(r) => {
                            word |= r;
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
            _ => unreachable!(),
        }
    }
    ret
}
