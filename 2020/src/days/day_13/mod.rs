use std::str;

use anyhow::{anyhow, Result};

use crate::Problem;
use crate::parsers::positive_integer;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    multi::separated_list1,
    combinator::{all_consuming, map},
    sequence::{tuple, terminated},
    IResult
};

#[derive(Default)]
pub struct Solution {
    start: u32,
    list: Vec<Option<u32>>
}

fn parse(i: &str) -> IResult<&str, (u32, Vec<Option<u32>>)> {
    let time = map(alt((tag("x"), digit1)), |s| {
        match s {
            "x" => None,
            s => Some(s.parse::<u32>().unwrap())
        }
    });
    let times = separated_list1(tag(","), time);
    all_consuming(tuple((terminated(positive_integer, line_ending), terminated(times, line_ending))))(i)
}

fn calc_u(v: i64, div: i64) -> i64 {
    (1..).find(|m| v*m % div == 1).unwrap()
}

fn solve_rem(v: &[(i64, i64)]) -> i64 {
    let n: i64 = v.iter().map(|e| e.1).product();
    let u: i64 = v.iter()
        .filter(|&&(id, _)| id != 0)
        .map(|&(id, period)| {
            let x = n / period;
            let rem = period-id;
            let u = calc_u(x, period);
            rem * x * u
        }).sum();
    u % n
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(str::from_utf8(i)?).map_err(|e| anyhow!(e.to_string()))?;
        self.start = result.1.0;
        self.list = result.1.1;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let (id, wait) = self.list.iter()
            .filter_map(|&v| v)
            .map(|v| {
                match self.start % v {
                    0 => (v, 0),
                    rem => (v, v - rem)
                }
            }).min_by_key(|t| t.1).unwrap();
        
        Ok(format!("{}", id * wait))
    }

    fn part2(&self) -> Result<String> {

        let idtime: Vec<_> =   self.list.iter()
            .enumerate()
            .filter(|(_, v)| v.is_some())
            .map(|(i, v)| (i as i64, v.unwrap() as i64))
            .collect();
        //all line ids are prime. So it seems I finally have to look into the chinese
        //remainder theorem. Forgive me my implementation, total noob here.
        Ok(format!("{}", solve_rem(&idtime)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "939\n7,13,x,x,59,x,31,19\n";

    #[test]
    fn p1() {
        let result = parse(TEST);
        assert!(result.is_ok());
        let data = result.unwrap().1;
        assert_eq!(data.0, 939);
        assert_eq!(data.1.len(), 8)
    }

    #[test]
    fn p2() {
        let data = parse("0\n17,x,13,19\n").unwrap().1;
        let idtime: Vec<_> =   data.1.iter()
            .enumerate()
            .filter(|(_, v)| v.is_some())
            .map(|(i, v)| (i as i64, v.unwrap() as i64))
            .collect();
        println!("{:?}", idtime);
        assert_eq!(solve_rem(&idtime), 3417)
    }
}
