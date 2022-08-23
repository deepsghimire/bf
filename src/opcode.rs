



pub const ALLCMDS: &[u8] = b"<>[]+-.,";


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

   pub fn to_bits(&self) -> [u8;4] {
        match self {
            Opcode::Inc => [0x00,0x00,0x00,0xAA],
            Opcode::Dec => [0x00,0x00,0x00,0xBB],
            Opcode::MovLeft => [0x00,0x00,0x00,0xCC],
            Opcode::MovRight => [0x00,0x00,0x00,0xDD],
            Opcode::Out => [0x00,0x00,0x00,0xEE],
            Opcode::In => [0x00,0x00,0x00,0xFF],
            Opcode::JumpOut(location) => {
                let addrbyte0 = (location & 0x00_00_00_FF) as u8;
                let addrbyte1 =((location & 0x00_00_FF_00) >> 8) as u8;
                let addrbyte2:u8 = ((location & 0x00_FF_00_00) >> 16) as u8;

                [0xAA,addrbyte2,addrbyte1,addrbyte0]
            }
            Opcode::JumpIn(location) => {
                let addrbyte0 = (location & 0x00_00_00_FF) as u8;
                let addrbyte1 =((location & 0x00_00_FF_00) >> 8) as u8;
                let addrbyte2:u8 = ((location & 0x00_FF_00_00) >> 16) as u8;

                [0xBB,addrbyte2,addrbyte1,addrbyte0]
            }

        }

    }

    pub fn from_bits(bits: [u8;4])  -> Self {
        let last = bits[3];
        let jump = bits[0];

        match last {
            0xAA => Opcode::Inc,
            0xBB => Opcode::Dec,
            0xCC => Opcode::MovLeft,
            0xDD => Opcode::MovRight,
            0xEE => Opcode::Out,
            0xFF => Opcode::In,
            _ => {
                match jump {
                    0xAA => Opcode::JumpOut(u32::from_be_bytes([0x00,bits[1],bits[2],bits[3]])),
                    0xBB => Opcode::JumpIn(u32::from_be_bytes([0x00,bits[1],bits[2],bits[3]])),
                    _ => todo!()
                }
            }
        }

    }

}
