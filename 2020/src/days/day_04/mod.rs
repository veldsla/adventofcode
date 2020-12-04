use std::collections::HashMap;
use std::str;

use anyhow::{anyhow, Result};

use crate::Problem;

#[derive(Default)]
pub struct Solution {
    passports: Vec<PassPort>
}

use nom::{
    branch::alt,
    bytes::complete::{is_not, take, take_while_m_n, tag},
    character::complete::{char, digit1, line_ending, space1},
    multi::{fold_many1, many1},
    combinator::{all_consuming, eof, verify},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Clone, Debug)]
struct PassPort(HashMap<String, String>);
impl PassPort {
    fn is_valid(&self) -> bool {
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .into_iter()
            .all(|k| self.0.contains_key(k))
    }

    fn is_valid_strict(&self) -> bool {
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .into_iter()
            .all(|k| self.0.contains_key(k) && self.check_key(k))
    }

    fn check_key(&self, key: &str) -> bool {
        let val: &str = &self.0[key];
        match key {
            "byr" => year(val, 1920, 2002).is_ok(),
            "iyr" => year(val, 2010, 2020).is_ok(),
            "eyr" => year(val, 2020, 2030).is_ok(),
            "hgt" => height(val).is_ok(),
            "hcl" => hair(val).is_ok(),
            "ecl" => val == "amb" || val == "blu" || val == "brn" || val == "gry" || val == "grn" || val == "hzl" || val == "oth",
            "pid" => val.len() == 9,
            _ => true
        }

    }
}

fn parse(i: &str) -> IResult<&str, Vec<PassPort>> {
    let item = separated_pair(take(3usize), char(':'), terminated(is_not("\r\n "), alt((space1, line_ending))));
    let passport = fold_many1(item, PassPort(HashMap::new()), |mut pp: PassPort, (key, value):(&str, &str)| {
        pp.0.insert(key.to_owned(), value.to_owned());
        pp
    });
    all_consuming(many1(terminated(passport, alt((line_ending, eof)))))(i)
}

//only used in verification:
fn year(i: &str, min: u16, max: u16) -> IResult<&str, &str> {
    verify(all_consuming(digit1), |n: &str| {
            if let Ok(y) = n.parse::<u16>() {
                y >= min && y <= max
            }else {
                false
            }
    })(i)
}
fn height(i: &str) -> IResult<&str, (&str, &str)> {
    verify(tuple((digit1, alt((tag("cm"), tag("in"))))), |(n, unit): &(&str, &str)| {
        if let Ok(n) = n.parse::<u8>() {
            match unit {
                &"in" if n >= 59 && n <= 76 => true,
                &"in" => false,
                &"cm" if n >= 150 && n <= 193 => true,
                &"cm" => false,
                _=> false,
            }
        } else {
            false
        }
    })(i)
}

fn hair(i: &str) -> IResult<&str, &str> {
    all_consuming(preceded(char('#'), take_while_m_n(6, 6, |d: char| d.is_ascii_hexdigit())))(i)
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(str::from_utf8(i)?).map_err(|e| anyhow!(e.to_string()))?;
        self.passports = result.1;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let valid = self.passports.iter().filter(|pp| pp.is_valid()).count();
        Ok(format!("{}/{}", valid, self.passports.len()))
    }

    fn part2(&self) -> Result<String> {
        let valid = self.passports.iter().filter(|pp| pp.is_valid_strict()).count();
        Ok(format!("{}/{}", valid, self.passports.len()))
    }
}

mod tests {
    use super::parse;

    #[test]
    fn p1() {
     const LIST: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";
       let pp = parse(LIST).unwrap();
        println!("{:?}", pp);
        let l = pp.1;
        assert_eq!(l.len(), 4);
        assert_eq!(l[2].0["hgt"], "179cm");
        assert_eq!(l.iter().filter(|pp| pp.is_valid()).count(), 2);
    }


}

