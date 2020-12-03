use std::path::PathBuf;

use structopt::StructOpt;
use chrono::prelude::*;
use chrono::offset::FixedOffset;
use anyhow::{anyhow, Result};

use aoc2020::{run_day, run_all};

const YEAR:i32 = 2020;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2020", about = "Advent of Code 2020 runner.")]
struct Opt {
    #[structopt(short = "d", long = "day")]
    day: Option<u32>,
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input: Option<PathBuf>,
    #[structopt(long)]
    all: bool,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    if opt.all {
        run_all()
    } else {
        let now = Utc::now().with_timezone(&FixedOffset::west(5 * 3600));
        let aoc_start = FixedOffset::west(5 * 3600).ymd(YEAR, 1, 1).and_hms(0,0,0);
        let run_date = if let Some(day) = opt.day {
            FixedOffset::west(5 * 3600).ymd(YEAR, 12, day).and_hms(0,0,0)
        } else {
            now.clone()
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
