use bf::asm::{assemble, emitcodes};
use bf::machine::Machine;
use std::env;
use std::fs;
use std::process;

fn getfilename() -> Result<String, String> {
    let filename = env::args()
        .skip(1)
        .next()
        .ok_or(String::from("filename required"));
    filename
}

fn main() {
    let filename = getfilename().unwrap_or_else(|err| {
        eprintln!("Error command arguments: {err}");
        process::exit(1);
    });

    let program = fs::read_to_string(&filename);
    let mut mach = Machine::new();

    let codes;

    if program.is_ok() {
        let program = program.unwrap();
        codes = assemble(String::from(program)).unwrap_or_else(|err| {
            eprintln!("Syntax error : {err}");
            process::exit(1);
        });
    } else {
        let program = fs::read(&filename).unwrap_or_else(|er| {
            eprintln!("Error reading binary {filename}  : {er}");
            process::exit(1);
        });
        codes = emitcodes(program);
    }

    mach.run(codes).unwrap_or_else(|err| {
        eprintln!("Error occured: {err}");
        process::exit(1);
    });
}
