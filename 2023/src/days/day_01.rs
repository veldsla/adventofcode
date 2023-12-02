#[allow(unused_imports)]
use anyhow::{anyhow, Result};

use nom::{IResult,
branch::alt,
bytes::complete::{tag, take_while1},
character::complete::{alpha1, anychar, digit1, line_ending},
character::is_alphabetic,
combinator::{all_consuming, map, peek},
multi::many1,
sequence::terminated,
};

use crate::parsers::{single_dec_digit, single_alpha};
use crate::Problem;

#[derive(Default)]
pub struct Solution{
    data: Vec<CalibData>
}

struct CalibData(Vec<Item>);

impl CalibData {
    fn calib_value_digits(&self) -> u32 {
        let mut it = self.0.iter().filter(|e| matches!(e, Item::Digit(_n)));
        let first = it.next().expect("No number in line");
        let last = it.last().unwrap_or(first);

        first.val() * 10 + last.val()
    }

    fn calib_value_both(&self) -> u32 {
        let mut it = self.0.iter();
        let first = it.next().expect("No number in line");
        let last = it.last().unwrap_or(first);

        first.val() * 10 + last.val()
    }
}

enum Item {
    Word(u32),
    Digit(u32),
}

impl Item {
    fn val(&self) -> u32 {
        match self {
            Item::Digit(n)  | Item::Word(n) => *n,
        }
    }
}

fn named_digit(s: &str) -> IResult<&str, u32> {
    let one = map(tag("one"), |_s| 1u32);
    let two = map(tag("two"), |_s| 2u32);
    let three = map(tag("three"), |_s| 3u32);
    let four = map(tag("four"), |_s| 4u32);
    let five = map(tag("five"), |_s| 5u32);
    let six = map(tag("six"), |_s| 6u32);
    let seven = map(tag("seven"), |_s| 7u32);
    let eight = map(tag("eight"), |_s| 8u32);
    let nine = map(tag("nine"), |_s| 9u32);

     match peek(alt((one, two, three, four, five, six, seven, eight, nine)))(s) {
         Ok((s, v)) => {
             Ok((&s[1..], v))
         },
         Err(e) => Err(e)
     }
}

fn parse_line(mut s: &str) -> IResult<&str, CalibData> {
    let mut digits = Vec::new();
    while !s. is_empty() && peek(line_ending::<&str, nom::error::Error<&str>>)(s).is_err() {
        if let Ok((_, d)) = named_digit(s) {
            digits.push(Item::Word(d));
        } else if let Ok((_, d)) = single_dec_digit(s) {
            digits.push(Item::Digit(d));
        }
        s = &s[1..];
    }

    Ok((s, CalibData(digits)))
}

impl Problem for Solution {
    fn parse(&mut self, s: &str) -> Result<()> {
        let (_s, items) = many1(terminated(parse_line, line_ending))(s).unwrap();
        self.data = items;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
       Ok(self.data.iter()
           .map(|e| e.calib_value_digits())
           .sum::<u32>()
           .to_string())
    }

   fn part2(&self) -> Result<String> {
       Ok(self.data.iter()
           .map(|e| e.calib_value_both())
           .sum::<u32>()
           .to_string())
   }
}

