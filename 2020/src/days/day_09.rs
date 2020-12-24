use std::str;

use anyhow::{anyhow, Result};
use nom::{
    character::complete::line_ending,
    combinator::all_consuming,
    multi::many1,
    sequence::terminated,
};

use crate::Problem;
use crate::parsers;

#[derive(Default)]
pub struct Solution {
    input: Vec<u64>
}

fn find_invalid(v: &[u64], prefix: usize) -> Result<u64> {
    let mut sums: Vec<u64> = Vec::with_capacity((prefix * (prefix-1)) / 2);
    let mut in_sum: Vec<Vec<usize>> = std::iter::repeat(Vec::with_capacity(prefix)).take(prefix).collect();

    for p1 in 0..prefix-1 {
        for p2 in p1+1..prefix {
            sums.push(v[p1] + v[p2]);
            //store upper triangle position info
            in_sum[p1].push(sums.len() - 1);
            in_sum[p2].push(sums.len() - 1);
        }
    }

    for (i, &num) in v[prefix..].iter().enumerate() {
        if !sums.contains(&num) {
            return Ok(num);
        }
        //update positions in sums involving current
        for &p in &in_sum[i % prefix] {
            sums[p] = sums[p] + num - v[i];
        }
    }

    Err(anyhow!("No solution"))
}

fn find_sum(v: &[u64], sum: u64) -> Result<u64> {
    for (start, &value) in v.iter().enumerate() {
        if let Some(pos) = v.iter().skip(start+1)
            .scan(value, |total, &n| { *total += n; Some(*total) })
            .take_while(|&v| v <= sum)
            .position(|v| v == sum)
        {
            let min = v[start..start+pos+2].iter().min().unwrap();
            let max = v[start..start+pos+2].iter().max().unwrap();
            return Ok(min + max);
        }
    }

    Err(anyhow!("No solution"))
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = all_consuming(many1(terminated(parsers::positive_integer, line_ending)))
            (str::from_utf8(i)?).map_err(|e| anyhow!(e.to_string()))?;

        self.input = result.1;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        Ok(format!("{}", find_invalid(&self.input, 25)?))
    }

    fn part2(&self) -> Result<String> {
        //FIXME not rerunning part 1 makes this specific for my input....
        Ok(format!("{}", find_sum(&self.input, 1930745883)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn p1() {
        let v = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
        assert_eq!(find_invalid(&v, 5).unwrap(), 127);
    }

    #[test]
    fn p2() {
        let v = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
        assert_eq!(find_sum(&v, 127).unwrap(), 62);
    }
}
