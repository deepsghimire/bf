use std::env;
use std::fs;
use std::io::Write;
use std::process;

use bf::asm::assemble;
use bf::asm::emitbits;

fn get_config() -> Result<(String, String), String> {
    let mut args = env::args().skip(1);

    let input = args.next().ok_or("Input file required")?;
    let output = args.next().ok_or("Output file required")?;
    Ok((input, output))
}

fn main() {
    let (programfile, outputfile) = get_config().unwrap_or_else(|err| {
        eprintln!("Argument error: {err}");
        process::exit(1);
    });

    let mut outfile = fs::File::create(&outputfile).unwrap_or_else(|err| {
        eprintln!("Unable to open to {outputfile}: {err}");
        process::exit(1);
    });

    let program = fs::read_to_string(&programfile).unwrap_or_else(|err| {
        eprintln!("Error reading program from {programfile} : {err}");
        process::exit(1);
    });

    let code = assemble(&program).unwrap_or_else(|err| {
        eprintln!("Syntax error : {err}");
        process::exit(1);
    });

    let instructions = emitbits(&code);

    outfile.write_all(&instructions).unwrap_or_else(|err| {
        eprintln!("Error reading program from {programfile} : {err}");
        process::exit(1);
    });
}
