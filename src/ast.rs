use std::fmt::Display;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Nop = 0,
    Inc,
    Dec,
    Add,
    Adr,
    Neg,
    Sub,
    EMPTY,
    Cmp,
    Bor,
    And,
    Xor,
    Bsl,
    Bsr,
    Lia,
    Lir,
    Rst,
    Rld,
    Mst,
    Mld,
    Cal,
    Ret,
    Brc,
    Jid,
    Psh,
    Pop,
    Pst,
    Pld,
    Hlt,
    Ics
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Condition {
    Jmp,
    Even,
    Eq,
    Neq,
    Gt,
    Lt,
    Gteq,
    Lteq,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Immediate(u8),
    Condition(Condition),
    Label(String),
    Register(u8),
    Address(u8),
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: Vec<Operand>,
}

#[derive(Debug, Clone)]
pub enum FuncBody {
    Instruction(Instruction),
    Label(String),
    Data(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct TopLvl {
    pub functions: Vec<(String, Vec<FuncBody>)>,
    pub instrs: Vec<FuncBody>,
}

pub enum TopLvlEnum {
    Function(String, Vec<FuncBody>),
    Instruction(FuncBody),
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.opcode,
            self.operands
                .iter()
                .map(|e| format!("{e}"))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Address(a) => write!(f, "${a}"),
            Operand::Condition(cond) => write!(f, "{cond}"),
            Operand::Immediate(i) => write!(f, "{:#x}", i),
            Operand::Register(r) => write!(f, "r{r}"),
            Operand::Label(l) => write!(f, "{l}"),
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl Display for FuncBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FuncBody::Data(d) => {
                write!(f, "{:?}", d)
            }
            FuncBody::Instruction(i) => {
                write!(f, "{i}")
            }
            FuncBody::Label(l) => {
                write!(f, "{l}")
            }
        }
    }
}
