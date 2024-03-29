use std::fmt::Debug;
use std::str::FromStr;
use std::ops::Range;

#[allow(unused)]
use nom::{
    branch::alt,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res, opt},
    error::ParseError,
    multi::{many0, many1, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    AsChar, IResult, InputIter, InputLength, Slice,
    lib::std::ops::RangeFrom
};

//FIXME should use map_res? when i32 doesn't fit?
pub fn parse_i32(i: &str) -> IResult<&str, i32> {
  alt((
    map(digit1, |digit_str: &str| {
      digit_str.parse::<i32>().unwrap()
    }),
    map(preceded(char('-'), digit1), |digit_str: &str| {
      -digit_str.parse::<i32>().unwrap()
    }),
  ))(i)
}

pub fn positive_integer<N: FromStr>(i: &str) -> IResult<&str, N> {
    map_res(preceded(opt(char('+')), digit1), |number: &str| number.parse::<N>())(i)
}

fn negative<N: std::ops::Neg + std::ops::Neg<Output = N>>(n: N) -> N { n.neg() }
pub fn negative_integer<N: Debug + FromStr + std::ops::Neg + std::ops::Neg<Output = N>>(i: &str) -> IResult<&str, N> {
    map(preceded(char('-'), positive_integer), negative)(i)
}

//Debug required for error?
pub fn signed_integer<N: Debug + FromStr + std::ops::Neg + std::ops::Neg<Output = N>>(i: &str) -> IResult<&str, N> {
    alt((positive_integer, negative_integer))(i)
}

pub fn range_positive_integer<N: FromStr>(i: &str) -> IResult<&str, std::ops::RangeInclusive<N>> {
    map(separated_pair(positive_integer, char('-'), positive_integer), |(b, e)| b..=e)(i)
}

pub fn commasep_positive_integer<N: FromStr>(i: &str) -> IResult<&str, Vec<N>> {
    separated_list1(char(','), positive_integer)(i)
}

pub fn int_range_inclusive(i: &str) -> IResult<&str, Range<i32>> {
    map(separated_pair(parse_i32, char('-'), parse_i32), |(start, stop)| start..stop+1)(i)
}

pub fn numbers_on_lines(i: &str) -> IResult<&str, Vec<i32>> {
    many1(terminated(parse_i32, many0(line_ending)))(i)
    //separated_list1(line_ending, parse_i32)(i)
}

pub fn single_dec_digit(i: &str) -> IResult<&str, u32> {
    if let Some(c) = i.chars().nth(0) {
        if c.is_digit(10) {
            return Ok((&i[1..], c.to_digit(10).unwrap()))
        }
    }
    Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Digit)))
}

pub fn single_alpha(i: &str) -> IResult<&str, char> {
    if let Some(c) = i.chars().nth(0) {
        if c.is_alpha() {
            return Ok((&i[1..], c))
        }
    }
    Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Digit)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        assert_eq!(positive_integer::<u32>("123"), Ok(("", 123)));
        //is allowed but will never be negatve
        assert_eq!(positive_integer::<i32>("123"), Ok(("", 123)));
        //negative fails to parse
        assert!(positive_integer::<i32>("-123").is_err());
        // overflow fails
        assert!(positive_integer::<u8>("300").is_err());

        assert_eq!(negative_integer::<i32>("-123"), Ok(("", -123)));
        //positive numbers fail
        assert!(negative_integer::<i32>("123").is_err());
        // overflow fails
        assert!(negative_integer::<i8>("-300").is_err());

        //signed integers can be parsed into any signed type
        assert_eq!(signed_integer::<i32>("123"), Ok(("", 123)));
        assert_eq!(signed_integer::<i32>("-123"), Ok(("", -123)));
        assert_eq!(signed_integer::<i32>("+123"), Ok(("", 123)));
        assert!(signed_integer::<i8>("-300").is_err());
        assert!(signed_integer::<i8>("300").is_err());

    }
}
