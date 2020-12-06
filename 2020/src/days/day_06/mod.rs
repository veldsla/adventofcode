use anyhow::{anyhow, Result};

use crate::Problem;

#[derive(Default)]
pub struct Solution {
    groups: Vec<(Vec<u8>, usize)>,
}

use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::is_alphabetic,
    character::complete::line_ending,
    multi::{many1, fold_many1},
    combinator::{all_consuming, eof},
    sequence::terminated,
    IResult
};
fn parse(i: &[u8]) -> IResult<&[u8], Vec<(Vec<u8>, usize)>> {
    let person =  terminated(take_while1(is_alphabetic), line_ending);
    let group = fold_many1(person , (Vec::new(), 0), |(mut v, n), p| {
        v.extend(p);
        (v, n+1)
    });
    all_consuming(many1(terminated(group, alt((line_ending, eof)))))(i)
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(i).map_err(|e| anyhow!(e.to_string()))?;
        self.groups = result.1;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        //optimized version assumes no duplicate answers by persons
        let mut n_yes = 0;
        for g in &self.groups {
            let mut letters = [0; 26];
            for &q in &g.0 {
                letters[q as usize - 97 ] += 1;
            }
            n_yes += letters.iter().filter(|&&l| l > 0).count();
        }
        Ok(format!("{}", n_yes))
    }

    fn part2(&self) -> Result<String> {
        //optimized version assumes no duplicate answers by persons
        let mut n_yes = 0;
        for g in &self.groups {
            let mut letters = [0; 26];
            for &q in &g.0 {
                letters[q as usize - 97 ] += 1;
            }
            n_yes += letters.iter().filter(|&&l| l == g.1).count();
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
        println!("{:?}", p);
        assert!(p.is_ok());
    }
}

