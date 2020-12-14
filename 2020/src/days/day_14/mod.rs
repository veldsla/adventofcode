use std::collections::HashMap;
use std::str;

use anyhow::{anyhow, Result};

use crate::Problem;
use crate::parsers::positive_integer;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    multi::{fold_many_m_n, many1},
    combinator::{all_consuming, map},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult
};

#[derive(Default)]
pub struct Solution {
    program: Vec<Operation>
}

type MaskNot = u64; // set 0 to position to clear a bit, 
type MaskOr = u64; // set 1 to mask pos

#[derive(Debug)]
enum Operation {
    Mask(MaskOr, MaskNot),
    Mem(usize, u64),
}

fn mask_line(i: &str) -> IResult<&str,Operation> {
    let mask = map(fold_many_m_n(36,36, one_of("X01"), (0xFFFFFFF000000000, 0, 35), |(mut mnot, mut mor, bitnr), c| {
        match c {
            '0' => mnot |= 1 << bitnr,
            '1' => mor |= 1 << bitnr,
            'X' | _ => {},
        }
        (mnot, mor, bitnr - 1)
    }), |(mnot, mor, _)| Operation::Mask(mor, !mnot));
    preceded(tag("mask = "), terminated(mask, line_ending))(i)
}

fn mem_line(i: &str) -> IResult<&str,Operation> {
    let address = delimited(tag("["), positive_integer, tag("]"));
    map(preceded(tag("mem"), separated_pair(address, tag(" = "), positive_integer)),
        |(addr, val)| Operation::Mem(addr, val))(i)
}

fn parse(i: &str) -> IResult<&str,Vec<Operation>> {
    all_consuming(many1(alt((mask_line, terminated(mem_line, line_ending)))))(i)
}

fn run_program(p: &[Operation]) -> u64{
    //determine mem size
    let max_mem = p.iter().filter_map(|o| match o {
        Operation::Mem(i, _) => Some(*i),
        _ => None
    }).max().unwrap_or(0);
    let mut mem = vec![0; max_mem+1];

    let mut mask_zero = 0;
    let mut mask_one = 0;
    for o in p {
        match o {
            &Operation::Mask(or, not) => { mask_one = or; mask_zero = not; },
            &Operation::Mem(addr, val) => { 
                let val = val & mask_zero;
                let val = val | mask_one;
                mem[addr] = val;
            },
        }
    }

    mem.into_iter().sum()
}

fn float_addr(v: usize, addr: usize) -> Vec<usize> {
    let ones:Vec<_> = (0..36).into_iter().filter(|s| (v & 1 << s) > 0).collect();

    let mut perms = vec![addr];
    for p in ones {
        //flip bits at position p in previous
        let toadd: Vec<_> = perms.iter().map(|&v| {
            let sv = 1 << p;
            if v & sv > 0 {
                v & !sv
            } else {
                v | sv
            }
        }).collect();
        perms.extend(toadd);
    }
    perms
}

fn set_mem_float(addr: usize, mask_or: usize, mask_not: usize, mem: &mut HashMap<usize, u64>, val: u64)
{
    let new_addr = addr | mask_or;
    let float = !(mask_or | !mask_not);
    for ao in float_addr(float, new_addr).into_iter() {
        mem.insert(ao, val);
    }
}

fn run_program_alt(p: &[Operation]) -> u64{
    //create map for memory
    let mut mem = HashMap::new();
    let mut mask_zero = 0;
    let mut mask_one = 0;
    for o in p {
        match o {
            &Operation::Mask(or, not) => { mask_one = or as usize; mask_zero = not as usize; },
            &Operation::Mem(addr, val) => set_mem_float(addr, mask_one, mask_zero, &mut mem, val),
        }
    }
    mem.values().sum()
}


impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(str::from_utf8(i)?).map_err(|e| anyhow!(e.to_string()))?;
        self.program = result.1;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        Ok(format!("{}", run_program(&self.program)))
    }

    fn part2(&self) -> Result<String> {
        Ok(format!("{}", run_program_alt(&self.program)))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let program = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";
        let result = parse(program);
        assert!(result.is_ok());
        println!("result");
        let program = result.unwrap().1;

        println!("{:?}", program);

        assert_eq!(run_program(&program), 165);
    }

    #[test]
    fn p2() {
        let program = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
";
        let result = parse(program);
        assert!(result.is_ok());
        println!("result");
        let program = result.unwrap().1;

        println!("{:?}", program);

        assert_eq!(run_program_alt(&program), 208);

    }
   
}
