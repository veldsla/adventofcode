use std::str;

use anyhow::{anyhow, Result};
use nom::{
    character::complete::line_ending,
    combinator::all_consuming,
    multi::many1,
    sequence::terminated,
};

use crate::Problem;
use crate::parsers;

#[derive(Default)]
pub struct Solution {
    input: Vec<u8>
}

fn diff_1_3(v: &[u8]) -> (usize, usize) {
    let mut s: Vec<_> = v.iter().copied().collect();
    s.sort_unstable();

    s.windows(2)
        .map(|v| v[1] - v[0])
        .fold((1,1), |(mut d1, mut d3), d| {
            match d {
                1 => d1 +=1,
                3 => d3 +=1,
                _ => {}
            }
            (d1, d3)
        })
}

fn count_valid(v: &[u8]) -> usize {
    let mut s: Vec<_> = v.iter().copied().collect();
    s.sort_unstable_by(|a, b| b.cmp(a));
    s.push(0);

    let mut from_jolt = vec![0; s.len()];

    //only one possibility from max value
    from_jolt[0] = 1;

    for (pos, current) in s.iter().enumerate() {
        //get compatible with current
        s.iter()
            .enumerate()
            .skip(pos+1)
            .take_while(|(_, &v)| current - v <= 3)
            .for_each(|(p, _)| from_jolt[p] += from_jolt[pos]);
    }

    *from_jolt.last().unwrap()
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = all_consuming(many1(terminated(parsers::positive_integer, line_ending)))
            (str::from_utf8(i)?).map_err(|e| anyhow!(e.to_string()))?;

        self.input = result.1;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let (d1, d3) = diff_1_3(&self.input);
        Ok(format!("{}", d1 * d3))
    }

    fn part2(&self) -> Result<String> {
        Ok(format!("{}", count_valid(&self.input)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn p1() {
        let v = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 
            49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 
            9, 4, 2, 34, 10, 3];
        assert_eq!(diff_1_3(&v), (22,10));
    }

    #[test]
    fn p2() {
        let v = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12,4];
        assert_eq!(count_valid(&v),8);
        let v = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 
            49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 
            9, 4, 2, 34, 10, 3];
        assert_eq!(count_valid(&v),19208);
    }
}
