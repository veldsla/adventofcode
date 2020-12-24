use std::collections::HashMap;
use std::str;
use std::ops::RangeInclusive;

use anyhow::{anyhow, Result};

use crate::Problem;
use crate::parsers::{commasep_positive_integer, range_positive_integer};

use nom::{
    bytes::complete::tag,
    bytes::complete::is_not,
    character::complete::line_ending,
    combinator::map,
    multi::many1,
    combinator::all_consuming,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult
};

#[derive(Default)]
pub struct Solution {
    input: Option<Data>
}

#[derive(Debug)]
struct Data {
    rules: Vec<Rule>,
    my_ticket: Vec<u16>,
    other_tickets: Vec<Vec<u16>>,
}

#[derive(Debug)]
struct Rule {
    name: String,
    r1: RangeInclusive<u16>,
    r2: RangeInclusive<u16>,
}

impl Rule {
    fn in_range(&self, v: &u16) -> bool {
        self.r1.contains(&v) || self.r2.contains(&v)
    }

    fn in_range_all<'a, I: IntoIterator<Item=&'a u16> + 'a>(&self, e: I) -> bool {
        e.into_iter().all(|v| self.in_range(v))
    }
}

impl Data {
    fn completely_invalid(&self) -> u16 {
        self.other_tickets
            .iter()
            .flat_map(|t| {
                t.iter()
                    .filter(|v| !self.rules.iter().any(|r| r.in_range(v)))
            }).sum()
    }

    fn decode(&self) -> u64 {
        let valid: Vec<_> = self.other_tickets
            .iter()
            .filter(|t| {
                !t.iter()
                    .any(|v| !self.rules.iter().any(|r| r.in_range(v)))
            }).collect();

        //FIXME this should be part of the parser
        assert!(self.other_tickets.iter().all(|t| t.len() == self.rules.len()));

        let nfields = self.rules.len();
        let ntickets = valid.len();

        //Vec of iterators
        let mut byfield: Vec<_> = valid.iter().map(|row| row.iter()).collect();
        let columns: Vec<Vec<u16>> = (0..nfields)
            .map(|_col| {
                //take one from each row
                byfield.iter_mut().fold(Vec::with_capacity(ntickets), |mut col, it| {
                    col.push(*it.next().unwrap());
                    col
                })
            }).collect();

        let mut known = HashMap::with_capacity(nfields);
        let mut rules: HashMap<_, _> = self.rules.iter().enumerate().collect();

        while known.len() < nfields {
            for col in 0..nfields {
                if known.contains_key(&col) {
                    continue;
                }
                //find the columns that fit one remaining rule
                let fits: Vec<_> = rules.iter().filter_map(|(id, rule)| {
                    if rule.in_range_all(&columns[col]) {
                        Some(*id)
                    } else {
                        None
                    }
                }).collect();

                if fits.len() == 1 {
                    known.insert(col, rules.remove(&fits[0]).unwrap());
                }
            }
        }
        
        known.into_iter()
            .filter(|(_, r)| r.name.starts_with("departure"))
            .map(|(col, _)| self.my_ticket[col] as u64)
            .product()
    }
}

fn parse(i: &str) -> IResult<&str, Data> {
    let rangepair = separated_pair(range_positive_integer, tag(" or "), range_positive_integer);
    let valid = map(terminated(separated_pair(is_not(":"), tag(": "), rangepair), line_ending), 
        |(s, p)| Rule { name: s.to_owned(), r1: p.0, r2: p.1 });
    let rules = terminated(many1(valid), line_ending);

    let my = preceded(terminated(tag("your ticket:"), line_ending), 
        terminated(commasep_positive_integer, line_ending));
    let other = preceded(terminated(tag("nearby tickets:"), line_ending), 
        many1(terminated(commasep_positive_integer, line_ending)));

    all_consuming(map(tuple((rules, terminated(my, line_ending), other)), 
            |(rules, my_ticket, other_tickets)| Data {rules, my_ticket, other_tickets} ))(i)
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(str::from_utf8(i)?).map_err(|e| anyhow!(e.to_string()))?;
        self.input = Some(result.1);
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let data = self.input.as_ref().ok_or_else(|| anyhow!("Not parsed"))?;
        Ok(format!("{}", data.completely_invalid()))
    }

    fn part2(&self) -> Result<String> {
        let data = self.input.as_ref().ok_or_else(|| anyhow!("Not parsed"))?;
        Ok(format!("{}", data.decode()))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
const TEST: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";

    #[test]
    fn p1() {
        let result = parse(TEST);
        assert!(result.is_ok());
        let data = result.unwrap().1;
        assert_eq!(data.my_ticket, vec![7,1,14]);
        assert_eq!(data.other_tickets.len(), 4);
        assert_eq!(data.other_tickets[3], vec![38,6,12]);

        assert_eq!(data.completely_invalid(), 71);
    }

    #[test]
    fn p2() {
        let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
";
        let data = parse(input).unwrap().1;
        assert_eq!(data.completely_invalid(), 0);
        //not functional
        assert_eq!(data.decode(), 1);
    
    }
}
