use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

struct Computer {
    registers: Registers,
    program: Vec<Instruction>,
    current_address: i32
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
enum Instruction {
    cpy(i32, Register),
    cpy_reg(Register, Register),
    inc(Register),
    dec(Register),
    jnz_reg(Register, i32),
    jmp(i32),
    nop
}

#[derive(Copy, Clone)]
struct Register(char);

impl From<Register> for usize {
    fn from(r: Register) -> usize {
        let Register(c) = r;
        c as usize - 97
    }
}

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

    fn set<R: Into<usize>>(&mut self, r: R, value: i32) {
        self.values[r.into()] = value;
    }

    fn get<R: Into<usize>>(&mut self, r: R) -> i32 {
        self.values[r.into()]
    }

    fn inc<R: Into<usize>>(&mut self, r: R) {
        self.values[r.into()] += 1;
    }

    fn dec<R: Into<usize>>(&mut self, r: R) {
        self.values[r.into()] -= 1;
    }
}

impl From<String> for Instruction {
    fn from(line: String) -> Instruction {
        let p = line.split(" ").collect::<Vec<&str>>();
        match p[0] {
            "cpy" => {
                if let Some(val) = p[1].parse().ok() {
                    Instruction::cpy(val, Register(p[2].chars().nth(0).unwrap()))
                } else {
                    Instruction::cpy_reg(Register(p[1].chars().nth(0).unwrap()), Register(p[2].chars().nth(0).unwrap()))
                }
            },
            "inc" => Instruction::inc(Register(p[1].chars().nth(0).unwrap())),
            "dec" => Instruction::dec(Register(p[1].chars().nth(0).unwrap())),
            "jnz" => {
                if let Some(val) = p[1].parse::<i32>().ok() {
                    if val != 0 {
                        Instruction::jmp(p[2].parse().unwrap())
                    } else {
                        Instruction::nop
                    }
                } else {
                    Instruction::jnz_reg(Register(p[1].chars().nth(0).unwrap()), p[2].parse().unwrap())
                }
            },
            _ => panic!("Unknown instruction")
        }
    }
}

impl Computer {
    fn new () -> Computer {
        Computer {registers: Registers::new(4), program: Vec::new(), current_address: 0}
    }

    fn add_instruction<I: Into<Instruction>>(&mut self, instruction: I) {
        self.program.push(instruction.into());
    }

    fn run_program(&mut self) {
        println!("Executing program...");
        loop {
            if self.done() {
                break;
            }
            let v = self.program[self.current_address as usize];
            match v {
                Instruction::cpy(val, reg) => {
                    self.registers.set(reg, val);
                    self.current_address += 1;
                },
                Instruction::cpy_reg(from, to) => {
                    let val = self.registers.get(from);
                    self.registers.set(to, val);
                    self.current_address += 1;
                },
                Instruction::inc(reg) => {
                    self.registers.inc(reg);
                    self.current_address += 1;
                },
                Instruction::dec(reg) => {
                    self.registers.dec(reg);
                    self.current_address += 1;
                },
                Instruction::jmp(offset) => {
                    self.current_address += offset
                },
                Instruction::jnz_reg(reg, offset) => {
                    let val = self.registers.get(reg);
                    if val != 0 {
                        self.current_address += offset
                    } else {
                        self.current_address += 1;
                    }
                },
                Instruction::nop => {
                },
            };
        }
    }

    fn done(&self) -> bool {
        self.current_address as usize > self.program.len() -1
    }

    fn reset(&mut self) {
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
        computer.add_instruction(line);
    }

    computer.run_program();
    println!("The value in register a is {}", computer.registers.get(Register('a')));
    println!("Resetting and setting register c to 1");
    computer.reset();
    computer.registers.set(Register('c'), 1);
    computer.run_program();
    println!("The value in register a is now {}", computer.registers.get(Register('a')));
}

#[test]
fn test() {
    let mut c = Computer::new();
    let input = "\
cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a
";    
    for l in input.lines() {
        c.add_instruction(l.to_string());
    } 
    assert_eq!(c.registers.get(Register('a')), 0);
    c.run_program();
    assert_eq!(c.registers.get(Register('a')), 42);
}
