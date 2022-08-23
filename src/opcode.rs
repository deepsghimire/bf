

const ALLCMDS: &str = "<>[]+-.,";

#[derive(Debug,PartialEq)]
pub enum Opcode {
    MovRight,
    MovLeft,
    Inc,
    Dec,
    Out,
    In,
    JumpOut(u32),
    JumpIn(u32),
}

impl Opcode {
   pub fn from(cmd: u8) -> Self {

        match cmd {
            b'+' => Opcode::Inc,
            b'-' => Opcode::Dec,
            b'<' => Opcode::MovLeft,
            b'>' => Opcode::MovRight,
            b'.' => Opcode::Out,
            b',' => Opcode::In,
            b']' => Opcode::JumpOut(0),
            b'[' => Opcode::JumpIn(0),
            _ => todo!()
        }
    }
}
