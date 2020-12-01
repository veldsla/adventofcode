use nom::{
    branch::alt,
    character::complete::{char, digit1, line_ending},
    combinator::map,
    error::VerboseError,
    multi::{many0, many1},
    sequence::{preceded, terminated},
    IResult
};

//FIXME should use map_res? when i32 doesn't fit?
fn parse_i32<'a>(i: &'a str) -> IResult<&'a str, i32, VerboseError<&'a str>> {
  alt((
    map(digit1, |digit_str: &str| {
      digit_str.parse::<i32>().unwrap()
    }),
    map(preceded(char('-'), digit1), |digit_str: &str| {
      -1 * digit_str.parse::<i32>().unwrap()
    }),
  ))(i)
}

pub fn numbers_on_lines(i: &str) -> IResult<&str, Vec<i32>, VerboseError<&str>> {
    many1(terminated(parse_i32, many0(line_ending)))(i)
    //separated_list1(line_ending, parse_i32)(i)
}

