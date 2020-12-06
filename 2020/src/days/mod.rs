use anyhow::{anyhow, Result};

//remove default impl and copy template from day_00
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
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
        1 => Ok(Box::new(day_01::Solution::default())),
        2 => Ok(Box::new(day_02::Solution::default())),
        3 => Ok(Box::new(day_03::Solution::default())),
        4 => Ok(Box::new(day_04::Solution::default())),
        5 => Ok(Box::new(day_05::Solution::default())),
        6 => Ok(Box::new(day_06::Solution::default())),
        7 => Ok(Box::new(day_07::Solution::default())),
        8 => Ok(Box::new(day_08::Solution::default())),
        9 => Ok(Box::new(day_09::Solution::default())),
        10 => Ok(Box::new(day_10::Solution::default())),
        11 => Ok(Box::new(day_11::Solution::default())),
        12 => Ok(Box::new(day_13::Solution::default())),
        13 => Ok(Box::new(day_13::Solution::default())),
        14 => Ok(Box::new(day_14::Solution::default())),
        15 => Ok(Box::new(day_15::Solution::default())),
        16 => Ok(Box::new(day_16::Solution::default())),
        17 => Ok(Box::new(day_17::Solution::default())),
        18 => Ok(Box::new(day_18::Solution::default())),
        19 => Ok(Box::new(day_19::Solution::default())),
        20 => Ok(Box::new(day_20::Solution::default())),
        21 => Ok(Box::new(day_21::Solution::default())),
        22 => Ok(Box::new(day_22::Solution::default())),
        23 => Ok(Box::new(day_23::Solution::default())),
        24 => Ok(Box::new(day_24::Solution::default())),
        25 => Ok(Box::new(day_25::Solution::default())),
        _ => Err(anyhow!("Day not available"))
    }
}

