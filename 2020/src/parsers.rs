use std::ops::Range;
use nom::{
    branch::alt,
    character::complete::{char, digit1, line_ending},
    combinator::map,
    error::VerboseError,
    multi::{many0, many1},
    sequence::{preceded, separated_pair, terminated},
    IResult
};

//FIXME should use map_res? when i32 doesn't fit?
pub fn parse_i32(i: &str) -> IResult<&str, i32> {
  alt((
    map(digit1, |digit_str: &str| {
      digit_str.parse::<i32>().unwrap()
    }),
    map(preceded(char('-'), digit1), |digit_str: &str| {
      -1 * digit_str.parse::<i32>().unwrap()
    }),
  ))(i)
}

pub fn int_range_inclusive(i: &str) -> IResult<&str, Range<i32>> {
    map(separated_pair(parse_i32, char('-'), parse_i32), |(start, stop)| start..stop+1)(i)
}

pub fn numbers_on_lines(i: &str) -> IResult<&str, Vec<i32>> {
    many1(terminated(parse_i32, many0(line_ending)))(i)
    //separated_list1(line_ending, parse_i32)(i)
}
