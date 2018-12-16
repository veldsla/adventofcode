use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, Read};
use std::str::FromStr;

struct Computer<'a> {
    registers: [i64; 4],
    opcodes: HashMap<&'a str, Box<dyn Fn(&mut [i64; 4], i64, i64, usize)>>
}

fn make_instr(f: impl Fn(&mut [i64; 4], i64, i64, usize) + 'static) -> 
    Box<(dyn Fn(&mut [i64; 4], i64, i64, usize) + 'static)> 
{
    Box::new(f)
}

impl<'a> Computer<'a> {
    fn new() -> Computer<'a> {
        let mut instr = HashMap::new();
        instr.insert("addr", make_instr(|r, a, b, c| r[c] = r[a as usize] + r[b as usize]));
        instr.insert("addi", make_instr(|r, a, b, c| r[c] = r[a as usize] + b));
        instr.insert("mulr", make_instr(|r, a, b, c| r[c] = r[a as usize] * r[b as usize]));
        instr.insert("muli", make_instr(|r, a, b, c| r[c] = r[a as usize] * b));
        instr.insert("banr", make_instr(|r, a, b, c| r[c] = r[a as usize] & r[b as usize]));
        instr.insert("bani", make_instr(|r, a, b, c| r[c] = r[a as usize] & b));
        instr.insert("borr", make_instr(|r, a, b, c| r[c] = r[a as usize] | r[b as usize]));
        instr.insert("bori", make_instr(|r, a, b, c| r[c] = r[a as usize] | b));
        instr.insert("setr", make_instr(|r, a, _, c| r[c] = r[a as usize]));
        instr.insert("seti", make_instr(|r, a, _, c| r[c] = a));
        instr.insert("gtir", make_instr(|r, a, b, c| r[c] = if a > r[b as usize] { 1 } else { 0 }));
        instr.insert("gtri", make_instr(|r, a, b, c| r[c] = if r[a as usize] > b { 1 } else { 0 }));
        instr.insert("gtrr", make_instr(|r, a, b, c| r[c] = if r[a as usize] > r[b as usize] { 1 } else { 0 }));
        instr.insert("eqir", make_instr(|r, a, b, c| r[c] = if a == r[b as usize] { 1 } else { 0 }));
        instr.insert("eqri", make_instr(|r, a, b, c| r[c] = if r[a as usize] == b { 1 } else { 0 }));
        instr.insert("eqrr", make_instr(|r, a, b, c| r[c] = if r[a as usize] == r[b as usize] { 1 } else { 0 }));

        Computer { registers: [0; 4], opcodes: instr }
    }

    fn set_registers(&mut self, registers: [i64; 4]) {
        self.registers = registers;
    }

    fn cmd(&mut self, s: &str, a: i64, b: i64, c: usize) {
        let f = self.opcodes.get(s).expect("Command not available");
        f(&mut self.registers, a, b, c);
    }

    fn available_opcodes(&self) -> Vec<String> {
        self.opcodes.keys().map(|s| s.to_string()).collect()
    }

    fn test_unit(&mut self,cmd: &str, unit: &Unit) -> bool {
        self.set_registers(unit.before);
        if let Some(f) = self.opcodes.get(cmd) {
            f(&mut self.registers, unit.a, unit.b, unit.c);
        } else {
            panic!("Illegal Instruction");
        }
        self.registers == unit.after
    }
}

#[derive(Debug, Clone)]
struct Unit {
    before:  [i64; 4],
    after:  [i64; 4],
    cmd: usize,
    a: i64,
    b: i64,
    c: usize,
}

impl FromStr for Unit {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split('\n');
        let be = lines.next().unwrap();
        let mut bi = be.split(|c| c == '[' || c== ',' || c== ' ' || c == ']').skip(1).filter(|s| !s.is_empty()).take(4);
        let mut before = [0; 4];
        before[0] = bi.next().unwrap().parse().unwrap();
        before[1] = bi.next().unwrap().parse().unwrap();
        before[2] = bi.next().unwrap().parse().unwrap();
        before[3] = bi.next().unwrap().parse().unwrap();

        let mut ci = lines.next().unwrap().split_whitespace();
        let cmd = ci.next().unwrap().parse::<usize>().unwrap();
        let a = ci.next().unwrap().parse::<i64>().unwrap();
        let b = ci.next().unwrap().parse::<i64>().unwrap();
        let c = ci.next().unwrap().parse::<usize>().unwrap();

        let ae = lines.next().unwrap();
        let mut ai = ae.split(|c| c == '[' || c == ',' || c == ' ' || c == ']').skip(1).filter(|s| !s.is_empty()).take(4);
        let mut after = [0; 4];
        after[0] = ai.next().unwrap().parse().unwrap();
        after[1] = ai.next().unwrap().parse().unwrap();
        after[2] = ai.next().unwrap().parse().unwrap();
        after[3] = ai.next().unwrap().parse().unwrap();

        Ok( Unit {before, after, cmd, a, b, c })
    }
}

fn main() -> io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let parts: Vec<_> = s.split("\n\n\n").collect();

    // reverse enineer using the first part
    let units: Vec<Unit> = parts[0].split("\n\n").map(|s| {
        s.parse().unwrap()
    }).collect();

    //
    // part one
    // count the units that work on three or more opcodes
    let mut c = Computer::new();
    let opcodes: Vec<_> = c.available_opcodes();
    let uocount = units.iter().map(|u| {
        opcodes.iter().filter(|o| {
            c.test_unit(o, u)
        }).count()
    }).filter(|&n| n >= 3).count();
    println!("16a: {} units work for three or more opcodes", uocount);

    // part two, map all to an opcode
    // split by opcode number
    let mut unitmap = HashMap::new();
    for u in units {
        let e = unitmap.entry(u.cmd).or_insert_with(Vec::new);
        e.push(u);
    }

    // reverse engineer the opcode numbers
    let mut c = Computer::new();
    let mut opcode_map = HashMap::new();
    let mut opcodes_found = HashSet::new();
    let n = c.available_opcodes().len();

    'opcode: while opcode_map.len() < n {
        for (on, list) in unitmap.iter().filter(|(k,_)| !opcode_map.contains_key(k)) {
            let mut opcodes: Vec<_> = c.available_opcodes().into_iter().filter(|c| !opcodes_found.contains(c)).collect();
            for unit in list {
                opcodes.retain(|opc|c.test_unit(opc, unit));
                if opcodes.len() == 1 {
                    opcode_map.insert(on, opcodes[0].to_owned());
                    opcodes_found.insert(opcodes[0].to_owned());
                    continue 'opcode;
                }
            }
        }
    }

    // run the binary
    let mut c = Computer::new();
    let binary: Vec<i64> = parts[1].split_whitespace().map(|s| s.parse().unwrap()).collect();
    for bin in binary.chunks(4) {
        let cmd = opcode_map.get(&(bin[0] as usize)).expect("Illegal instruction");
        c.cmd(cmd, bin[1], bin[2], bin[3] as usize);
    }
    println!("16b: After running the binary register A contains {}", c.registers[0]);

    Ok(())
}
