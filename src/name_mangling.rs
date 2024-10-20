use crate::{
    ast::{FuncBody, Operand, TopLvl},
    instr,
};

/// this function lowers functions & correctly mangles the identifiers
pub fn mangle(module: TopLvl) -> Vec<FuncBody> {
    let mut ret = Vec::new();
    ret.extend(module.instrs);
    ret.push(FuncBody::Instruction(instr!(Hlt; vec![])));
    for (name, func) in module.functions {
        ret.push(FuncBody::Label(name.clone()));
        for instr in func {
            match instr {
                FuncBody::Instruction(mut instr) => {
                    for op in instr.operands.iter_mut() {
                        if let Operand::Label(l) = op {
                            *l = format!(".__INNER_FUNC_LABEL{}_{}", name, l);
                        }
                    }
                    //let mut push_nop = false;
                    //if instr.opcode == Opcode::Brc {
                    //    push_nop = true;
                    //}
                    ret.push(FuncBody::Instruction(instr));
                    //if push_nop {
                    //    ret.push(FuncBody::Instruction(instr!(Nop; vec![])));
                    //}
                }
                FuncBody::Label(l) => ret.push(FuncBody::Label(format!(
                    ".__INNER_FUNC_LABEL{}_{}",
                    name, l
                ))),
                _ => (),
            }
        }
    }
    ret
}
