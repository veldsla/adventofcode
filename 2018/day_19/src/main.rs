use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

struct Computer<'a> {
    registers: [i64; 6],
    mnemonics: HashMap<&'a str, usize>,
    opcodes: Vec<Box<dyn Fn(&mut [i64; 6], i64, i64, usize)>>,
    ip_reg: usize,
    program: Option<Vec<Instruction>>
}

fn make_opcode(f: impl Fn(&mut [i64; 6], i64, i64, usize) + 'static) -> 
    Box<(dyn Fn(&mut [i64; 6], i64, i64, usize) + 'static)> 
{
    Box::new(f)
}

fn ioerror<E: fmt::Display>(e: E) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, format!("{}", e))
}

impl<'a> Computer<'a> {
    fn new() -> Computer<'a> {
        let mnemonics: HashMap<_, usize> = vec!["addr", "addi", "mulr", "muli", "banr", "bani",
        "borr", "bori", "setr", "seti", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr"]
            .into_iter().enumerate().map(|(p, i)| (i, p))
            .collect();

        let opcodes = vec![
            make_opcode(|r, a, b, c| r[c] = r[a as usize] + r[b as usize]),
            make_opcode(|r, a, b, c| r[c] = r[a as usize] + b),
            make_opcode(|r, a, b, c| r[c] = r[a as usize] * r[b as usize]),
            make_opcode(|r, a, b, c| r[c] = r[a as usize] * b),
            make_opcode(|r, a, b, c| r[c] = r[a as usize] & r[b as usize]),
            make_opcode(|r, a, b, c| r[c] = r[a as usize] & b),
            make_opcode(|r, a, b, c| r[c] = r[a as usize] | r[b as usize]),
            make_opcode(|r, a, b, c| r[c] = r[a as usize] | b),
            make_opcode(|r, a, _, c| r[c] = r[a as usize]),
            make_opcode(|r, a, _, c| r[c] = a),
            make_opcode(|r, a, b, c| r[c] = if a > r[b as usize] { 1 } else { 0 }),
            make_opcode(|r, a, b, c| r[c] = if r[a as usize] > b { 1 } else { 0 }),
            make_opcode(|r, a, b, c| r[c] = if r[a as usize] > r[b as usize] { 1 } else { 0 }),
            make_opcode(|r, a, b, c| r[c] = if a == r[b as usize] { 1 } else { 0 }),
            make_opcode(|r, a, b, c| r[c] = if r[a as usize] == b { 1 } else { 0 }),
            make_opcode(|r, a, b, c| r[c] = if r[a as usize] == r[b as usize] { 1 } else { 0 }),
        ];

        Computer { registers: [0; 6], mnemonics, opcodes, ip_reg: 0, program: None }
    }

    fn set_register(&mut self, reg: usize, value: i64) {
        self.registers[reg] = value;
    }

    fn get_register(&mut self, reg: usize) -> i64{
        self.registers[reg]
    }

    fn load_program<P: AsRef<Path>>(&mut self, p: P) -> io::Result<()> {
        let f = File::open(p)?;
        let mut lines = BufReader::new(f).lines();
        let ip_reg = lines.next().ok_or(ioerror("No ip header")).and_then(|l| {
            l.and_then(|l| {
                if l.starts_with("#ip ") {
                    l.split_whitespace().nth(1).unwrap().parse::<usize>()
                        .map_err(|_| ioerror("Cannot parse ip register"))
                } else {
                    Err(ioerror("No ip header"))
                }
            })
        })?;
        
        let program = lines.map(|l| {
            l.and_then(|l| {
                let mut parts = l.split_whitespace();
                let cmd = parts.next().ok_or(ioerror("Error parsing instruction"))?;
                if let Some(&opcode) = self.mnemonics.get(cmd) {
                    let a = parts.next().ok_or(ioerror("Error parsing instruction"))
                        .and_then(|s| s.parse().map_err(ioerror))?;
                    let b = parts.next().ok_or(ioerror("Error parsing instruction"))
                        .and_then(|s| s.parse().map_err(ioerror))?;
                    let c = parts.next().ok_or(ioerror("Error parsing instruction"))
                        .and_then(|s| s.parse().map_err(ioerror))?;
                    Ok(Instruction { opcode, a, b, c })
                } else {
                    Err(ioerror("Uknown instruction"))
                }
            })
        }).collect::<Result<Vec<Instruction>, _>>()?;

        self.ip_reg = ip_reg;
        self.program = Some(program);

        Ok(())
    }

    fn run(&mut self) -> Result<usize, &'static str> {
        if let Some(ref program) = self.program {
            let mut it = 0;
            loop {
                let current_instruction = self.registers[self.ip_reg] as usize;
                if let Some(instruction) = program.get(current_instruction) {
                    it += 1;
                    self.opcodes[instruction.opcode](&mut self.registers, instruction.a, instruction.b, instruction.c);
                    self.registers[self.ip_reg] += 1;
                } else {
                    break;
                }
            }
            Ok(it)
        } else {
            Err("Load a program before calling run")
        }
    }

    fn reset(&mut self) {
        self.program = None;
        self.registers = [0, 0, 0, 0, 0, 0];
        self.ip_reg = 0;
    }

}

#[derive(Debug)]
struct Instruction {
    opcode: usize,
    a: i64,
    b: i64,
    c: usize,
}

fn main() -> Result<(), String> {
    let mut c = Computer::new();
    c.load_program("input.txt").map_err(|e| format!("Fatal error:\n{}", e))?;
    let now = Instant::now();
    let n_it = c.run()?;
    let elapsed = now.elapsed();
    println!("Ran {} iterations in {}µs, ({:.2}ns/iteration)",
             n_it, elapsed.subsec_micros(), elapsed.subsec_nanos() as f64 / n_it as f64);
    println!("19a: Value in register[0] after runnning input.txt is {}", c.get_register(0));
    println!("19b: sum(divisors(10551264)) = 27941760");

    Ok(())
}
