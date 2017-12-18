use std::collections::VecDeque;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::str::FromStr;

struct Computer {
    registers: Registers,
    program: Vec<Instruction>,
    current_address: i64,
    send_queue: VecDeque<i64>
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
enum Instruction {
    snd(Arg),
    set(Arg, Arg),
    add(Arg, Arg),
    mul(Arg, Arg),
    modulo(Arg, Arg),
    rcv(Arg),
    jgz(Arg, Arg),
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
            "snd" => Ok(Instruction::snd(arg1)),
            "set" => Ok(Instruction::set(arg1, arg2? )),
            "add" => Ok(Instruction::add(arg1, arg2?)),
            "mul" => Ok(Instruction::mul(arg1, arg2?)),
            "mod" => Ok(Instruction::modulo(arg1, arg2? )),
            "rcv" => Ok(Instruction::rcv(arg1)),
            "jgz" => Ok(Instruction::jgz(arg1, arg2? )),
            _ => Err("Unknown instruction")
        }
    }
}

impl Computer {
    fn new () -> Computer {
        Computer {registers: Registers::new(26), program: Vec::new(), current_address: 0, send_queue: VecDeque::new()}
    }

    fn add_instruction(&mut self, instruction: Instruction) {
        self.program.push(instruction.into());
    }

    fn run_program(&mut self) -> Option<Register> {
        loop {
            if self.done() {
                return None;
            }
            let v = self.program[self.current_address as usize];
            match v {
                Instruction::snd(arg) => {
                    let val = match arg {
                        Arg::Reg(c) => self.registers.get(c),
                        Arg::Value(v) => v
                    };
                    self.send_queue.push_back(val);
                    self.current_address += 1;
                },
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
                Instruction::mul(to, from) => {
                    let val = match from {
                        Arg::Reg(c) => self.registers.get(c),
                        Arg::Value(v) => v
                    };
                    match to {
                        Arg::Reg(c) => self.registers.mul(c, val),
                        _ => {} //nop
                    }
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
                Instruction::rcv(arg) => {
                    self.current_address += 1;
                    match arg {
                        Arg::Reg(c) => return Some(c),
                        Arg::Value(v) => panic!("Cannot receive into value {}", v)
                    };
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

    let mut computer = Computer::new();
    let mut computer_a = Computer::new();
    let mut computer_b = Computer::new();
    computer_a.registers.set(Register('p'), 0);
    computer_b.registers.set(Register('p'), 1);

    for l in reader.lines() {
        let line = l.unwrap();
        let instruction: Instruction = line.parse().unwrap_or_else(|e| panic!("{}", e));
        computer.add_instruction(instruction.clone());
        computer_a.add_instruction(instruction.clone());
        computer_b.add_instruction(instruction.clone());
    }
    while let Some(v) = computer.run_program() {
        //if reg v == 0 we are done
        if computer.registers.get(v) != 0 {
            break;
        }
    }
    println!("18a: The last played sound? is {}", computer.send_queue.back().unwrap());

    let mut rec_count = 0;
    let mut receive_here_from_b = computer_a.run_program();
    let mut receive_here_from_a = computer_b.run_program();
    loop {
        if receive_here_from_a.is_some() && computer_a.done() && computer_a.send_queue.is_empty() {
            //a is finished and send queue empty, b wants data...
            break;
        }
        if receive_here_from_b.is_some() && computer_b.done() && computer_b.send_queue.is_empty() {
            //b is finished and send queue empty, a wants data...
            break;
        }

        if receive_here_from_a.is_none() && receive_here_from_b.is_none() {
            //both ready
            break;
        }

        if computer_a.send_queue.is_empty() && computer_b.send_queue.is_empty() {
            //deadlock
            break;
        }

        //send from a to b
        if let Some(reg) = receive_here_from_a {
            if let Some(v) = computer_a.send_queue.pop_front() {
                computer_b.registers.set(reg, v);
                receive_here_from_a = computer_b.run_program();
            }
        }

        //send from b to a
        if let Some(reg) = receive_here_from_b {
            if let Some(v) = computer_b.send_queue.pop_front() {
                rec_count += 1;
                computer_a.registers.set(reg, v);
                receive_here_from_b = computer_a.run_program();
            }
        }
    }
    println!("18b: Program 2 sent {} times before the deadlock/exit occurred", rec_count);

}

#[test]
fn test() {
    let mut c = Computer::new();
    let input = "\
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
";    
    for l in input.lines() {
        c.add_instruction(l.parse().unwrap());
    } 
    c.run_program();
    assert_eq!(*c.send_queue.back().unwrap(), 4);
}
