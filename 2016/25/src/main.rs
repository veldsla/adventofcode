use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::str::FromStr;

struct Computer {
    registers: Registers,
    program: Vec<Instruction>,
    backup: Vec<Instruction>,
    current_address: i32
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
enum Instruction {
    cpy(Arg, Arg),
    inc(Arg),
    dec(Arg),
    jnz(Arg, Arg),
    tgl(Arg),
    out(Arg)
}

#[derive(Copy, Clone, Debug)]
enum Arg {
    Value(i32),
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
    values: Vec<i32>
}

impl Registers {
    fn new(n: usize) -> Registers {
        Registers {values: vec![0; n]}
    }

    fn clear(&mut self) {
        for v in self.values.iter_mut() {
            *v = 0;
        }
    }

    fn set(&mut self, r: Register, value: i32) {
        self.values[r.index()] = value;
    }

    fn get(&mut self, r: Register) -> i32 {
        self.values[r.index()]
    }

    fn inc(&mut self, r: Register) {
        self.values[r.index()] += 1;
    }

    fn dec(&mut self, r: Register) {
        self.values[r.index()] -= 1;
    }
}

impl FromStr for Instruction {
    type Err = &'static str;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let p = line.split(" ").collect::<Vec<&str>>();
        let arg1: Arg = p[1].parse()?;
        let arg2 = if p.len() == 3 { p[2].parse::<Arg>()} else { Err("Arg not set") };

        match p[0] {
            "cpy" => Ok(Instruction::cpy(arg1, arg2? )),
            "inc" => Ok(Instruction::inc(arg1)),
            "dec" => Ok(Instruction::dec(arg1)),
            "jnz" => Ok(Instruction::jnz(arg1, arg2? )),
            "tgl" => Ok(Instruction::tgl(arg1)),
            "out" => Ok(Instruction::out(arg1)),
            _ => Err("Unknown instruction")
        }
    }
}

impl Computer {
    fn new () -> Computer {
        Computer {registers: Registers::new(4), program: Vec::new(), backup: Vec::new(), current_address: 0}
    }

    fn add_instruction(&mut self, instruction: Instruction) {
        self.program.push(instruction.into());
    }

    fn run_program(&mut self) {
        //println!("Creating backup of program");
        self.backup = self.program.clone();
        //println!("Executing program...");
        let mut out_expect = 0;
        let mut n: usize = 0;
        loop {
            if self.done() {
                println!("Used {} iterations", n);
                break;
            }
            n +=1;
            let v = self.program[self.current_address as usize];
            match v {
                Instruction::cpy(from, to) => {
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
                Instruction::inc(arg) => {
                    match arg {
                        Arg::Reg(c) => self.registers.inc(c),
                        _ => {} //nop
                    }
                    self.current_address += 1;
                },
                Instruction::dec(arg) => {
                    match arg {
                        Arg::Reg(c) => self.registers.dec(c),
                        _ => {} //nop
                    }
                    self.current_address += 1;
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
                Instruction::tgl(arg) => {
                    let val = match arg {
                        Arg::Reg(c) => self.registers.get(c),
                        Arg::Value(v) => v
                    };
                    let pos = (self.current_address + val) as usize;

                    if pos < self.program.len() {
                        let new = match self.program[pos] {
                            Instruction::inc(a) => Instruction::dec(a),
                            Instruction::dec(a) => Instruction::inc(a),
                            Instruction::jnz(a, b) => Instruction::cpy(a, b),
                            Instruction::cpy(a,b) => Instruction::jnz(a, b),
                            Instruction::tgl(a) => Instruction::inc(a),
                            Instruction::out(a) => Instruction::inc(a)
                        };
                        self.program[pos] = new;
                    }
                    self.current_address += 1;
                },
                Instruction::out(arg) => {
                    let val = match arg {
                        Arg::Reg(c) => self.registers.get(c),
                        Arg::Value(v) => v
                    };
                    if out_expect != val {
                        //terminate
                        self.current_address = self.program.len() as i32;

                    } else {
                        out_expect = (out_expect + 1) % 2;
                        self.current_address += 1;
                    }
                }
            };
        }
    }

    fn done(&self) -> bool {
        self.current_address as usize > self.program.len() -1
    }

    fn reset(&mut self) {
        //println!("Restoring backup");
        self.program = self.backup.clone();
        self.registers.clear();
        self.current_address = 0;
    }
}

fn main() {

    let f = File::open("input.txt").unwrap_or_else(|_| panic!("Input file not found"));
    let reader = BufReader::new(f);

    let mut computer = Computer::new();
    for l in reader.lines() {
        let line = l.unwrap();

        computer.add_instruction(line.parse().unwrap_or_else(|e| panic!("{}", e)));
    }
    let mut a = 0;
    loop {
        println!("Trying {} in register a", a); 
        computer.registers.set(Register('a'), a);
        computer.run_program();
        computer.reset();
        a += 1;
    }
}

