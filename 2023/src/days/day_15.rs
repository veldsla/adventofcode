use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, is_not},
    character::complete::{alpha1, digit1, line_ending},
    multi::{separated_list1},
    combinator::{map},
    sequence::{preceded, terminated, tuple},
    IResult
};


use crate::Problem;

#[derive(Default)]
pub struct Solution {
    data: Vec<String>
}

type Label = String;
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
        //let label = hash(label);
        match command {
            "-" => Command::Remove(label.to_owned()),
            d => Command::Add(label.to_owned(), d.parse::<u8>().unwrap()),
        }

    })(s)
}

impl Problem for Solution {
    fn parse(&mut self, s: &str) -> Result<()> {
        let (_s,  data) = terminated(parse_line, line_ending)(s)
            .map_err(|e| anyhow!("parse error: {e}"))?;

        self.data = data;
        Ok(())

    }

    fn part1(&self) -> Result<String> {
        Ok(self.data.iter().map(|s| hash(s)).sum::<usize>().to_string())
    }

   fn part2(&self) -> Result<String> {
       let commands: Vec<Command>  = self.data.iter()
           .map(|s| parse_instruction(s.as_str()).unwrap().1)
           .collect();

       let mut boxes: Vec<Vec<(&str, u8)>> = (0..256).map(|_| Vec::new()).collect();

       for command in &commands {
           match command {
               Command::Remove(label) => {
                   let idx = hash(label);
                   if let Some(pos) = boxes[idx].iter().position(|(l, _)| l == label) {
                       boxes[idx].remove(pos);
                   }
               },
               Command::Add(label, f) => {
                   let idx = hash(label);
                   if let Some(pos) = boxes[idx].iter().position(|(l, _)| l == label) {
                       boxes[idx][pos] = (label, *f);
                   } else {
                       boxes[idx].push((label, *f));
                   }
               },
           }
       }

       let sum = boxes.into_iter()
           .enumerate()
           .map(|(i, v)|  {
               let bn = i + 1;
               v.into_iter().enumerate()
                   .map(|(i, (_, f))| bn as u64 * (i+1) as u64 * f as u64)
                   .sum::<u64>()
           }).sum::<u64>();

       Ok(sum.to_string())
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

