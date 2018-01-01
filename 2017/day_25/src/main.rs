#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use std::iter::repeat;
use std::mem;

mod parser;

#[derive(Debug)]
struct State {
    id: char,
    if_0: Action,
    if_1: Action,
}

#[derive(Debug, Copy, Clone)]
struct Action {
    write: u8,
    move_to: Direction,
    next_state: char,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "right" => Ok(Direction::Right),
            "left" => Ok(Direction::Left),
            _ => Err(format!("Cannot parse {} as Direction", s)),
        }
    }
}
#[derive(Debug)]
struct Tape {
    store: Vec<u8>,
    position: usize,
}

impl Tape {
    fn new() -> Tape {
        Tape {
            store: vec![0],
            position: 0,
        }
    }

    fn go(&mut self, d: Direction) {
        if self.position == 0 {
            let mut s = vec![0; 100];
            mem::swap(&mut s, &mut self.store);
            self.store.extend(s.into_iter());
            self.position = 100;
        }

        if self.position == self.store.len() - 1 {
            self.store.extend(repeat(0).take(100));
        }
        match d {
            Direction::Left => self.position -= 1,
            Direction::Right => self.position += 1,
        }
    }

    fn write(&mut self, v: u8) {
        self.store[self.position] = v;
    }

    fn read(&self) -> &u8 {
        &self.store[self.position]
    }

    fn checksum(&self) -> u64 {
        self.store.iter().map(|b| u64::from(*b)).sum()
    }
}

#[derive(Debug)]
struct Machine {
    states: Vec<State>,
    tape: Tape,
    next_state: char,
    check_at: usize,
    counter: usize,
}

impl Machine {
    fn from_file<P: AsRef<Path>>(p: P) -> Result<Machine, String> {
        let mut f = File::open(p).map_err(|e| format!("{}", e))?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).map_err(|e| format!("{}", e))?;
        parser::parse_machine(&buf)
    }

    fn run(&mut self) -> u64 {
        while self.counter < self.check_at {
            let current = *self.tape.read();
            let action = match current {
                0 => self.states[(self.next_state as u8 - b'A') as usize].if_0,
                1 => self.states[(self.next_state as u8 - b'A') as usize].if_1,
                a => panic!("No action for value {}", a),
            };
            self.do_action(action);
            self.counter += 1;
        }
        self.tape.checksum()
    }

    fn do_action(&mut self, a: Action) {
        self.tape.write(a.write);
        self.tape.go(a.move_to);
        self.next_state = a.next_state;
    }
}

fn main() {
    let mut m = Machine::from_file("input.txt").unwrap();
    let checksum = m.run();
    println!("25: Diag at cycle {} is {}", m.counter, checksum);
}
