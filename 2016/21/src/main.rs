extern crate permutohedron;

use permutohedron::Heap;

use std::fs::File;
use std::io::{BufReader, BufRead};

struct AoCHasher {
    hash: Vec<char>
}

impl AoCHasher {
    fn new(s: &str) -> AoCHasher {
        AoCHasher { hash: s.chars().collect() }
    }

    fn swap(&mut self, x: usize, y: usize) {
        self.hash.swap(x,y);
    }

    fn swap_char(&mut self, x: char, y: char) {
        let px = self.hash.iter().position(|&c| c == x).unwrap();
        let py = self.hash.iter().position(|&c| c == y).unwrap();
        self.hash.swap(px,py);
    }

    fn rotate_left(&mut self, amount: usize) {
        let l = self.hash.len();
        self.hash[0..amount].reverse();
        self.hash[amount..l].reverse();
        self.hash.reverse();
    }

    fn rotate_right(&mut self, amount: usize) {
        let l = self.hash.len();
        self.rotate_left(l - amount);        
    }
    
    fn rotate_char(&mut self, x: char) {
        let px = self.hash.iter().position(|&c| c == x).unwrap();
        let extra = if px > 3  {2} else {1};
        let amount = (px + extra) % self.hash.len();

        self.rotate_right(amount);        
    }

    fn reverse_range(&mut self, start: usize, end: usize) {
        self.hash[start..end+1].reverse();
    }

    fn move_char(&mut self, x: usize, y: usize) {
        let c = self.hash.remove(x);
        self.hash.insert(y,c);
    }
    
    fn do_command(&mut self, s: &str) {
        let p = s.split(" ").collect::<Vec<_>>();
        match p[0] {
            "rotate" => {
                match p[1] {
                    "right" => self.rotate_right(p[2].parse().unwrap()),
                    "left" => self.rotate_left(p[2].parse().unwrap()),
                    _ => self.rotate_char(p[6].chars().nth(0).unwrap())
                }
            },
            "swap" => {
                match p[1] {
                    "letter" => self.swap_char(p[2].chars().nth(0).unwrap(),p[5].chars().nth(0).unwrap()),
                    "position" => self.swap(p[2].parse().unwrap(), p[5].parse().unwrap()),
                    _ => panic!("error in swap command")
                }
            },
            "move" => {
                self.move_char(p[2].parse().unwrap(), p[5].parse().unwrap())
            },
            "reverse" => {
                self.reverse_range(p[2].parse().unwrap(), p[4].parse().unwrap())
            },
            _ => panic!("Error parsing command")
        }
    }

    fn get_hash(&self) -> String {
        self.hash.iter().cloned().collect()
    }

}

fn brute_force(hash: &str, input: &str, program: &Vec<String>) -> Option<String> {

    let mut data: Vec<char> = input.chars().collect();
    let heap = Heap::new(&mut data);
    for data in heap {
        let mut h = AoCHasher {hash: data.clone()};
        for l in program {
            h.do_command(l)
        }
        if h.get_hash() == hash {
            return Some(data.iter().cloned().collect());
        }
    }
    None
}

fn main() {
    let f = File::open("input.txt").unwrap_or_else(|_| panic!("input.txt not found"));
    let mut h = AoCHasher::new("abcdefgh");
    let process: Vec<String> = BufReader::new(f).lines().map(|l| l.unwrap()).collect();
    for l in &process {
        h.do_command(l)
    }
    println!("The hash for abcdefgh = {}", h.get_hash());

    //let see if we can brute force the hash instead of reverse the hashing :-)
    //there are only 8! = 40320 possibilities, so don't bother storing the parsed values
    //BUAHAHAHA...oops
    let hash = "fbgdceah";
    println!("The decoded hash for fbgdceah = {}", brute_force(hash,"abcdefgh",  &process).unwrap_or_else(|| panic!("No pw matches hashed input")));
}

#[test]
fn test() {
    let input = "\
swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d
";

    let mut h = AoCHasher::new("abcde");
    for l in input.lines() {
        println!("{}", h.get_hash());
         h.do_command(&l);
    }
    assert_eq!(h.get_hash(), "decab");
}
