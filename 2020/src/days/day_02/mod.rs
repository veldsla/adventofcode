use std::ops::Range;
use std::str;

use anyhow::{anyhow, Result};

use crate::Problem;

#[derive(Default)]
pub struct Solution {
    input: Option<Data>
}

struct Data(Vec<PwEntry>);
struct PwEntry {
    password: String,
    policy: Policy,
}
struct Policy {
    character: char,
    times: Range<usize>,
}

mod parse {
    use super::*;
    use nom::{
        character::complete::{anychar, char, not_line_ending, space1, line_ending},
        sequence::separated_pair,
        combinator::{all_consuming, map},
        multi::{many0, many1},
        sequence::{terminated},
        IResult
    };
    //my helpers
    use crate::parsers::*;

    //custom for problem
    //FIXME better types for number parsing. Macros? Generics?
    pub(super) fn policy(i: &str) -> IResult<&str, Policy> {
        let pol = separated_pair(int_range_inclusive, space1, anychar);
        map(terminated(pol, char(':')), |(times, character)| Policy { character, times: times.start as usize..times.end as usize })(i)
    }

    pub(super) fn pw_entry(i: &str) -> IResult<&str, PwEntry> {
        map(separated_pair(policy, space1, not_line_ending), |(policy, p)| {
            PwEntry { policy, password: p.to_owned() }
        })(i)
    }

    pub(super) fn pw_data(i: &str) -> IResult<&str, Vec<PwEntry>> {
        all_consuming(many1(terminated(pw_entry, many0(line_ending))))(i)
    }
}

impl Data {
    fn count_valid(&self) -> usize {
        self.0.iter().filter(|e| e.is_valid()).count()
    }

    fn count_valid_otcas(&self) -> usize {
        self.0.iter().filter(|e| e.is_valid_otcas()).count()
    }
}

impl PwEntry {
    fn is_valid(&self) -> bool {
        let count = self.password.chars()
            .filter(|&c| c == self.policy.character)
            .count();
        self.policy.times.contains(&count)
    }

    fn is_valid_otcas(&self) -> bool {
        self.password.char_indices()
            .filter(|&(i, c)| {
                c == self.policy.character &&
                (i+1 == self.policy.times.start || i+2 == self.policy.times.end)
            })
            .count() == 1
    }
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse::pw_data(str::from_utf8(i)?)
            .map_err(|e| anyhow!(e.to_string()))?;

        self.input = Some(Data(result.1));
        Ok(())
    }
    fn part1(&self) -> Result<String> {
        let data = self.input.as_ref().ok_or(anyhow!("not parsed"))?;
        Ok(format!("{}", data.count_valid()))
    }

    fn part2(&self) -> Result<String> {
        let data = self.input.as_ref().ok_or(anyhow!("not parsed"))?;
        Ok(format!("{}", data.count_valid_otcas()))
    }
}

#[cfg(test)]
mod tests {
}
