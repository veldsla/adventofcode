extern crate regex;
use regex::Regex;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;


#[derive(Clone, Debug)]
struct Connection {
    source1: Source,
    source2: Option<Source>,
    operator: Option<Operator>
}

#[derive(Clone, Debug)]
enum Source {
    Value(u16),
    Name(String)
}

#[derive(Clone, Debug)]
enum Operator {
    AND,
    NOT,
    OR,
    LSHIFT,
    RSHIFT
}

#[derive(Debug)]
struct Circuit {
    wires: HashMap<String, Connection>,
    values: HashMap<String, u16>
}

impl Circuit {
    fn new() -> Circuit {
        let hm = HashMap::new();
        Circuit {wires: hm, values: HashMap::new()}
    }

    fn add_connection(&mut self, s: &str) {
        let notre = Regex::new(r"^NOT (\w+) -> (\w+)$").unwrap();
        let opre = Regex::new(r"^(\w+) (AND|OR|LSHIFT|RSHIFT) (\w+) -> (\w+)$").unwrap();
        let assre = Regex::new(r"^(\w+) -> (\w+)$").unwrap();
        let allnum = |s : &str| s.chars().all(|c| c.is_numeric());

        let (name, connection) = if notre.is_match(s) {
            let caps = notre.captures(s).unwrap();
            let name = caps.at(2).unwrap();
            let s1 = caps.at(1).unwrap();
            let source = if allnum(s1) {
                Source::Value(s1.parse::<u16>().unwrap())
            } else {
                Source::Name(s1.to_string())
            };
            (name, Connection {source1: source, source2: None, operator: Some(Operator::NOT)})
        } else if opre.is_match(s) {
            let caps = opre.captures(s).unwrap();
            let name = caps.at(4).unwrap();
            let s1 = caps.at(1).unwrap();
            let source1 = if allnum(s1) {
                Source::Value(s1.parse::<u16>().unwrap())
            } else {
                Source::Name(s1.to_string())
            };

            let s1 = caps.at(3).unwrap();
            let source2 = if allnum(s1) {
                Source::Value(s1.parse::<u16>().unwrap())
            } else {
                Source::Name(s1.to_string())
            };

            let o = caps.at(2).unwrap();
            let op = match o {
                "AND"=> {Some(Operator::AND)},
                "OR" => {Some(Operator::OR)},
                "LSHIFT" => {Some(Operator::LSHIFT)},
                "RSHIFT" => {Some(Operator::RSHIFT)},
                _ => panic!("Operator unknown")
            };


            (name, Connection {source1: source1, source2: Some(source2), operator: op})

        } else if assre.is_match(s) {
            let caps = assre.captures(s).unwrap();
            let name = caps.at(2).unwrap();
            let s1 = caps.at(1).unwrap();
            let source = if allnum(s1) {
                Source::Value(s1.parse::<u16>().unwrap())
            } else {
                Source::Name(s1.to_string())
            };
            (name, Connection {source1: source, source2: None, operator: None})
            
        } else {
            panic!("Cannot parse");
        };

        self.wires.insert(name.to_string(), connection);
    }

    fn get_wire_value(&mut self, name: String) -> u16 {
        if let Some(v) = self.values.get(&name) {
            return *v;
        }
        
        let con = self.wires.get(&name).unwrap().clone();
        //get the values from the connections:
        let v1 :u16 = match con.source1.clone() {
            Source::Value(x) => x,
            Source::Name(n)  => self.get_wire_value(n)
        };
        let v2: Option<u16> = match con.source2.clone() {
            Some(Source::Value(x)) => Some(x),
            Some(Source::Name(n))  => Some(self.get_wire_value(n)),
            None => None
        };
        let v = match con.operator {
            Some(Operator::AND) => v1 & v2.unwrap(),
            Some(Operator::NOT) => !v1,
            Some(Operator::OR) => v1 | v2.unwrap(),
            Some(Operator::LSHIFT) => v1 << v2.unwrap(),
            Some(Operator::RSHIFT) => v1 >> v2.unwrap(),
            None => v1
        };
        self.values.insert(name, v);
        v
    }

    fn reset(&mut self) {
        self.values.clear();
    }
}

fn main() {
    let f = File::open("7_in.txt").unwrap();
    let reader = BufReader::new(f);

    let mut circuit = Circuit::new();
    for l in reader.lines() {
        let line = l.ok().unwrap();
        circuit.add_connection(line.as_ref());
    }

    let signal_a = circuit.get_wire_value("a".to_string());
    println!("The value of a is {}", signal_a);

    circuit.reset();
    circuit.add_connection("3176 -> b");
    println!("The value of a after setting b to 3176 is {}", circuit.get_wire_value("a".to_string()));
}

#[test]
fn test() {
   let mut c = Circuit::new();

    c.add_connection("123 -> x");
    c.add_connection("456 -> y");
    c.add_connection("x AND y -> d");
    c.add_connection("x OR y -> e");
    c.add_connection("x LSHIFT 2 -> f");
    c.add_connection("y RSHIFT 2 -> g");
    c.add_connection("NOT x -> h");
    c.add_connection("NOT y -> i");

    assert_eq!(c.get_wire_value("d".to_string()), 72);
    assert_eq!(c.get_wire_value("e".to_string()), 507);
    assert_eq!(c.get_wire_value("f".to_string()), 492);
    assert_eq!(c.get_wire_value("h".to_string()), 65412);
    assert_eq!(c.get_wire_value("i".to_string()), 65079);
    assert_eq!(c.get_wire_value("x".to_string()), 123);
    assert_eq!(c.get_wire_value("y".to_string()), 456);
}
