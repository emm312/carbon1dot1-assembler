use crate::ast::Opcode;

pub fn disassemble(binary_data: &[u8]) -> Vec<String> {
    let mut result = Vec::new();
    let mut pc = 0; // Program counter (address)
    
    while pc < binary_data.len() {
        let byte = binary_data[pc];
        let opcode_num = (byte >> 3) & 0b11111; // Extract bits 7-3
        let operand_bits = byte & 0b111; // Extract bits 2-0
        
        // Convert numeric opcode to enum
        let opcode = match opcode_num {
            0 => Opcode::Nop,
            1 => Opcode::Inc,
            2 => Opcode::Dec,
            3 => Opcode::Add,
            4 => Opcode::Adr,
            5 => Opcode::Neg,
            6 => Opcode::Sub,
            7 => Opcode::Bsb,
            8 => Opcode::Cmp,
            9 => Opcode::Bor,
            10 => Opcode::And,
            11 => Opcode::Xor,
            12 => Opcode::Bsl,
            13 => Opcode::Bsr,
            14 => Opcode::Lim,
            15 => Opcode::Lim,
            16 => Opcode::Rst,
            17 => Opcode::Rld,
            18 => Opcode::Mst,
            19 => Opcode::Mld,
            20 => Opcode::Cal,
            21 => Opcode::Ret,
            22 => Opcode::Brc,
            23 => Opcode::Jid,
            24 => Opcode::Psh,
            25 => Opcode::Pop,
            26 => Opcode::Pst,
            27 => Opcode::Psi,
            28 => Opcode::Pld,
            29 => Opcode::Prd,
            30 => Opcode::Hlt,
            31 => Opcode::Fls,
            _ => {
                result.push(format!("{:04X}: {:08b} INVALID_OPCODE", pc, byte));
                pc += 1;
                continue;
            }
        };
        
        let mut instruction = format!("{:?}", opcode).to_lowercase();
        let start_pc = pc;
        pc += 1;
        
        // Handle different instruction types based on opcode
        match opcode {
            Opcode::Nop | Opcode::Ret | Opcode::Hlt | Opcode::Fls => {
                // No operands
            }
            Opcode::Inc | Opcode::Dec | Opcode::Neg | Opcode::Rst | Opcode::Rld | 
            Opcode::Mst | Opcode::Mld | Opcode::Psh | Opcode::Pop => {
                // Register operand
                instruction.push_str(&format!(" r{}", operand_bits));
            }
            Opcode::Add | Opcode::Adr | Opcode::Sub | Opcode::Bsb | Opcode::Cmp | 
            Opcode::Bor | Opcode::And | Opcode::Xor => {
                // Register operand only (no immediates)
                instruction.push_str(&format!(" r{}", operand_bits));
            }
            Opcode::Bsl | Opcode::Bsr => {
                // Immediate operand (shift amount) stored in operand_bits
                instruction.push_str(&format!(" {}", operand_bits));
            }
            Opcode::Lim => {
                // Register + immediate
                instruction.push_str(&format!(" r{}", operand_bits));
                if pc < binary_data.len() {
                    let immediate = binary_data[pc];
                    instruction.push_str(&format!(" {:#x}", immediate));
                    pc += 1;
                }
            }
            Opcode::Pst | Opcode::Psi | Opcode::Pld | Opcode::Prd => {
                // Address operand
                instruction.push_str(&format!(" ${}", operand_bits));
                if pc < binary_data.len() && has_immediate_operand(&binary_data, start_pc) {
                    let immediate = binary_data[pc];
                    instruction.push_str(&format!(" {:#x}", immediate));
                    pc += 1;
                }
            }
            Opcode::Brc => {
                // Condition + 16-bit address
                let condition = match operand_bits {
                    0 => "jmp",
                    1 => "even", 
                    2 => "eq",
                    3 => "neq",
                    4 => "gt",
                    5 => "lt",
                    6 => "gteq",
                    7 => "lteq",
                    _ => "unknown"
                };
                instruction.push_str(&format!(" {}", condition));
                
                // BRC has special 16-bit address encoding
                if pc + 1 < binary_data.len() {
                    let addr_high = binary_data[pc] as u16;
                    let addr_low = binary_data[pc + 1] as u16;
                    let address = (addr_high << 7) | addr_low;
                    instruction.push_str(&format!(" {:#x}", address));
                    pc += 2;
                }
            }
            Opcode::Cal | Opcode::Jid => {
                // These might have special handling - need to check assembler logic
                if pc < binary_data.len() {
                    let immediate = binary_data[pc];
                    instruction.push_str(&format!(" {:#x}", immediate));
                    pc += 1;
                }
            }
        }
        
        // Format the output line
        let binary_str = if pc - start_pc == 1 {
            format!("{:08b}", binary_data[start_pc])
        } else if pc - start_pc == 2 {
            format!("{:08b} {:08b}", binary_data[start_pc], binary_data[start_pc + 1])
        } else if pc - start_pc == 3 {
            format!("{:08b} {:08b} {:08b}", 
                binary_data[start_pc], 
                binary_data[start_pc + 1], 
                binary_data[start_pc + 2])
        } else {
            format!("{:08b}", binary_data[start_pc])
        };
        
        result.push(format!("{:04X}: {} {}", start_pc, binary_str, instruction));
    }
    
    result
}

// Helper function to determine if an instruction has an immediate operand
// This is a simplified heuristic - might need refinement based on actual usage
fn has_immediate_operand(binary_data: &[u8], pc: usize) -> bool {
    if pc + 1 >= binary_data.len() {
        return false;
    }
    
    let current_byte = binary_data[pc];
    let next_byte = binary_data[pc + 1];
    let opcode_num = (current_byte >> 3) & 0b11111;
    let next_opcode_num = (next_byte >> 3) & 0b11111;
    
    // If the next byte looks like a valid opcode, probably no immediate
    // If it doesn't, it's likely an immediate value
    // This heuristic works because immediate values are often smaller numbers
    // while opcodes are in the range 0-31 and when shifted would create larger numbers
    
    // Simple heuristic: if next byte has high bits set in a way that suggests it's an opcode
    if next_opcode_num <= 31 && (next_byte & 0b11111000) != 0 {
        // Could be an opcode, check if it makes sense
        match opcode_num {
            3..=13 => next_byte < 32, // Add, Adr, Sub, etc. might have immediates
            14 => true, // Lim always has immediate
            26..=29 => next_byte < 32, // Pst, Psi, Pld, Prd might have immediates
            _ => false,
        }
    } else {
        false
    }
}

pub fn disassemble_from_file(filename: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;
    let binary_data: Result<Vec<u8>, _> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| u8::from_str_radix(line.trim(), 2))
        .collect();
    
    match binary_data {
        Ok(data) => Ok(disassemble(&data)),
        Err(e) => Err(format!("Failed to parse binary data: {}", e).into()),
    }
}