use std::str;

use anyhow::{anyhow, Result};

use crate::Problem;

use nom::{
    branch::alt,
    character::complete::{char, line_ending, one_of, space0},
    combinator::map,
    multi::{fold_many0, fold_many1, separated_list1},
    combinator::all_consuming,
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser
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

//
//both:
fn in_brackets<'a, O, F>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, nom::error::Error<&'a str>>
where F: Parser<&'a str, O, nom::error::Error<&'a str>>,
{
  delimited(char('('), inner, char(')'))
}

fn space_padded<'a, O, F>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, nom::error::Error<&'a str>>
where F: Parser<&'a str, O, nom::error::Error<&'a str>>,
{
  delimited(space0, inner, space0)
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
    alt((positive_integer, in_brackets(no_precedence)))(i)
}

fn no_precedence(i: &str) -> IResult<&str, u64> {
    let (i, v) = value(i)?;
    fold_many1(tuple((space_padded(operator), value)), v, |mut acc, (op, val)| {
        match op {
            Op::Add => acc += val,
            Op::Mul => acc *= val,
        }
        acc
    })(i)
}


//part 2
fn addition(i: &str) -> IResult<&str, u64> {
    map(separated_list1(
                space_padded(char('+')), 
                alt((positive_integer, in_brackets(plus_precedence)))
            ), |v| v.iter().sum())(i)
}

fn product(i: &str) -> IResult<&str, u64> {
    map(preceded(space_padded(char('*')),value2), |v| v)(i)
}

fn value2(i: &str) -> IResult<&str, u64> {
    alt((addition, in_brackets(plus_precedence), positive_integer))(i)
}

fn plus_precedence(i: &str) -> IResult<&str, u64> {
    let (i, v) = value2(i)?;
    fold_many0(product, v, |mut acc, val| {
        acc *= val;
        acc
    })(i)
}

// parse according to passed parser
fn parse<'a, F>(parser: F) -> impl FnMut(&'a str) -> IResult<&'a str, u64, nom::error::Error<&'a str>>
where F: Parser<&'a str, u64, nom::error::Error<&'a str>>,
{
    all_consuming(fold_many1(terminated(parser, line_ending), 0, |acc, val| {
        acc + val
    }))
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        self.input = String::from_utf8(i.to_vec())?;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let result = parse(no_precedence)(self.input.as_str()).map_err(|e| anyhow!(e.to_string()))?;
        Ok(format!("{}", result.1))
    }

    fn part2(&self) -> Result<String> {
        let result = parse(plus_precedence)(self.input.as_str()).map_err(|e| anyhow!(e.to_string()))?;
        Ok(format!("{}", result.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let s = no_precedence("1 + 2 * 3 + 4 * 5 + 6");
        println!("{:?}", s);
        assert_eq!(s.unwrap().1, 71);

        let s = no_precedence("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        println!("{:?}", s);
        assert_eq!(s.unwrap().1, 13632);

    }
    #[test]
    fn p2() {

        let s = plus_precedence("1 + 2 + 3");
        println!("{:?}", s);
        assert_eq!(s.unwrap().1, 6);

        let s = plus_precedence("1 + 2 * 3 + 4 * 5 + 6");
        println!("{:?}", s);
        assert_eq!(s.unwrap().1, 231);

        let s = value2("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        println!("{:?}", s);
        assert_eq!(s.unwrap().1, 1445);

        let s = plus_precedence("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        println!("{:?}", s);
        assert_eq!(s.unwrap().1, 23340);

    }
}
