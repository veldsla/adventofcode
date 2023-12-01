use std::path::PathBuf;

use clap::Parser;
use chrono::prelude::*;
use chrono::offset::FixedOffset;
use anyhow::{anyhow, Result};

use aoc2023::{run_day, run_all};

const YEAR:i32 = 2023;

#[derive(Debug, Parser)]
#[clap(name = "aoc2023", about = "Advent of Code 2023 runner.")]
struct Opt {
    #[clap(short = 'd', long = "day")]
    day: Option<u32>,
    #[clap(short = 'i', long = "input")]
    input: Option<PathBuf>,
    #[clap(long)]
    all: bool,
}

fn main() -> Result<()> {
    let opt = Opt::parse();

    if opt.all {
        run_all()
    } else {
        let now = Utc::now().with_timezone(&FixedOffset::west_opt(5 * 3600).unwrap());
        //let aoc_start = FixedOffset::west_opt(5 * 3600).unwrap().with_ymd_and_hms(YEAR, 12, 1, 0, 0, 0);
        let aoc_start = Utc.with_ymd_and_hms(YEAR, 12, 1, 0, 0, 0).unwrap().with_timezone(&FixedOffset::west_opt(5 * 3600).unwrap());
        let run_date = if let Some(day) = opt.day {
            //FixedOffset::west_opt(5 * 3600).unwrap().with_ymd_and_hms(YEAR, 12, day, 0, 0, 0)
            Utc.with_ymd_and_hms(YEAR, 12, day, 0, 0, 0).unwrap().with_timezone(&FixedOffset::west_opt(5 * 3600).unwrap())
        } else {
            now
        };

        if run_date < aoc_start || run_date > now {
            return Err(anyhow!("AoC day is in the future"));
        }

        let torun = run_date.day();
        let input = opt.input.unwrap_or_else(|| PathBuf::from(format!("inputs/day_{:02}.txt", torun)));

        println!("run day {} for {}", torun, input.display());

        run_day(torun, input)?;

        Ok(())
    }
}
