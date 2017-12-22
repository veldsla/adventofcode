use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Coord((i32, i32));
impl From<(i32, i32)> for Coord {
    fn from(t: (i32, i32)) -> Coord {
        Coord((t.0,t.1))
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy)]
enum Infection {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl Direction {
    fn turn(&mut self, d: Direction) {
        match d {
            Direction::Left => {
                match *self {
                    Direction::Up => *self = Direction::Left,
                    Direction::Down => *self = Direction::Right,
                    Direction::Left => *self = Direction::Down,
                    Direction::Right => *self = Direction::Up
                }
            },
            Direction::Right => {
                match *self {
                    Direction::Up => *self = Direction::Right,
                    Direction::Down => *self = Direction::Left,
                    Direction::Left => *self = Direction::Up,
                    Direction::Right => *self =  Direction::Down
                }
            },
            _ => panic!("Cannot turn up or down")
        }
    }

    fn reverse(&mut self) {
        match *self {
            Direction::Up => *self = Direction::Down,
            Direction::Down => *self = Direction::Up,
            Direction::Left => *self = Direction::Right,
            Direction::Right => *self = Direction::Left
        }
    }
}

impl Coord {
    fn go(&mut self, d: &Direction) {
        match *d {
            Direction::Up => (self.0).1 -= 1,
            Direction::Down => (self.0).1 += 1,
            Direction::Left => (self.0).0 -= 1,
            Direction::Right => (self.0).0 += 1
        }
    }
}

#[derive(Debug, Clone)]
struct Grid(HashMap<Coord, Infection>);

impl Grid {
    fn from_file<P: AsRef<Path>>(p: P) -> Result<Grid, String> {
        let f = File::open(p).map_err(|e| format!("{}", e))?;
        Ok(Grid(BufReader::new(f).lines().enumerate()
             .flat_map(|(row, line)| {
                 line.unwrap()
                     .chars().enumerate()
                     .filter(|s| s.1 == '#')
                     .map(|(col, _)| ( (col as i32, row as i32).into(), Infection::Infected ))
                     .collect::<Vec<(Coord, Infection)>>()
             }).collect()))
    }

    fn infect_from_center(&mut self, n: usize) -> u32 {
        let mut pos = self.middle();
        let mut direction = Direction::Up;
        let mut infected = 0;
        for _ in 0..n {
            let infection = self.0.entry(pos).or_insert(Infection::Clean);
            match *infection {
                Infection::Weakened => {
                    *infection = Infection::Infected;
                    //self.0.insert(pos, Infection::Infected);
                    infected +=1;
                },
                Infection::Infected => {
                    *infection = Infection::Flagged;
                    direction.turn(Direction::Right);
                },
                Infection::Flagged => {
                    *infection = Infection::Clean;
                    direction.reverse();
                },
                Infection::Clean => {
                    *infection = Infection::Weakened;
                    direction.turn(Direction::Left);
                }
            }
            pos.go(&direction);
        }
        infected
    }

    fn infect_simple_from_center(&mut self, n: usize) -> u32 {
        let mut pos = self.middle();
        let mut direction = Direction::Up;
        let mut infected = 0;
        for _ in 0..n {
            if self.0.remove(&pos).is_some() {
                direction.turn(Direction::Right);
            } else {
                direction.turn(Direction::Left);
                self.0.insert(pos, Infection::Infected);
                infected +=1;
            }
            pos.go(&direction);
        }
        infected
    }

    fn middle(&self) -> Coord {
        let m = self.0.keys().map(|&Coord((x,y))| std::cmp::max(x, y)).max().unwrap();
        debug_assert!(m % 2 == 0);
        let c = m / 2;
        Coord((c, c))
    }
}

fn main() {
    let mut grid = Grid::from_file("input.txt").unwrap();
    println!("22a: {} have been infected during 10K rounds",
             grid.clone().infect_simple_from_center(10_000));
    println!("22b: {} have been infected during 10M rounds",
             grid.infect_from_center(10_000_000));
}

#[test]
fn test_a() {
    let mut grid = Grid::from_file("test.txt").unwrap();
    assert_eq!(grid.infect_simple_from_center(10_000),5587);
}

#[test]
fn test_b() {
    let mut grid = Grid::from_file("test.txt").unwrap();
    assert_eq!(grid.infect_from_center(10_000_000),2511944);
}
