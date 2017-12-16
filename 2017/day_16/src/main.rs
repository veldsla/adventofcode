use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct DanceLine(Vec<Dancer>);
#[derive(Clone, Copy, Eq, PartialEq)]
struct Dancer(char);

enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(Dancer, Dancer)
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = s.chars();
        let m = p.next().ok_or(format!("Error in line: {}", s))?;
        let v = p.as_str();
        match m {
            's' => {
                let n = v.parse().map_err(|e| format!("Error in spin move: {}", e))?;
                Ok(Move::Spin(n))
            },
            'x' => {
                let ab = v.split('/')
                    .map(|pos| pos.parse::<usize>())
                    .collect::<Result<Vec<usize>, _>>().map_err(|e| format!("Error in exchange move: {}: {}",s , e))?;
                Ok(Move::Exchange(ab[0], ab[1]))
            },
            'p' => {
                let mut p = v.chars();
                let a: char = p.next().ok_or(format!("Error in partner move for {}", v))?;
                let _: char = p.next().ok_or(format!("Error in partner move for {}", v))?;
                let b: char = p.next().ok_or(format!("Error in partner move for {}", v))?;
                    
                Ok(Move::Partner(Dancer(a), Dancer(b)))
            },
            _ => Err(format!("Unknown move in line: {}", s))
        }
    }
}

impl DanceLine {
    fn new() -> DanceLine {
        DanceLine("abcdefghijklmnop".chars().map(|c| Dancer(c)).collect())
    }

    fn dance(&mut self, moves: &[Move]) {
        for m in moves {
            self.do_move(m);
        }
    }

    fn do_move(&mut self, m: &Move) {
        match m {
            &Move::Spin(n) => self.spin_line(n),
            &Move::Exchange(a, b) => self.0.swap(a, b),
            &Move::Partner(a, b) => self.swap_dancer(a, b),
        }
    }

    fn spin_line(&mut self, n: usize) {
        let n = self.0.len() - (n % self.0.len());
        self.0[..n].reverse();
        self.0[n..].reverse();
        self.0.reverse();
    }

    fn swap_dancer(&mut self, a: Dancer, b: Dancer) {
        let pos_a = self.0.iter().position(|&d| d == a).unwrap();
        let pos_b = self.0.iter().position(|&d| d == b).unwrap();
        self.0.swap(pos_a, pos_b);
    }

    fn line_order(&self) -> String {
        self.0.iter().map(|d| d.0).collect()
    }
}

fn analyze_dance(d: &[Move]) -> usize {
    let mut danceline = DanceLine::new();
    for o in 0.. {
        for  m in d {
            danceline.do_move(m);
        }
        if danceline.line_order() == "abcdefghijklmnop" {
            return o + 1;
        }
    }
    unreachable!()
}



fn main() {
    let f = File::open("input.txt").unwrap_or_else(|e| panic!("Error opening file:\n\t{}", e));
    let mut danceline = DanceLine::new();
    let moves: Vec<Move> = BufReader::new(f).lines().nth(0).unwrap().unwrap()
        .split(',').map(|l| l.parse::<Move>().unwrap()).collect();

    danceline.dance(&moves);
    println!("15a: Dance line after one round is: {}", danceline.line_order());

    let mut danceline = DanceLine::new();
    let repeat = analyze_dance(&moves);
    print!("15b: Repeat in dance found after {} rounds. ", repeat);
    for _ in 0..(1_000_000_000 % repeat) {
        danceline.dance(&moves);
    }
    println!("Dance line after one biiilion rounds is: {}", danceline.line_order());
}
