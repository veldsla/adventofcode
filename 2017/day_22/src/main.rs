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
#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy)]
enum Infection {
    Weakened,
    Infected,
    Flagged,
}

impl Direction {
    fn turn(&self, d: Direction) -> Direction {
        match d {
            Direction::Left => {
                match self {
                    &Direction::Up => Direction::Left,
                    &Direction::Down => Direction::Right,
                    &Direction::Left => Direction::Down,
                    &Direction::Right => Direction::Up
                }
            },
            Direction::Right => {
                match self {
                    &Direction::Up => Direction::Right,
                    &Direction::Down => Direction::Left,
                    &Direction::Left => Direction::Up,
                    &Direction::Right => Direction::Down
                }
            },
            _ => panic!("Cannot turn up or down")
        }
    }

    fn reverse(&self) -> Direction {
        match self {
            &Direction::Up => Direction::Down,
            &Direction::Down => Direction::Up,
            &Direction::Left => Direction::Right,
            &Direction::Right => Direction::Left
        }

    }
}

impl Coord {
    fn go(&mut self, d: &Direction) {
        match d {
            &Direction::Up => (self.0).1 -= 1,
            &Direction::Down => (self.0).1 += 1,
            &Direction::Left => (self.0).0 -= 1,
            &Direction::Right => (self.0).0 += 1
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
            if let Some(infection) = self.0.remove(&pos) {
                match infection {
                    Infection::Weakened => {
                        self.0.insert(pos.clone(), Infection::Infected);
                        infected +=1;
                    },
                    Infection::Infected => {
                        self.0.insert(pos.clone(), Infection::Flagged);
                        direction = direction.turn(Direction::Right);
                    },
                    Infection::Flagged => {
                        direction = direction.reverse();
                    },
                }
            } else {
                direction = direction.turn(Direction::Left);
                self.0.insert(pos.clone(), Infection::Weakened);
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
            if let Some(_) = self.0.remove(&pos) {
                direction = direction.turn(Direction::Right);
            } else {
                direction = direction.turn(Direction::Left);
                self.0.insert(pos.clone(), Infection::Infected);
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
fn test() {
    let mut grid = Grid::from_file("test.txt").unwrap();
    assert_eq!(grid.infect(10_000_000),2511944);
}
