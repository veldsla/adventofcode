use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

use anyhow::{anyhow, Result};

pub trait Problem {
    #[allow(unused_variables)]
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        Err(anyhow!("Parser not yet implemented"))
    }

    fn part1(&self) -> Result<String> {
        Err(anyhow!("Part 1 not yet implemented"))
    }

    fn part2(&self) -> Result<String> {
        Err(anyhow!("Part 2 not yet implemented"))
    }
}


pub mod days;
pub mod parsers;

pub fn run_day<I: AsRef<Path>>(day: u32, input: I) -> Result<()> {
    let mut p = days::get_solution(day)?;

    let mut f = File::open(input)?;
    let mut b = Vec::new();
    f.read_to_end(&mut b)?;

    let t = Instant::now();
    p.parse(&b)?;
    println!("Day {} parsed ({:?})", day, t.elapsed());

    let t = Instant::now();
    let res1 = p.part1()?;
    println!("Day {} part 1: {} ({:?})", day, res1, t.elapsed());


    let t = Instant::now();
    let res2 = p.part2()?;
    println!("Day {} part 2: {} ({:?})", day, res2, t.elapsed());

    Ok(())
}

