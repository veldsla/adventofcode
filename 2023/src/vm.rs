use thiserror::Error;

#[derive(Debug, Default)]
pub struct Vm {
    acc: isize,
    ip: usize,
    program: Vec<Instr>
}

#[derive(Debug, Clone, Copy)]
pub enum Instr {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}
use Instr::*;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum VmError {
    #[error("acc register overflow")]
    Overflow,
    #[error("jmp out of bounds")]
    InvalidJump,
    #[error("jmp 0 causes hang")]
    Jmp0,
    #[error("Infinte loop detected")]
    InfiniteLoop
}

impl Vm {
    pub fn from_program<I: IntoIterator<Item=Instr>>(v: I) -> Vm {
        Vm { ip: 0, acc: 0, program: v.into_iter().collect() }
    }

    pub fn run(&mut self) -> Result<(), VmError>{
        let mut run = vec![false; self.program.len()];
        loop {
            run[self.ip] = true;
            match self.program[self.ip] {
                Acc(n) => {
                    let sum = self.acc.checked_add(n);
                    self.acc = sum.ok_or(VmError::Overflow)?;
                    self.ip += 1;
                },
                Jmp(n) => {
                    if n == 0 {
                        return Err(VmError::Jmp0);
                    }
                    let pos = if n < 0 {
                        self.ip.checked_sub(-n as usize)
                    } else {
                        self.ip.checked_add(n as usize)
                    };
                    self.ip = pos.ok_or(VmError::InvalidJump)?;
                }
                Nop(_) => self.ip += 1,
            }

            if self.ip == self.program.len() {
                break;
            }

            if run[self.ip] {
                return Err(VmError::InfiniteLoop);
            }
        }
        Ok(())
    }

    pub fn ip(&self) -> usize {
        self.ip
    }

    pub fn acc(&self) -> isize {
        self.acc
    }
}
