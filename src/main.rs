use bf::asm::assemble;
use bf::machine::Machine;
use std::process;

fn main() {
    let program = ">>>>++>+[[[>>]<[>+>]<<[>->>+<<<-]>+[[\
        +>>[<<+>>-]>]+[-<<+<]>-[-[<+>>+<-]++++++[>+++++++\
        +<-]+>.[-]<<[[[<<+>+>-]>>>]<<<<[[>+<-]<-<<]>-]>>>[\
        <<-[<<+>>-]<+++++++++<[[->+>]>>>[<<[<+>-]>>>+>>[-<]\
        <[>]>+<]<<<<<<-]>[-]>+>>[<<<\
        +>>>-]>>>]<<<+[-[+>>]<<<]>[<<\
        <]>]>>>[<[>>>]<<<[[>>>+<<<-]<<<]>>>>>>>-[<]>>>[<<]<<[>+>]<]<<]++>>]<<++++++++.+]";
    let mut mach = Machine::new();
    let codes = assemble(String::from(program)).unwrap();

    mach.run(codes).unwrap_or_else(|err| {
        eprintln!("Error occured: {err}");
        process::exit(1);
    });
}
