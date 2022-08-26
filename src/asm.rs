use crate::opcode::Opcode;
use crate::opcode::ALLCMDS;
fn filter(code: &str) -> String {
    let only_commands: Vec<_> = code.bytes().filter(|byte| ALLCMDS.contains(byte)).collect();
    let code = String::from_utf8(only_commands).unwrap();
    code
}

pub fn assemble(code: &str) -> Result<Vec<Opcode>, String> {
    let code = filter(code);
    let mut codes = Vec::new();
    let mut stack = Vec::new();

    for (index, byte) in code.bytes().enumerate() {
        let code = Opcode::from(byte);
        let code = match code {
            Opcode::JumpIn(_) => {
                stack.push(index);
                code
            }
            Opcode::JumpOut(_) => match stack.pop() {
                Some(location) => Opcode::JumpOut(location as u32),
                _ => return Err(format!("'[' not found for ']' at: {index}")),
            },
            other => other,
        };
        codes.push(code)
    }

    if !stack.is_empty() {
        return Err(format!(
            "']' not found for '[' at: {}",
            stack.pop().unwrap()
        ));
    };

    let mut backlocations = std::collections::HashMap::new();
    for (index, code) in codes.iter().enumerate() {
        if let Opcode::JumpOut(location) = code {
            backlocations.insert(*location, index);
        }
    }

    for (in_location, jumplocation) in backlocations {
        codes[in_location as usize] = Opcode::JumpIn(jumplocation as u32);
    }
    Ok(codes)
}

pub fn emitbits(codes: &[Opcode]) -> Vec<u8> {
    let mut result = Vec::new();
    for code in codes {
        for byte in code.to_bits() {
            result.push(byte);
        }
    }
    result
}

pub fn emitcodes(bits: &[u8]) -> Vec<Opcode> {
    bits.chunks_exact(4)
        .map(|chunk| {
            let array = [chunk[0], chunk[1], chunk[2], chunk[3]];
            Opcode::from_bits(array)
        })
        .collect()
}
