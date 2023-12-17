use std::cmp::{min, max};
use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    character::complete::{digit1, line_ending},
    multi::fold_many1,
    combinator::{all_consuming, eof, map},
    sequence::terminated,
    IResult
};

use crate::Problem;
use crate::grid::{Grid, Coord};

#[derive(Default)]
pub struct Solution {
    city: Grid<u8>
}

/// We can arrive at a location from four directions
/// having either 1, 2, 3 straight steps remaining in any direction
/// we store all 16 possible scores
#[derive(Clone, Default)]
struct RouteScore {
    scores: [Option<usize>; 16],
}

impl RouteScore {
    fn score(&self, direction: Direction, straight_counter: usize) -> Option<usize> {
        self.scores[direction as usize * 4 + straight_counter]
    }

    fn set_score(&mut self, direction: Direction, straight_counter: usize, score: usize) {
        // when we enter a cell from any direction we can have 0, 1, 2 straight steps remaining
        // we can update the score for many directions at once
        // when we turn we reset the straight counter so update if the score is lower
        // when we go straight we update if the score is lower for the same straight counter or
        // higher
        
        /*
          this appears to be wrong
        let mut left = direction.clone();
        left.turn_left();
        for i in 1..3 {
            let idx = left as usize * 4 + i;
            self.scores[idx] = Some(self.scores[idx].map_or(score, |s| s.min(score)));
        }

        let mut right = direction.clone();
        right.turn_right();
        for i in 1..3 {
            let idx = right as usize * 4 + i;
            self.scores[idx] = Some(self.scores[idx].map_or(score, |s| s.min(score)));
        }
        */

        for i in straight_counter..4 {
            let idx = direction as usize * 4 + i;
            self.scores[idx] = Some(self.scores[idx].map_or(score, |s| s.min(score)));
        }
    }
}

/// the heavy crubile walks betwen 4 and 10 steps
#[derive(Clone, Default)]
struct RouteScoreHeavy {
    scores: [Option<usize>; 7*4],
}

impl RouteScoreHeavy {
    fn score(&self, direction: Direction, straight_counter: usize) -> Option<usize> {
        self.scores[direction as usize * 7 + straight_counter - 4]
    }

