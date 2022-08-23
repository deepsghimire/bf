use std::env;
use std::process;

use bf::compile_and_save;

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

    compile_and_save(&programfile, &outputfile).unwrap_or_else(|err| {
        eprintln!("Compilation Error: {err}");
    })
}
