#[allow(unused_imports)]
use anyhow::{anyhow, Result};
use nom::{IResult,
branch::alt,
bytes::complete::{tag},
character::complete::{digit1, line_ending, space1},
combinator::{opt},
multi::{many1, separated_list1},
sequence::{delimited, terminated, separated_pair}
};

use crate::parsers::{positive_integer};
use crate::Problem;

#[derive(Default)]
pub struct Solution {
    data: Vec<Game>,
    limit_red: u32,
    limit_green: u32,
    limit_blue: u32,
}

#[derive(Default, Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Default, Debug)]
struct Game(Vec<Draw>);

impl Game {
    fn is_possible(&self, limit_red: u32, limit_green: u32, limit_blue: u32) -> bool {
        self.0.iter().all(|draw| draw.red <= limit_red && draw.green <= limit_green && draw.blue <= limit_blue)
    }

    fn power(&self) -> u32 {
        let (r, g, b) = self.0.iter()
            .fold((0,0,0), |acc, draw| {
                (acc.0.max(draw.red), acc.1.max(draw.green), acc.2.max(draw.blue))
        });

        r * g * b
    }
}


fn parse_draw(mut s: &str) -> IResult<&str, Draw> {
    let mut color = separated_pair(positive_integer, space1, alt((tag("red"), tag("green"), tag("blue"))));

    let mut draw = Draw::default();
    while let Ok((ss, (n, c))) = terminated(&mut color, opt(tag(", ")))(s) {
        match c {
            "red" => draw.red = n,
            "green" => draw.green = n,
            "blue" => draw.blue = n,
            _ => unreachable!(),
        }
        s = ss;
    }

    Ok((s, draw))
}

fn parse_line(s: &str) -> IResult<&str, Game> {
    // ignore num, assume ascending numbers. We can trust Eric? Right?
    let (s, _num) = delimited(tag("Game "), digit1, tag(": "))(s)?;

    let (s, draws) = separated_list1(tag("; "), parse_draw)(s)?;

    Ok((s, Game(draws)))
}


impl Problem for Solution {
    fn parse(&mut self, s: &str) -> Result<()> {
        let (_s, items) = many1(terminated(parse_line, line_ending))(s).unwrap();
        self.data = items;

        self.limit_red = 12;
        self.limit_green = 13;
        self.limit_blue = 14;

        Ok(())
    }

    fn part1(&self) -> Result<String> {
        Ok(self.data.iter()
           .enumerate()
           .filter(|(_, g)| g.is_possible(self.limit_red, self.limit_green, self.limit_blue))
           .map(|(id, _)| id + 1)
           .sum::<usize>()
           .to_string())
    }

   fn part2(&self) -> Result<String> {
        Ok(self.data.iter()
           .map(|game| game.power())
           .sum::<u32>()
           .to_string())
   }
}

