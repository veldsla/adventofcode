use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct Firewall {
    layers: Vec<Option<Scanner>>,
}

#[derive(Debug)]
struct Scanner {
    depth: usize,
    range: usize,
}

impl FromStr for Scanner {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = s.split(':');
        Ok(Scanner {
            depth: p.next()
                .ok_or(format!("Error parsing Scanner line: {}", s))?
                .parse()
                .map_err(|e| format!("Error parsing scanner depth:{}", e))?,
            range: p.next()
                .ok_or(format!("Error parsing Scanner line: {}", s))?
                .trim().parse()
                .map_err(|e| format!("Error parsing scanner range: {}", e))?,
        })
    }
}

impl Scanner {
    fn at_zero(&self, time: usize) -> bool {
        (time % (self.range * 2 - 2)) == 0
    }
}

impl Firewall {
    fn new() -> Firewall {
        Firewall { layers: Vec::new() }
    }

    fn from_file<P: AsRef<Path>>(p: P) -> Result<Firewall, String> {
        let mut wall = Firewall::new();
        let f = File::open(p).map_err(|e| format!("{}", e))?;
        for line in BufReader::new(f).lines() {
            let line = line.map_err(|e| format!("{}", e))?;
            wall.set_scanner(line.parse()?);
        }
        Ok(wall)
    }

    fn set_scanner(&mut self, s: Scanner) {
        let i = s.depth;
        while self.layers.len() <= i {
            self.layers.push(None);
        }
        self.layers[i] = Some(s);
    }

    fn traverse_now(&self) -> usize {
        self.layers.iter().enumerate()
            .filter(|&(_, layer)| layer.is_some())
            .map(|(time, layer)| {
                let scanner = layer.as_ref().unwrap();
                if scanner.at_zero(time) {
                    time * scanner.range
                } else {
                    0
                }
            }).sum()
    }

    fn traverse_safely(&self) -> usize {
        (0usize..).filter(|time| {
            self.layers.iter().enumerate()
                .all(|(atime, l)| {
                    if let Some(scanner) = l.as_ref() {
                        !scanner.at_zero(*time + atime)
                    } else {
                        true
                    }
                })
        }).nth(0).unwrap()
    }
}

fn main() {
    let firewall = Firewall::from_file("input.txt").unwrap();
    println!("Blindly crossing the firewall at time=0 has a severity of {}", firewall.traverse_now());
    println!("Safe passage at time = {}", firewall.traverse_safely());
    
}

#[test]
fn part_a() {
    let mut f = Firewall::new();
    f.set_scanner("0: 3".parse().unwrap());
    f.set_scanner("1: 2".parse().unwrap());
    f.set_scanner("4: 4".parse().unwrap());
    f.set_scanner("6: 4".parse().unwrap());

    assert_eq!(f.traverse_now(), 24);
}
