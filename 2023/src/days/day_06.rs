use std::ops::Range;
use anyhow::{anyhow, Result};

use nom::{
    bytes::complete::{tag},
    character::complete::{line_ending, space1},
    multi::{separated_list1},
    sequence::{delimited, terminated},
    IResult
};

use crate::parsers::positive_integer;
use crate::Problem;

#[derive(Default)]
pub struct Solution {
    races: Vec<Race>,
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn optimal_solutions(&self) -> Range<u64> {
        // we solve this from the equation
        // dist = (time - presstime) * speed
        // speed = presstime, time is given.
        // (t - p) * p > d
        // -p^2 + tp > d
        // -p^2 + tp - d> 0
        // plug in abc formula
        // p = (t +- sqrt(t^2 + 4d)) / -2
        
        let time = self.time as f64;
        let distance = self.distance as f64;
        let p1 = (-time + (time.powi(2) - 4.0 * distance).sqrt()) / -2.0;
        let p2 = (-time - (time.powi(2) - 4.0 * distance).sqrt()) / -2.0;

        // we use floor + 1 to account for exact solutions (considerinng the > and not >=)
        let p1 = p1.floor() as u64 + 1;
        let p2 = p2.ceil() as u64;
        p1..p2
    }
}


fn parse(s: &str) -> IResult<&str, Vec<Race>> {

    let (s, times) = delimited(terminated(tag("Time:"), space1), separated_list1(space1, positive_integer), line_ending)(s)?;
    let (s, distances) = delimited(terminated(tag("Distance:"), space1), separated_list1(space1, positive_integer), line_ending)(s)?;

    let result = times.into_iter().zip(distances.into_iter()).map(|(time, distance)| Race { time, distance }).collect();
    Ok((s, result))
}

impl Problem for Solution {
    fn parse(&mut self, s: &str) -> Result<()> {
        let (s, races) = parse(s).map_err(|e| anyhow!("Error: {:?}", e))?;

        self.races = races;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        Ok(self.races.iter()
           .map(|r| r.optimal_solutions().count())
           .product::<usize>().to_string())
    }

    fn part2(&self) -> Result<String> {
        let time = self.races.iter().map(|r| r.time.to_string()).collect::<String>().parse::<u64>().unwrap();
        let distance = self.races.iter().map(|r| r.distance.to_string()).collect::<String>().parse::<u64>().unwrap();

        let new_race = Race { time, distance };
        Ok(new_race.optimal_solutions().count().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parts() {
        let s = r#"Time:      7  15   30
Distance:  9  40  200
"#;
        let (s, races) = parse(s).unwrap();
        assert!(s.is_empty());

        assert_eq!(races[0].optimal_solutions(), 2..6);
        assert_eq!(races[1].optimal_solutions(), 4..12);
        assert_eq!(races[2].optimal_solutions(), 11..20);

        let time = races.iter().map(|r| r.time.to_string()).collect::<String>().parse::<u64>().unwrap();
        let distance = races.iter().map(|r| r.distance.to_string()).collect::<String>().parse::<u64>().unwrap();
        let new_race = Race { time, distance };
        dbg!(&new_race);
        assert!(false);
    }
}
