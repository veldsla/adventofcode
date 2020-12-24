use anyhow::{anyhow, Result};

use crate::Problem;

#[derive(Default)]
pub struct Solution {
    seats: Vec<u16>
}

use nom::{
    bytes::complete::take,
    character::complete::line_ending,
    multi::many1,
    combinator::{all_consuming, map_res},
    sequence::terminated,
    IResult
};
fn parse(i: &[u8]) -> IResult<&[u8], Vec<u16>> {
    all_consuming(many1(map_res(terminated(take(10usize), line_ending), |s: &[u8]| {
        let mut r = 0;
        for (i, c) in s.iter().enumerate() {
            match c {
                b'B' | b'R' => r |= 1 << (9 - i),
                b'F' | b'L' => {},
                _ => return Err(anyhow!("unknown char"))
            }
        }
        Ok(r)
    })))(i)
}


impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(i).map_err(|e| anyhow!(e.to_string()))?;
        self.seats = result.1;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let highest = self.seats.iter().max().ok_or_else(|| anyhow!("No solution"))?;
        Ok(format!("{}", highest))
    }

    fn part2(&self) -> Result<String> {
        let mut ids: Vec<_> = self.seats.clone();
        ids.sort();
        let seat = ids.windows(2)
            .find(|w| w[1] - w[0] > 1)
            .map(|w| w[0]+1)
            .ok_or_else(|| anyhow!("No solution"))?;

        Ok(format!("{:?}", seat))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let seat = parse(b"BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL\n").unwrap().1;
        assert_eq!(seat,  vec![567, 119, 820]);
    }
}

