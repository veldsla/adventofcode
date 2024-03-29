use anyhow::{anyhow, Result};

//remove default impl and copy template from day_00
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

use crate::Problem;

#[allow(clippy::box_default)]
pub fn get_solution(d: u32) -> Result<Box<dyn Problem>> {
    match d {
        1 => Ok(Box::<day_01::Solution>::default()),
        2 => Ok(Box::<day_02::Solution>::default()),
        3 => Ok(Box::<day_03::Solution>::default()),
        4 => Ok(Box::<day_04::Solution>::default()),
        5 => Ok(Box::<day_05::Solution>::default()),
        6 => Ok(Box::<day_06::Solution>::default()),
        7 => Ok(Box::<day_07::Solution>::default()),
        8 => Ok(Box::<day_08::Solution>::default()),
        9 => Ok(Box::<day_09::Solution>::default()),
        10 => Ok(Box::<day_10::Solution>::default()),
        11 => Ok(Box::<day_11::Solution>::default()),
        12 => Ok(Box::<day_12::Solution>::default()),
        13 => Ok(Box::<day_13::Solution>::default()),
        14 => Ok(Box::<day_14::Solution>::default()),
        15 => Ok(Box::<day_15::Solution>::default()),
        16 => Ok(Box::<day_16::Solution>::default()),
        17 => Ok(Box::<day_17::Solution>::default()),
        18 => Ok(Box::<day_18::Solution>::default()),
        19 => Ok(Box::<day_19::Solution>::default()),
        20 => Ok(Box::<day_20::Solution>::default()),
        21 => Ok(Box::<day_21::Solution>::default()),
        22 => Ok(Box::<day_22::Solution>::default()),
        23 => Ok(Box::<day_23::Solution>::default()),
        24 => Ok(Box::<day_24::Solution>::default()),
        25 => Ok(Box::<day_25::Solution>::default()),
        _ => Err(anyhow!("Day not available"))
    }
}

