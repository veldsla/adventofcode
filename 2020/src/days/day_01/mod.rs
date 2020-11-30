use anyhow::Result;

use crate::Problem;

#[derive(Default)]
pub struct Solution {
    input: Option<String>
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        self.input = Some(String::from_utf8(i.to_vec())?);
        Ok(())
    }

    fn part1(&self) -> Result<&str> {
        println!("Day 1, part1");
        Ok("Solved")
    }
}

