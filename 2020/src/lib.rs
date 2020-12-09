use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use std::time::{Instant, Duration};

use anyhow::{anyhow, Result};
use tabular::{Table, row};

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
pub mod grid;
pub mod vm;

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

pub fn run_all() -> Result<()> {
    let mut table = Table::new("{:<} {:<} {:<} {:<} {:<} {:<} {:<}");
    table.add_row(row!("day", "parse", "part1", "part2", "total", "output",""));
    let mut parse_time = Duration::new(0,0);
    let mut solution_time = Duration::new(0,0);
    let run_time = Instant::now();
    for day in 1..25 {
        let mut p = days::get_solution(day)?;
        if let Ok(mut f) = File::open(PathBuf::from(format!("inputs/day_{:02}.txt", day))) {
            let t = Instant::now();
            let mut b = Vec::new();
            f.read_to_end(&mut b)?;
            p.parse(&b)?;
            let tp = t.elapsed();
            parse_time += tp;

            let t = Instant::now();
            let res1 = p.part1()?;
            let tp1 = t.elapsed();
            solution_time += tp1;

            let t = Instant::now();
            let res2 = p.part2()?;
            let tp2 = t.elapsed();
            solution_time += tp2;
            //println!("{:?}\t{:?}\t\t{:?}\t{:?}\t{:?}\tout:{}, {}", day,  tp, tp1, tp2, tp+tp1+tp2, res1, res2);
            table.add_row(row!(day, format!("{:?}", tp), format!("{:?}",tp1), format!("{:?}",tp2), format!("{:?}",tp+tp1+tp2), res1, res2));
        }
    }
   table.add_row(row!("all", format!("{:?}", parse_time),"",format!("{:?}", solution_time), format!("{:?}", run_time.elapsed()),"",""));
   print!("{}", table);
   //println!("real time:\t{:?}", t0.elapsed());

    Ok(())
}

