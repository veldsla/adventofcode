use anyhow::{anyhow, Result};

//remove default impl and copy template from day_00
mod day_01;
mod day_02 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_03 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_04 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_05 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_06 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_07 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_08 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_09 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_10 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_11 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_12 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_13 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_14 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_15 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_16 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_17 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_18 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_19 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_20 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_21 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_22 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_23 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_24 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}
mod day_25 {use crate::Problem; #[derive(Default)]pub struct Solution; impl Problem for Solution {}}

use crate::Problem;
pub fn get_solution(d: u32) -> Result<Box<dyn Problem>> {
    match d {
        1 => Ok(day_01::Solution::new()),
        2 => Ok(day_02::Solution::new()),
        3 => Ok(day_03::Solution::new()),
        4 => Ok(day_04::Solution::new()),
        5 => Ok(day_05::Solution::new()),
        6 => Ok(day_06::Solution::new()),
        7 => Ok(day_07::Solution::new()),
        8 => Ok(day_08::Solution::new()),
        9 => Ok(day_09::Solution::new()),
        10 => Ok(day_10::Solution::new()),
        11 => Ok(day_11::Solution::new()),
        12 => Ok(day_13::Solution::new()),
        13 => Ok(day_13::Solution::new()),
        14 => Ok(day_14::Solution::new()),
        15 => Ok(day_15::Solution::new()),
        16 => Ok(day_16::Solution::new()),
        17 => Ok(day_17::Solution::new()),
        18 => Ok(day_18::Solution::new()),
        19 => Ok(day_19::Solution::new()),
        20 => Ok(day_20::Solution::new()),
        21 => Ok(day_21::Solution::new()),
        22 => Ok(day_22::Solution::new()),
        23 => Ok(day_23::Solution::new()),
        24 => Ok(day_24::Solution::new()),
        25 => Ok(day_25::Solution::new()),
        _ => Err(anyhow!("Day not available"))
    }
}

