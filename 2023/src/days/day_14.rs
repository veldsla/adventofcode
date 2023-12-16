use std::collections::HashMap;
use std::fmt;
use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::is_a,
    character::complete::line_ending,
    multi::fold_many1,
    combinator::{all_consuming, eof, map},
    sequence::terminated,
    IResult
};


use crate::Problem;
use crate::grid::{Grid, Coord};

#[derive(Default)]
pub struct Solution {
    platform: Platform,
}

#[derive(Debug, Default, Clone)]
struct Platform {
    rocks: Grid<Rock>,
}

#[derive(Debug, Default, Eq, Hash, PartialEq, Copy, Clone)]
enum Rock {
    Fixed,
    Moving,
    #[default]
    NoRock,
}

impl fmt::Display for Rock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rock::Fixed => write!(f, "#"),
            Rock::Moving => write!(f, "O"),
            Rock::NoRock => write!(f, "."),
        }
    }
}

impl Platform {
    fn tilt_up(&mut self) {
        // try a naive approach first
        // from the top, skip row one
        // for each row move each moving rock up one position if possible
        
        for row in 1..self.rocks.dim_y as isize {
            for col in 0..self.rocks.dim_x as isize {
                let from: Coord = (col, row).into();
                if self.rocks[from] == Rock::Moving {
                    // move up while possible
                    let mut new_pos: Coord = (col, row-1).into();
                    while new_pos.y >= 0 && self.rocks[new_pos] == Rock::NoRock {
                        new_pos.y -= 1;
                    }
                    new_pos.y += 1;
                    if new_pos != from {
                        self.rocks[new_pos] = Rock::Moving;
                        self.rocks[from] = Rock::NoRock;
                    }
                }
            }
        }
    }

    fn tilt_down(&mut self) {
        for row in (0..self.rocks.dim_y as isize-1).rev() {
            for col in 0..self.rocks.dim_x as isize {
                let from: Coord = (col, row).into();
                if self.rocks[from] == Rock::Moving {
                    // move down while possible
                    let mut new_pos: Coord = (col, row+1).into();
                    while new_pos.y < self.rocks.dim_y as isize && self.rocks[new_pos] == Rock::NoRock {
                        new_pos.y += 1;
                    }
                    new_pos.y -= 1;
                    if new_pos != from {
                        self.rocks[new_pos] = Rock::Moving;
                        self.rocks[from] = Rock::NoRock;
                    }
                }
            }
        }
    }

    fn tilt_right(&mut self) {
        for col in (0..self.rocks.dim_x as isize-1).rev() {
            for row in 0..self.rocks.dim_y as isize {
                let from: Coord = (col, row).into();
                if self.rocks[from] == Rock::Moving {
                    // move right while possible
                    let mut new_pos: Coord = (col+1, row).into();
                    while new_pos.x < self.rocks.dim_x as isize && self.rocks[new_pos] == Rock::NoRock {
                        new_pos.x += 1;
                    }
                    new_pos.x -= 1;
                    if new_pos != from {
                        self.rocks[new_pos] = Rock::Moving;
                        self.rocks[from] = Rock::NoRock;
                    }
                }
            }
        }
    }

    fn tilt_left(&mut self) {
        for col in 1..self.rocks.dim_x as isize {
            for row in 0..self.rocks.dim_y as isize {
                let from: Coord = (col, row).into();
                if self.rocks[from] == Rock::Moving {
                    // move left while possible
                    let mut new_pos: Coord = (col-1, row).into();
                    while new_pos.x >= 0 && self.rocks[new_pos] == Rock::NoRock {
                        new_pos.x -= 1;
                    }
                    new_pos.x += 1;
                    if new_pos != from {
                        self.rocks[new_pos] = Rock::Moving;
                        self.rocks[from] = Rock::NoRock;
                    }
                }
            }
        }
    }

    fn load(&self) -> usize {
        let nrow = self.rocks.dim_y;
        (0..self.rocks.dim_y)
            .map(|x| (nrow - x) * self.rocks.iter_row(x).filter(|r| **r == Rock::Moving).count())
            .sum()
    }
}

fn parse_grid(s: &str) -> IResult<&str, Grid<Rock>> {
    let gridline = terminated(is_a(".O#"), alt((line_ending, eof)));
    let grid = fold_many1(gridline, || (Vec::new(), 0usize, 0usize), |(mut v, _x, y), line: &str| {
        //dimnension checks?
        let x = line.len();
        v.extend(line.chars().map(|c| match c {
            '#' => Rock::Fixed,
            'O' => Rock::Moving,
            '.' => Rock::NoRock,
            _ => unreachable!(),
        }));
        (v, x, y+1)
    });
    map(all_consuming(grid), |(elements, dim_x, dim_y)| Grid::new(elements, dim_x, dim_y))(s)
}


impl Problem for Solution {
    fn parse(&mut self, s: &str) -> Result<()> {
        let (_s, grid) = parse_grid(s).map_err(|e| anyhow!("parse error: {e}"))?;
        self.platform = Platform { rocks: grid };
        Ok(())

    }

    fn part1(&self) -> Result<String> {
        let mut platform = self.platform.clone();

        platform.tilt_up();

        Ok(platform.load().to_string())
    }

    fn part2(&self) -> Result<String> {

        let mut platform = self.platform.clone();

        let mut load_cache = HashMap::new();
        let mut it = 0;
        let mut todo = None;

        loop {
            platform.tilt_up();
            platform.tilt_left();
            platform.tilt_down();
            platform.tilt_right();
            it += 1;
            let load = platform.load();
            if let Some(prev_it) = load_cache.get(&platform.rocks.elements) {
                // cycle detected
                if todo.is_none() {
                    //eprintln!("cycle detected: {} -> {}", prev_it, it);
                    let cycle_len = it - prev_it;
                    todo = Some((1_000_000_000 - it) % cycle_len);
                }
            } else {
                load_cache.insert(platform.rocks.elements.clone(), it);
            }

            if let Some(mut len) = todo {
                if len == 0 {
                    return Ok(load.to_string());
                } else {
                    len -= 1;
                    todo = Some(len);
                }
            }
        }
    }
}

