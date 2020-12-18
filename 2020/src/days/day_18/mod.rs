use std::str;

use anyhow::{anyhow, Result};

use crate::Problem;

use nom::{
    branch::alt,
    bytes::complete::is_a,
    character::complete::{char, line_ending, one_of, space0},
    combinator::{map, iterator},
    multi::{fold_many1, separated_list1},
    combinator::all_consuming,
    sequence::{delimited, preceded, terminated, tuple},
    IResult
};
use crate::parsers::positive_integer;

#[derive(Default)]
pub struct Solution {
    input: String
}


enum Op {
    Mul,
    Add,
}

//part one
fn operator(i: &str) -> IResult<&str, Op> {
    map(one_of("+*"), |c| {
        match c {
            '+' => Op::Add,
            '*' => Op::Mul,
            _ => unreachable!()
        }
    })(i)
}

fn value(i: &str) -> IResult<&str, u64> {
    alt((positive_integer, nested_expression))(i)
}

fn nested_expression(i: &str) -> IResult<&str, u64> {
    delimited(char('('), expression, char(')'))(i)
}

fn expression(i: &str) -> IResult<&str, u64> {
    let (i, v) = value(i)?;
    let mut it = iterator(i, tuple((terminated(preceded(space0, operator), space0), value)));
    let result = it.fold(v, |mut acc, (op, val)| {
        match op {
            Op::Add => acc += val,
            Op::Mul => acc *= val,
        }
        acc
    });
    let res = it.finish()?;
    Ok((res.0, result))
}

fn parse(i: &str) -> IResult<&str, u64> {
    fold_many1(terminated(expression, line_ending), 0, |acc, val| {
        acc + val
    })(i)
}

//part 2
fn addition(i: &str) -> IResult<&str, u64> {
    map(separated_list1(terminated(preceded(space0, char('+')), space0), alt((positive_integer, nested_expression2))), |v| v.iter().sum())(i)
    //fold_many1(preceded(terminated(preceded(space0, char('+')), space0), value), 0, |acc, val| acc + val)(i)
}

fn product(i: &str) -> IResult<&str, u64> {
    map(preceded(terminated(preceded(space0, char('*')), space0),value2), |v| v)(i)
}

fn value2(i: &str) -> IResult<&str, u64> {
    alt((addition, nested_expression2, positive_integer))(i)
}

fn nested_expression2(i: &str) -> IResult<&str, u64> {
    delimited(char('('), expression2, char(')'))(i)
}

fn expression2(i: &str) -> IResult<&str, u64> {
    let (i, v) = value2(i)?;
    let mut it = iterator(i, product);
    let result = it.fold(v, |mut acc, val| {
        acc *= val;
        acc
    });
    
    let res = it.finish()?;
    Ok((res.0, result))
}

fn parse2(i: &str) -> IResult<&str, u64> {
    fold_many1(terminated(expression2, line_ending), 0, |acc, val| {
        acc + val
    })(i)
}


impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        self.input = String::from_utf8(i.to_vec())?;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let result = parse(self.input.as_str()).map_err(|e| anyhow!(e.to_string()))?;
        Ok(format!("{}", result.1))
    }

    fn part2(&self) -> Result<String> {
        let result = parse2(self.input.as_str()).map_err(|e| anyhow!(e.to_string()))?;
        Ok(format!("{}", result.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let s = expression("1 + 2 * 3 + 4 * 5 + 6");
        println!("{:?}", s);
        assert_eq!(s.unwrap().1, 71);

        let s = expression("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        println!("{:?}", s);
        assert_eq!(s.unwrap().1, 13632);

    }
    #[test]
    fn p2() {

        let s = expression2("1 + 2 + 3");
        println!("{:?}", s);
        assert_eq!(s.unwrap().1, 6);

        let s = expression2("1 + 2 * 3 + 4 * 5 + 6");
        println!("{:?}", s);
        assert_eq!(s.unwrap().1, 231);

        let s = value2("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        println!("{:?}", s);
        assert_eq!(s.unwrap().1, 1445);

        let s = expression2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        println!("{:?}", s);
        assert_eq!(s.unwrap().1, 23340);

    }
}
