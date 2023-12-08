use std::collections::{HashSet, HashMap};
use anyhow::{anyhow, Result};

use nom::{
    bytes::complete::{tag},
    character::complete::{anychar, alpha1, line_ending, one_of},
    multi::fold_many1,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult
};

use crate::Problem;

#[derive(Default)]
pub struct Solution {
    directions: Vec<Direction>,
    map: HashMap<String, (String, String)>,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Solution {
    fn to_endz<'a>(&'a self, mut from: &'a (String, String)) -> usize {
        let mut count = 0;

        for d in self.directions.iter().cycle() {
            from = match d {
                Direction::Left => {
                    if from.0.ends_with("Z") {
                        break;
                    } else {
                        self.map.get(&from.0)
                    }
                },
                Direction::Right => {
                    if from.1.ends_with("Z") {
                        break;
                    } else {
                        self.map.get(&from.1)
                    }
                },
            }.expect("map not found");
            count += 1;
        }

        count += 1;
        count
    }
}

fn lcmold(n: &[usize]) -> usize {
    // very naive cause I'm lazy
    // left in the code because it's so terrible (takes 3 seconds)
    let max = n.iter().max().unwrap();
    let mut m = 2usize;

    loop {
        if n.iter().all(|n| (max * m) % n == 0) {
            break m*max
        }
        m += 1;
    }
}

fn lcm(n: &[usize]) -> usize {
    let s = n[0];
    n.iter().skip(1).fold(s, |acc, &v| num::integer::lcm(acc, v))
}

fn parse_directions(s: &str) -> IResult<&str, Vec<Direction>> {
    terminated(
        fold_many1(one_of("RL"), Vec::new, |mut acc, c| {
            let d = match c {
                'R' => Direction::Right,
                'L' => Direction::Left,
                _ => unreachable!(),
            };
            acc.push(d);
            acc
        }),
        line_ending)(s)
}

fn parse_mapline(s: &str) -> IResult<&str, (String, (String, String))> {
    let (s, name) = alpha1(s)?;

    let (s, (from, to)) = preceded(tag(" = "), delimited(tag("("), separated_pair(alpha1, tag(", "), alpha1), tag(")")))(s)?;

    Ok((s, (name.to_string(), (from.to_string(), to.to_string()))))

}



impl Problem for Solution {
    fn parse(&mut self, s: &str) -> Result<()> {
        let (s, directions) = terminated(parse_directions, line_ending)(s).map_err(|e| anyhow!("parse error: {e}"))?;
        let (_s, map) = fold_many1(
            terminated(parse_mapline, line_ending),
            HashMap::new,
            |mut acc, (id, lr)| {
                acc.insert(id, lr);
                acc
            })(s).map_err(|e| anyhow!("parse error: {e}"))?;

        self.directions = directions;
        self.map = map;


        Ok(())

    }

    fn part1(&self) -> Result<String> {
        let mut map = self.map.get("AAA").unwrap();

        let mut count = 0;
        for d in self.directions.iter().cycle() {
            map = match d {
                Direction::Left => {
                    if map.0 == "ZZZ" {
                        break;
                    } else {
                        self.map.get(&map.0)
                    }
                },
                Direction::Right => {
                    if map.1 == "ZZZ" {
                        break;
                    } else {
                        self.map.get(&map.1)
                    }
                },
            }.expect("map not found");
            count += 1;
        }
        count += 1;

        Ok(count.to_string())
    }

    fn part2(&self) -> Result<String> {

        // for every start calc the length to the end
        let l: Vec<usize> = self.map.iter().filter_map(|(k, v)| {
            if k.ends_with("A") {
                Some(v)
            } else {
                None
            }
        }).map(|start| self.to_endz(start)).collect();

        // calculate the first common multiple of all those numbers
        let count = lcm(&l);

        Ok(count.to_string())
    }
}

