use crate::opcode::Opcode;

pub fn assemble(code: String) -> Result<Vec<Opcode>, String> {
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
                _ => return Err(format!("no [ for ] at: {index}")),
            },
            other => other,
        };
        codes.push(code)
    }

    if !stack.is_empty() {
        return Err(format!("no ] found for [ at: {}", stack.pop().unwrap()));
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
