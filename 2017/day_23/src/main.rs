use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::str::FromStr;

struct Computer {
    registers: Registers,
    program: Vec<Instruction>,
    current_address: i64,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
enum Instruction {
    set(Arg, Arg),
    add(Arg, Arg),
    sub(Arg, Arg),
    mul(Arg, Arg),
    modulo(Arg, Arg),
    jgz(Arg, Arg),
    jnz(Arg, Arg),
}

#[derive(Copy, Clone, Debug)]
enum Arg {
    Value(i64),
    Reg(Register),
}

impl FromStr for Arg {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            Err("Arg is emtpy")
        } else {
            let c = s.chars().nth(0).unwrap();
            if c >= 'a' && c <= 'z' {
                Ok(Arg::Reg(Register(c)))
            } else {
                let val = try!(s.parse().map_err(|_| "Error parsing value"));
                Ok(Arg::Value(val))
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Register(char);
impl Register {
    fn index(&self) -> usize {
        let Register(a) = *self;
        a as usize - 97
    }
}

#[derive(Debug)]
struct Registers {
    values: Vec<i64>
}

impl Registers {
    fn new(n: usize) -> Registers {
        Registers {values: vec![0; n]}
    }

    fn set(&mut self, r: Register, value: i64) {
        self.values[r.index()] = value;
    }

    fn get(&mut self, r: Register) -> i64 {
        self.values[r.index()]
    }

    fn add(&mut self, r: Register, value: i64) {
        self.values[r.index()] += value;
    }

    fn sub(&mut self, r: Register, value: i64) {
        self.values[r.index()] -= value;
    }

    fn mul(&mut self, r: Register, value: i64) {
        self.values[r.index()] *= value;
    }

    fn modulo(&mut self, r: Register, value: i64) {
        self.values[r.index()] %= value;
    }
}

impl FromStr for Instruction {
    type Err = &'static str;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let p = line.split(" ").collect::<Vec<&str>>();
        let arg1: Arg = p[1].parse()?;
        let arg2 = if p.len() == 3 { p[2].parse::<Arg>()} else { Err("Arg not set") };

        match p[0] {
            "set" => Ok(Instruction::set(arg1, arg2? )),
            "add" => Ok(Instruction::add(arg1, arg2?)),
            "sub" => Ok(Instruction::sub(arg1, arg2?)),
            "mul" => Ok(Instruction::mul(arg1, arg2?)),
            "mod" => Ok(Instruction::modulo(arg1, arg2? )),
            "jgz" => Ok(Instruction::jgz(arg1, arg2? )),
            "jnz" => Ok(Instruction::jnz(arg1, arg2? )),
            _ => Err("Unknown instruction")
        }
    }
}

impl Computer {
    fn new () -> Computer {
        Computer {registers: Registers::new(8), program: Vec::new(), current_address: 0 }
    }

    fn add_instruction(&mut self, instruction: Instruction) {
        self.program.push(instruction.into());
    }

    fn run_program(&mut self) -> usize {
        let mut mulcount = 0;
        loop {
            if self.done() {
                return mulcount;
            }
            if self.registers.get(Register('a')) == 1 {
                panic!("Arfrrghh HCF!");
            }
            let v = self.program[self.current_address as usize];
            match v {
                Instruction::set(to, from) => {
                    let val = match from {
                        Arg::Reg(c) => self.registers.get(c),
                        Arg::Value(v) => v
                    };
                    match to {
                        Arg::Reg(c) => self.registers.set(c, val),
                        _ => {} //nop
                    }
                    self.current_address += 1;
                },
                Instruction::add(to, from) => {
                    let val = match from {
                        Arg::Reg(c) => self.registers.get(c),
                        Arg::Value(v) => v
                    };
                    match to {
                        Arg::Reg(c) => self.registers.add(c, val),
                        _ => {} //nop
                    }
                    self.current_address += 1;
                },
                Instruction::sub(to, from) => {
                    let val = match from {
                        Arg::Reg(c) => self.registers.get(c),
                        Arg::Value(v) => v
                    };
                    match to {
                        Arg::Reg(c) => self.registers.sub(c, val),
                        _ => {} //nop
                    }
                    self.current_address += 1;
                },
                Instruction::mul(to, from) => {
                    let val = match from {
                        Arg::Reg(c) => self.registers.get(c),
                        Arg::Value(v) => v
                    };
                    match to {
                        Arg::Reg(c) => self.registers.mul(c, val),
                        _ => {} //nop
                    }
                    mulcount += 1;
                    self.current_address += 1;
                },
                Instruction::modulo(to, from) => {
                    let val = match from {
                        Arg::Reg(c) => self.registers.get(c),
                        Arg::Value(v) => v
                    };
                    match to {
                        Arg::Reg(c) => self.registers.modulo(c, val),
                        _ => {} //nop
                    }
                    self.current_address += 1;
                },
                Instruction::jgz(arg1, arg2) => {
                    let val = match arg1 {
                        Arg::Reg(c) => self.registers.get(c),
                        Arg::Value(v) => v
                    };
            
                    let offset = match arg2 {
                        Arg::Reg(c) => self.registers.get(c),
                        Arg::Value(v) => v
                    };

                    if val > 0 {
                        self.current_address += offset
                    } else {
                        self.current_address += 1;
                    }
                },
                Instruction::jnz(arg1, arg2) => {
                    let val = match arg1 {
                        Arg::Reg(c) => self.registers.get(c),
                        Arg::Value(v) => v
                    };
            
                    let offset = match arg2 {
                        Arg::Reg(c) => self.registers.get(c),
                        Arg::Value(v) => v
                    };

                    if val != 0 {
                        self.current_address += offset
                    } else {
                        self.current_address += 1;
                    }
                },
            };
        }
    }

    fn done(&self) -> bool {
        self.current_address as usize > self.program.len() -1 || self.current_address < 0
    }
}

fn main() {

    let f = File::open("input.txt").unwrap_or_else(|_| panic!("Input file not found"));
    let reader = BufReader::new(f);

    let mut debug_computer = Computer::new();
    let mut computer = Computer::new();

    for l in reader.lines() {
        let line = l.unwrap();
        let instruction: Instruction = line.parse().unwrap_or_else(|e| panic!("{}", e));
        debug_computer.add_instruction(instruction.clone());
        computer.add_instruction(instruction.clone());
    }
    let mul = debug_computer.run_program();
    println!("23a: The mul instruction was called {} times", mul);
    computer.registers.set(Register('a'), 1);
    computer.run_program();
}
