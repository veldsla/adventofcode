use std::str;

use anyhow::{anyhow, Result};

use crate::Problem;
use crate::vm::*;
use crate::parsers::signed_integer;

#[derive(Default)]
pub struct Solution {
    program: Vec<Instr>
}

use nom::{
    bytes::complete::tag,
    branch::alt,
    character::complete::line_ending,
    multi::many1,
    combinator::{all_consuming, map},
    sequence::{preceded, terminated},
    IResult
};


fn parse(i: &str) -> IResult<&str, Vec<Instr>> {
    let acc = map(preceded(tag("acc "), signed_integer), Instr::Acc);
    let jmp = map(preceded(tag("jmp "), signed_integer), Instr::Jmp);
    let nop = map(preceded(tag("nop "), signed_integer), Instr::Nop);

    let instr = alt((acc, jmp, nop));

    all_consuming(many1(terminated(instr, line_ending)))(i)
}

fn mutate(p: &[Instr], pos: usize) -> impl Iterator<Item=Instr> + '_ {
    p.iter()
        .enumerate()
        .map(move |(i, &ins)| {
            if i == pos {
                match ins {
                    Instr::Nop(n) => Instr::Jmp(n),
                    Instr::Jmp(n) => Instr::Nop(n),
                    _ => ins
                }
            } else {
                ins
            }
        })
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(str::from_utf8(i)?).map_err(|e| anyhow!(e.to_string()))?;
        self.program = result.1;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let mut vm = Vm::from_program(self.program.iter().copied());
        match vm.run() {
            Err(e) if e == VmError::InfiniteLoop => Ok(()),
            x => x
        }?;
        Ok(format!("{}", vm.acc()))
    }

    fn part2(&self) -> Result<String> {
        let acc_ok = (0..self.program.len())
            .filter(|&i| matches!(self.program[i], Instr::Jmp(_)))
            .find_map(|mut_ins| {
                let mut vm = Vm::from_program(mutate(&self.program[..], mut_ins));
                if vm.run().is_ok() {
                    Some(vm.acc())
                } else {
                    None
                }
            }).ok_or_else(|| anyhow!("No solution"))?;


        Ok(format!("{:?}", acc_ok))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
    }
}
