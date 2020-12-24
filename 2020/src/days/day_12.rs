use std::str;

use anyhow::{anyhow, Result};

use crate::Problem;
use crate::parsers::positive_integer;

#[derive(Default)]
pub struct Solution {
    program: Vec<Move>
}

use nom::{
    character::complete::{anychar, line_ending},
    multi::many1,
    combinator::{all_consuming, map},
    sequence::{tuple, terminated},
    IResult
};

#[derive(Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
    direction: i32,
}

#[derive(Clone, Copy)]
enum Move {
    North(i32),
    South(i32),
    West(i32),
    East(i32),
    TurnLeft(i32),
    TurnRight(i32),
    Forward(i32),
}
use Move::*;

impl Position {
    fn do_move(&mut self, m: Move) {
        match m {
            North(n) => self.y += n,
            South(n) => self.y -= n,
            West(n) => self.x -= n,
            East(n) => self.x += n,
            TurnLeft(n) => self.turn(-n),
            TurnRight(n) => self.turn(n),
            Forward(n) => self.move_forward(n),
        }
    }

    //n right degrees
    fn turn(&mut self, n: i32) {
        self.direction = (self.direction + n).rem_euclid(360);
    }

    fn move_forward(&mut self, n: i32) {
        match self.direction {
            0   => self.do_move(North(n)),
            90  => self.do_move(East(n)),
            180 => self.do_move(South(n)),
            270 => self.do_move(West(n)),
            _   => panic!("Only square angles")
        }
    }

    fn dist_to_origin(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn turn_around(&mut self, n: i32, other: &Position) {
        let (dx, dy) = other.delta_to(self);
        match n {
            0   => { },
            90  | -270 => { self.x = other.x + dy; self.y = other.y - dx;},
            180 | -180 => { self.x = other.x - dx; self.y = other.y - dy; },
            270 |  -90 => { self.x = other.x - dy; self.y = other.y + dx },
            _   => panic!("Only square angles")
        }

    }

    fn delta_to(&self, other: &Position) -> (i32, i32) {
        (other.x - self.x, other.y - self.y)
    }
}

fn parse(i: &str) -> IResult<&str, Vec<Move>> {
    let line = map(tuple((anychar, positive_integer)), |(d, n)| {
        match d {
            'N' => North(n),
            'S' => South(n),
            'E' => East(n),
            'W' => West(n),
            'L' => TurnLeft(n),
            'R' => TurnRight(n),
            'F' => Forward(n),
            _ => panic!("parse error")
        }
    });

    all_consuming(many1(terminated(line, line_ending)))(i)
}

fn follow_da_waypoint(boat: &mut Position, waypoint: &mut Position, program: &[Move]) {
    for  &m in program {
        match m {
            North(n) => waypoint.y += n,
            South(n) => waypoint.y -= n,
            West(n) => waypoint.x -= n,
            East(n) => waypoint.x += n,
            TurnLeft(n) => waypoint.turn_around(-n, boat),
            TurnRight(n) => waypoint.turn_around(n, boat),
            Forward(n) => {
                let (dx, dy) = boat.delta_to(waypoint);
                boat.x += dx * n;
                boat.y += dy * n;
                waypoint.x += dx * n;
                waypoint.y += dy * n;
            }
        }
    }
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(str::from_utf8(i)?).map_err(|e| anyhow!(e.to_string()))?;
        self.program = result.1;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let mut boat = Position { x: 0, y: 0, direction: 90 };
        for &m in &self.program {
            boat.do_move(m);
        }
        Ok(format!("{}", boat.dist_to_origin()))
    }

    fn part2(&self) -> Result<String> {
        let mut boat = Position { x: 0, y: 0, direction: 90 };
        let mut waypoint = Position { x: 10, y: 1, direction: 00 };
        follow_da_waypoint(&mut boat, &mut waypoint, &self.program);
        Ok(format!("{}", boat.dist_to_origin()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "F10\nN3\nF7\nR90\nF11\n";

    #[test]
    fn p1() {
        let result = parse(TEST);
        assert!(result.is_ok());
        let program = result.unwrap().1;
        let mut boat = Position { x: 0, y: 0, direction: 90 };
        for &m in &program {
            boat.do_move(m);
        }
        assert_eq!(boat.dist_to_origin(), 25);
    }

    #[test]
    fn p2() {
        let result = parse(TEST);
        let program = result.unwrap().1;
        let mut boat = Position { x: 0, y: 0, direction: 90 };
        let mut waypoint = Position { x: 10, y: 1, direction: 00 };
        follow_da_waypoint(&mut boat, &mut waypoint, &program);
        assert_eq!(boat.dist_to_origin(), 286);
    }
}