    fn set_score(&mut self, direction: Direction, straight_counter: usize, score: usize) {
        // when we enter a cell from any direction we can have 0, 1, 2 straight steps remaining
        // we can update the score for many directions at once
        // when we turn we reset the straight counter so update if the score is lower
        // when we go straight we update if the score is lower for the same straight counter or
        // higher
        
        /*
          this appears to be wrong
        let mut left = direction.clone();
        left.turn_left();
        for i in 1..3 {
            let idx = left as usize * 7 + i;
            self.scores[idx] = Some(self.scores[idx].map_or(score, |s| s.min(score)));
        }

        let mut right = direction.clone();
        right.turn_right();
        for i in 1..3 {
            let idx = right as usize * 7 + i;
            self.scores[idx] = Some(self.scores[idx].map_or(score, |s| s.min(score)));
        }
        */

        for i in straight_counter..11 {
            let idx = direction as usize * 7 + i - 4;
            self.scores[idx] = Some(self.scores[idx].map_or(score, |s| s.min(score)));
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
enum Direction {
    #[default]
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[derive(Debug, Clone, Copy, Default)]
struct Walker {
    location: Coord,
    direction: Direction,
    straight_counter: usize,
    score: usize,

}

impl Direction {
    fn turn_left(&mut self) {
        *self = match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl Walker {
    // transform this walker into the new walkers for the allowed directions
    // only three steps in the same direction allowed
    fn allowed_steps(mut self) -> Vec<Walker> {
        let mut walkers = Vec::new();
        // turn left
        let mut left = self.clone();
        left.turn(Direction::Left);
        left.move_n(1);
        walkers.push(left);

        // turn right
        let mut right = self.clone();
        right.turn(Direction::Right);
        right.move_n(1);
        walkers.push(right);

        // go straight if allowed
        if self.straight_counter < 3 {
            self.move_n(1);
            walkers.push(self);
        }

        walkers
    }

    //the heavy stepper can only step 4 or more times when going straight
    fn allowed_steps_heavy(self) -> Vec<Walker> {
        let mut walkers = Vec::new();
        // turn left
        let mut left = self.clone();
        left.turn(Direction::Left);
        left.move_n(4);
        walkers.push(left);

        // turn right
        let mut right = self.clone();
        right.turn(Direction::Right);
        right.move_n(4);
        walkers.push(right);

        // go straight if allowed
        if self.straight_counter >= 4 && self.straight_counter < 10 {
            let mut straight = self.clone();
            straight.move_n(1);
            walkers.push(straight);
        }

        //eprintln!("Allowed steps for {:?}:  {:?}", self,  walkers);

        walkers
    }

    fn turn(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.direction.turn_left(),
            Direction::Right => self.direction.turn_right(),
            _ => panic!("Cannot step forward in direction {:?}", direction),
        }
        self.straight_counter = 0;
    }

    fn move_n(&mut self, amount: usize) {
        match self.direction {
            Direction::Up => self.location.y -= amount as isize,
            Direction::Down => self.location.y += amount as isize,
            Direction::Left => self.location.x -= amount as isize,
            Direction::Right => self.location.x += amount as isize,
        }
        self.straight_counter += amount;
    }
}



impl Solution {
    /// Find the coolest route through the city
    fn coolest_route(&self) -> usize {
        let start1 = Walker {
            location: Coord { x: 0, y: 0 },
            direction: Direction::Right,
            straight_counter: 0,
            score: 0,
        };

        let start2 = Walker {
            location: Coord { x: 0, y: 0 },
            direction: Direction::Down,
            straight_counter: 0,
            score: 0,
        };


        let end = Coord { x: self.city.dim_x as isize - 1, y: self.city.dim_y as isize - 1 };

        let mut costs = Grid::new(vec![RouteScore::default(); self.city.dim_x * self.city.dim_y], self.city.dim_x, self.city.dim_y);

        let mut queue = Vec::new();
        queue.push(start2);
        queue.push(start1);

        let mut it = 0;
        while !queue.is_empty() {
            it += 1;
            let walker = queue.pop().unwrap();
            
            for mut next in walker.allowed_steps().into_iter().filter(|w| self.city.in_bounds(w.location)) {
                //eprintln!("next: {:?}", next);
                // update the score for the next location
                next.score += self.city[next.location] as usize;
                // if the score is better than previous, update the score
                // and continue walking, otherwise stop
                if let Some(score) = costs[next.location].score(next.direction, next.straight_counter) {
                    if next.score >= score {
                        continue;
                    }
                }
                // update the score
                costs[next.location].set_score(next.direction, next.straight_counter, next.score);
                // add the next walker to the queue
                queue.push(next);
            }
        }
        eprintln!("Iterations: {}", it);

        *costs[end].scores.iter().flatten().min().unwrap()
    }

    fn coolest_route_heavy (&self) -> usize {
        let start1 = Walker {
            location: Coord { x: 0, y: 0 },
            direction: Direction::Right,
            straight_counter: 0,
            score: 0,
        };

        let start2 = Walker {
            location: Coord { x: 0, y: 0 },
            direction: Direction::Down,
            straight_counter: 0,
            score: 0,
        };


        let end = Coord { x: self.city.dim_x as isize - 1, y: self.city.dim_y as isize - 1 };

        let mut costs = Grid::new(vec![RouteScoreHeavy::default(); self.city.dim_x * self.city.dim_y], self.city.dim_x, self.city.dim_y);

        let mut queue = Vec::new();
        queue.push(start2);
        queue.push(start1);

        let mut it = 0;
        while !queue.is_empty() {
            it += 1;
            let walker = queue.pop().unwrap();
            
            for mut next in walker.allowed_steps_heavy().into_iter().filter(|w| self.city.in_bounds(w.location)) {
                //eprintln!("next: {:?}", next);
                // update the score for the path to the next location
                let x1 = walker.location.x;
                let y1 = walker.location.y;
                let x2 = next.location.x;
                let y2 = next.location.y;

                if x1 == x2 {
                    for y in min(y1, y2)..=max(y1, y2) {
                        next.score += self.city[Coord { x: x1, y }] as usize;
                    }
                } else if y1 == y2 {
                    for x in min(x1, x2)..=max(x1, x2) {
                        next.score += self.city[Coord { x, y: y1 }] as usize;
                    }
                } else {
                    panic!("Invalid path");
                }
                // fix the double counting of the current location
                next.score -= self.city[walker.location] as usize;

                // if the score is better than previous, update the score
                // and continue walking, otherwise stop
                if let Some(score) = costs[next.location].score(next.direction, next.straight_counter) {
                    if next.score >= score {
                        continue;
                    }
                }
                // update the score
                costs[next.location].set_score(next.direction, next.straight_counter, next.score);
                // add the next walker to the queue
                queue.push(next);
            }
        }
        eprintln!("Iterations: {}", it);

        *costs[end].scores.iter().flatten().min().unwrap()
    }
}



fn parse_grid(s: &str) -> IResult<&str, Grid<u8>> {
    let gridline = terminated(digit1, alt((line_ending, eof)));
    let grid = fold_many1(gridline, || (Vec::new(), 0usize, 0usize), |(mut v, _x, y), line: &str| {
        //dimnension checks?
        let x = line.len();
        v.extend(line.chars().map(|c| c.to_digit(10).unwrap() as u8));
        (v, x, y + 1)
    });
    map(all_consuming(grid), |(elements, dim_x, dim_y)| Grid::new(elements, dim_x, dim_y))(s)
}


impl Problem for Solution {
    fn parse(&mut self, s: &str) -> Result<()> {
        let (_s, grid) = parse_grid(s).map_err(|e| anyhow!("parse error: {e}"))?;
        self.city = grid;
        eprintln!("city: {}", &self.city);
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        Ok(self.coolest_route().to_string())
    }

    fn part2(&self) -> Result<String> {
        Ok(self.coolest_route_heavy().to_string())
    }
}

