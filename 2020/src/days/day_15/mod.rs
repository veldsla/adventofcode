use std::str;

use anyhow::{anyhow, Result};

use crate::Problem;
use crate::parsers::positive_integer;

use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    multi::separated_list1,
    combinator::all_consuming,
    sequence::terminated,
    IResult
};

#[derive(Default)]
pub struct Solution {
    input: Vec<usize>
}

struct NumbersIterator {
    spoken: Vec<Option<usize>>,
    last: usize,
    turn: usize,
}

impl NumbersIterator {
    fn new(v: &[usize]) -> NumbersIterator {
        let mut spoken = vec![None; *v.iter().max().unwrap() + 1];
        for (i, &s) in v[0..v.len()-1].iter().enumerate() {
            spoken[s] = Some(i);
        }
        //let spoken = v[0..v.len()-1].iter().enumerate().map(|(i, &v)| (v, i)).collect();
        NumbersIterator { spoken, last: *v.last().unwrap(), turn: v.len()-2 }
    }
}

impl Iterator for NumbersIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.turn += 1;
        let next = if let Some(Some(turn)) = self.spoken.get(self.last) {
            self.turn - turn
        } else {
            0
        };
        if self.last > self.spoken.len() {
            self.spoken.extend(std::iter::repeat(None).take(self.last + 1 - self.spoken.len()));
        }
        self.spoken[self.last] = Some(self.turn);
        self.last = next;
        Some(self.last)
    }
}

fn parse(i: &str) -> IResult<&str,Vec<usize>> {
    all_consuming(terminated(separated_list1(tag(","), positive_integer), line_ending))(i)
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(str::from_utf8(i)?).map_err(|e| anyhow!(e.to_string()))?;
        self.input = result.1;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        Ok(format!("{}",NumbersIterator::new(&self.input).nth(2019-self.input.len()).unwrap()))
    }

    fn part2(&self) -> Result<String> {
        Ok(format!("{}", NumbersIterator::new(&self.input).nth(29999999-self.input.len()).unwrap()))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let start = [0,3,6];
        let mut it = NumbersIterator::new(&start);
        assert_eq!(it.next(), Some(0));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(0));
        assert_eq!(it.next(), Some(4));
        assert_eq!(it.next(), Some(0));

        assert_eq!(it.nth(2009), Some(436));

        assert_eq!(NumbersIterator::new(&[3,1,2]).nth(2016), Some(1836))
    }
}
