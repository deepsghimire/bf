use crate::opcode::Opcode;
use std::io;
use std::io::{Read, Write};

const CAP: usize = 0xFF_FF_FF;

pub struct Machine {
    cells: Vec<u8>,
    point: usize,
    text: Vec<Opcode>,
    code_point: usize,
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}

impl Machine {
    pub fn new() -> Self {
        Self {
            cells: vec![0; CAP],
            point: 0,
            text: vec![],
            code_point: 0,
        }
    }

    fn nextcode(&mut self) {
        self.code_point += 1;
    }

    fn execute(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let code = &self.text[self.code_point];
        match code {
            Opcode::MovLeft => {
                if self.point > 0 {
                    self.point -= 1;
                };
            }
            Opcode::MovRight => {
                if self.point < CAP {
                    self.point += 1;
                };
            }
            Opcode::Inc => {
                self.cells[self.point] = self.cells[self.point].wrapping_add(1);
            }
            Opcode::Dec => {
                self.cells[self.point] = self.cells[self.point].wrapping_sub(1);
            }
            Opcode::In => {
                let input = io::stdin().bytes().next().ok_or("empty input")??;
                self.cells[self.point] = input;
            }
            Opcode::Out => {
                // let output = String::from_utf8(vec![self.cells[self.point]])?;
                let output = self.cells[self.point];
                write!(io::stdout(),"{}",output as char)?;
            }
            Opcode::JumpIn(loc) => {
                if self.cells[self.point] == 0 {
                    self.code_point = *loc as usize;
                }
            }
            Opcode::JumpOut(loc) => {
                if self.cells[self.point] != 0 {
                    self.code_point = *loc as usize;
                }
            }
        };
        self.nextcode();
        Ok(())
    }

    pub fn run(&mut self, codes: Vec<Opcode>) -> Result<(), Box<dyn std::error::Error>> {
        self.text = codes;
        while self.code_point < self.text.len() {
            if let Err(e) = self.execute() {
                eprintln!("Warning: {e}")
            }
        }
        Ok(())
    }
}
