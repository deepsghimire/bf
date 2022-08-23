use bf::asm::assemble;
use bf::machine::Machine;
use std::process;
use std::env;
use std::fs;

fn getfilename() -> Result<String,String> {
    
    let filename  = env::args().skip(1).next().ok_or(String::from("filename required"));
    filename


    
}

fn main() {
    
    let filename = getfilename().unwrap_or_else(|err|{
        eprintln!("Error command arguments: {err}");
        process::exit(1);
    });

    let program = fs::read_to_string(&filename).unwrap_or_else(|err| {
        eprintln!("Error reading {filename} arguments: {err}");
        process::exit(1);
    });



    let mut mach = Machine::new();
    let codes = assemble(String::from(program)).unwrap_or_else(|err|{
        eprintln!("Syntax error : {err}");
        process::exit(1);
    });

    mach.run(codes).unwrap_or_else(|err| {
        eprintln!("Error occured: {err}");
        process::exit(1);
    });
}
