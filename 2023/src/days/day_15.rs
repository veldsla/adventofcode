use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, is_not},
    character::complete::{alpha1, alphanumeric1, anychar, char, digit1, line_ending},
    multi::{many1, separated_list1},
    combinator::{all_consuming, eof, map},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult
};


use crate::Problem;

#[derive(Default)]
pub struct Solution {
    data: Vec<String>
}

type Label = usize;
#[derive(Debug)]
enum Command {
    Remove(Label),
    Add(Label, u8),
}


fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| {
        debug_assert!(c.is_ascii());
        let v = c as usize;
        ((acc + v) * 17) % 256
    })
}

fn parse_line(s: &str) -> IResult<&str, Vec<String>> {
   separated_list1(tag(","), 
       map(is_not(",\r\n"), |s: &str| s.to_owned()))(s)
}

fn parse_instruction(s: &str) -> IResult<&str, Command> {
    map(tuple((alpha1, alt((tag("-"), preceded(tag("="), digit1))) )), |(label, command)| {
        let label = hash(label);
        match command {
            "-" => Command::Remove(label),
            d => { eprintln!("d = '{d}'"); Command::Add(label, d.parse().unwrap())}
        }

    })(s)
}

impl Problem for Solution {
    fn parse(&mut self, s: &str) -> Result<()> {
        let (_s,  data) = terminated(parse_line, line_ending)(s)
            .map_err(|e| anyhow!("parse error: {e}"))?;

        self.data = data;
        dbg!(&self.data);
        Ok(())

    }

    fn part1(&self) -> Result<String> {
        Ok(self.data.iter().map(|s| hash(s)).sum::<usize>().to_string())
    }

   fn part2(&self) -> Result<String> {
       let commands: Vec<Command>  = self.data.iter()
           .inspect(|s| eprintln!("'{s}'"))
           .map(|s| parse_instruction(s.as_str()).unwrap().1)
           .collect();

       dbg!(commands);
       Ok(0.to_string())
   }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }
}

