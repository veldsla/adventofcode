use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

struct Computer {
    reg_a: u32,
    reg_b: u32,
    program: Vec<Instruction>,
    current_address: u32
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
enum Instruction {
    hlf(Register),
    tpl(Register),
    inc(Register),
    jmp(i64),
    jie((Register, i64)),
    jio((Register, i64))
}

#[derive(Copy, Clone)]
enum Register {
    A,
    B
}

//computer only does over/underflow checks on jumps not inc mul
//using the unstable std::num::wrapping::OverflowingOps this is quite simple
impl Computer {
    fn new () -> Computer {
        Computer {reg_a: 0 , reg_b: 0, program: Vec::new(), current_address: 0}
    }

    fn process_line(&mut self, line: String) {
        let p = line.split(" ").collect::<Vec<&str>>();
        let match_reg = |s| {
            match s { "a" => Register::A, "b" => Register::B, _=> panic!("Invalid register")}
        };
        match p[0] {
            "hlf" => self.program.push(Instruction::hlf(match_reg(p[1]))),
            "tpl" => self.program.push(Instruction::tpl(match_reg(p[1]))),
            "inc" => self.program.push(Instruction::inc(match_reg(p[1]))),
            "jmp" => self.program.push(Instruction::jmp(p[1].parse::<i64>().unwrap())),
            "jie" => self.program.push(Instruction::jie((match_reg(p[1]), p[2].parse::<i64>().unwrap()))),
            "jio" => self.program.push(Instruction::jio((match_reg(p[1]), p[2].parse::<i64>().unwrap()))),
            _ => panic!("Unknown instruction")
        }
    }

    fn run_program(&mut self) {
        println!("Executing program...");
        loop {
            if self.done() {
                break;
            }
            let v = self.program[self.current_address as usize];
            match v {
                Instruction::hlf(reg) => {
                    self.half_register(reg);
                    self.current_address += 1;
                },
                Instruction::tpl(reg) => {
                    self.triple_register(reg);
                    self.current_address += 1;
                },
                Instruction::inc(reg) => {
                    self.inc_register(reg);
                    self.current_address += 1;
                },
                Instruction::jmp(offset) => {
                    self.jump(offset)
                },
                Instruction::jie((reg, offset)) => {
                    if self.reg_even(reg){
                        self.jump(offset)
                    } else {
                        self.current_address += 1;
                    }
                },
                Instruction::jio((reg, offset)) => {
                    if self.reg_one(reg) {
                        self.jump(offset)
                    } else {
                        self.current_address += 1;
                    }
                },
            };
        }
    }

    fn half_register(&mut self, r: Register) {
        match r {
            Register::A => self.reg_a /= 2,
            Register::B => self.reg_b /= 2
        };
    }

    fn triple_register(&mut self, r: Register) {
        match r {
            Register::A => self.reg_a *= 3,
            Register::B => self.reg_b *= 3
        };
    }

    fn inc_register(&mut self, r: Register) {
        match r {
            Register::A => self.reg_a += 1,
            Register::B => self.reg_b += 1
        };
    }

    fn jump(&mut self, offset: i64) {
        let new_address = self.current_address as i64 + offset;
        if new_address < 0 {
            panic!("Segmentation fault");
        }
        self.current_address = new_address as u32;
    }

    fn reg_even(&self, r: Register) -> bool {
        match r {
            Register::A => self.reg_a % 2 == 0,
            Register::B => self.reg_b % 2 == 0
        }
    }

    fn reg_one(&self, r: Register) -> bool {
        match r {
            Register::A => self.reg_a == 1,
            Register::B => self.reg_b == 1
        }
    }

    fn done(&self) -> bool {
        self.current_address as usize > self.program.len() -1
    }

    fn reset(&mut self) {
        self.reg_a = 0;
        self.reg_b = 0;
        self.current_address = 0;
    }
}

fn main() {

    let f = File::open("23_in.txt").unwrap();
    let reader = BufReader::new(f);

    let mut computer = Computer::new();
    for l in reader.lines() {
        let mut line = l.ok().unwrap();
        //remove unnecessary characters
        line = line.replace(",","");
        line = line.replace("+","");
        computer.process_line(line);
    }

    computer.run_program();
    println!("The value in register b is {}", computer.reg_b);
    computer.reset();
    computer.reg_a = 1;
    computer.run_program();
    println!("The value in register b (reg_a starting with 1) is {}", computer.reg_b);
}

#[test]
fn test() {
    let mut c = Computer::new();
    c.process_line("inc a".to_string());
    c.process_line("jio a 2".to_string());
    c.process_line("tpl a".to_string());
    c.process_line("inc a".to_string());
    
    println!("{:?}", c);
    assert_eq!(c.reg_a, 0);
    c.run_program();
    assert_eq!(c.reg_a, 2);
}
