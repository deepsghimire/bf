pub mod asm;
pub mod machine;
mod opcode;

use crate::asm::{assemble, emitbits};
use std::error;
use std::fs;
use std::io::Write;

pub fn compile_and_save(progfile: &str, binfile: &str) -> Result<(), Box<dyn error::Error>> {
    let program = fs::read_to_string(&progfile)
        .map_err(|err| format!("Unable to read '{progfile}' file: {err}"))?;

    let code = assemble(&program)?;
    let instructions = emitbits(&code);

    let mut outfile = fs::File::create(&binfile)
        .map_err(|err| format!("Unable to create '{binfile}' file: {err}"))?;

    outfile
        .write_all(&instructions)
        .map_err(|err| format!("Unable to write to '{binfile}': {err}"))?;
    Ok(())
}
