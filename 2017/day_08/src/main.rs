use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

struct Registers(HashMap<String, i32>);

impl Registers {
    fn new() -> Registers {
        Registers(HashMap::new())
    }

    fn entry(&mut self, n: &str) -> &mut i32 {
        self.0.entry(n.to_string()).or_insert(0)
    }

    fn inc(&mut self, n: &str, val: i32) -> i32 {
        self.dec(n, -val)
    }

    fn dec(&mut self, n: &str, val: i32) -> i32 {
        let r = self.entry(n);
        *r -= val;
        *r
    }

    fn max_value(&self) -> i32 {
        self.0.values().cloned().max().unwrap_or(0)
    }
}

fn main() {
    let f = File::open("input.txt").unwrap_or_else(|e| panic!("Error opening file:\n\t{}", e));
    let mut reg = Registers::new();
    let max_value = BufReader::new(f).lines().map(|line| {
        let line = line.unwrap();
        line.split_whitespace().map(|w| w.to_string()).collect::<Vec<_>>()
        // p what
        // 0 reg name
        // 1 inc or dec
        // 2 i32
        // 3 if
        // 4 reg name
        // 5 comparator
        // 6 value
    }).filter_map(|p| {
        let a = *reg.entry(&p[4]);
        let b: i32 = p[6].parse().unwrap();
        let pass = match p[5].as_str() {
            "==" => a == b,
            "!=" => a != b,
            ">=" => a >= b,
            "<=" => a <= b,
            ">" => a > b,
            "<" => a < b,
            _ => panic!("unknown comparison operator: {}", p[5])
        };
        if pass {
            let new_val = match p[1].as_str() {
                "inc" => reg.inc(&p[0], p[2].parse::<i32>().unwrap()),
                "dec" => reg.dec(&p[0], p[2].parse::<i32>().unwrap()),
                _ => panic!("Unknown operator: {}", p[1])
            };
            Some(new_val)
        } else {
            None
        }
    }).max().unwrap();

    println!("Largest value is {}, maximum value encountered was {}", reg.max_value(), max_value);
}

#[test]
fn test() {
    let mut r = Registers::new();
    if *r.entry("a") > 1 { r.inc("b", 5); }
    if *r.entry("b") < 5 { r.inc("a", 1); }
    if *r.entry("a") >= 1 { r.dec("c", -10); }
    if *r.entry("c") == 10 { r.inc("c", -20); }

    assert_eq!(r.max_value(), 1);
}

