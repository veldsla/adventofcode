use std::str;

use anyhow::{anyhow, Result};
use nom::combinator::all_consuming;

use crate::Problem;
use crate::parsers;

#[derive(Default)]
pub struct Solution {
    input: Option<Data>
}

struct Data(Vec<i32>);

impl Data {
    fn new(i: &[u8]) -> Result<Data>{
        let result = all_consuming(parsers::numbers_on_lines)(str::from_utf8(i)?)
            .map_err(|e| anyhow!(format!("Parse error: {}:", e)))?;
        let mut numbers = result.1;
        numbers.sort();
        Ok(Data(numbers))
    }

    fn prod_of_sum2(&self, sum: i32) -> Result<i32> {
        self.0.iter().enumerate()
            .filter_map(|(i, &n1)| {
                let remain = sum - n1;
                if let Ok(_) = self.0[i+1..].binary_search(&remain) {
                    Some(remain*n1)
                } else {
                    None
                }
            }).next().ok_or_else(|| anyhow!("No solution"))
    }

    fn prod_of_sum3(&self, sum: i32) -> Result<i32> {
        for (i1, &n1) in self.0.iter().enumerate() {
            for (i2, &n2) in self.0.iter().enumerate().skip(i1) {
                let wanted = sum - n1 - n2;
                if wanted < n2 { break }
                if let Ok(_) = self.0[i2+1..].binary_search(&wanted) {
                    return Ok(wanted * n1 * n2)
                }
            }
        }

        Err(anyhow!("No solution"))
    }
}



impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {

        self.input = Some(Data::new(i)?);
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let data = self.input.as_ref().unwrap();
        Ok(format!("{}", data.prod_of_sum2(2020)?))
    }

    fn part2(&self) -> Result<String> {
        let data = self.input.as_ref().unwrap();
        Ok(format!("{}", data.prod_of_sum3(2020)?))
    }
}

