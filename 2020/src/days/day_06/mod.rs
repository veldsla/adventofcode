use std::collections::{HashSet, HashMap};
use anyhow::{anyhow, Result};

use crate::Problem;

#[derive(Default)]
pub struct Solution {
    groups: Vec<Vec<Vec<u8>>>
}

use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::is_alphabetic,
    character::complete::line_ending,
    multi::many1,
    combinator::{all_consuming, eof, map},
    sequence::terminated,
    IResult
};
fn parse(i: &[u8]) -> IResult<&[u8], Vec<Vec<Vec<u8>>>> {
    let person =  map(terminated(take_while1(is_alphabetic), line_ending), |s: &[u8]| s.to_vec());
    let group = terminated(many1(person), alt((line_ending, eof)));
    all_consuming(many1(group))(i)
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(i).map_err(|e| anyhow!(e.to_string()))?;
        self.groups = result.1;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let n_yes:usize = self.groups
            .iter()
            .map(|g| {
                g.iter().flat_map(|p| p)
                    .collect::<HashSet<_>>()
                    .len()
            }).sum();

        Ok(format!("{}", n_yes))
    }

    fn part2(&self) -> Result<String> {
        let mut counter = HashMap::new();
        let mut n_yes = 0;
        for g in &self.groups {
            counter.clear();
            for p in g {
                for q in p {
                    let c = counter.entry(q).or_insert(0);
                    *c += 1;
                }
            }
            n_yes += counter.values().filter(|&&v| v == g.len()).count();
        }

        Ok(format!("{}", n_yes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
const TEST: &[u8] = b"abc

a
b
c

ab
ac

a
a
a
a

b
";
    #[test]
    fn p1() {
        let p = parse(TEST);
        assert!(p.is_ok());
    }
}

