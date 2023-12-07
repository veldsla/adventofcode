use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::{tag},
    character::complete::{alpha1, line_ending, space0, space1},
    combinator::{map, opt},
    multi::{fold_many1, many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult
};

use crate::parsers::positive_integer;
use crate::Problem;

#[derive(Default)]
pub struct Solution {
    seeds: Vec<u64>,
    maps: HashMap<(String, String), Vec<Map>>,
}

#[derive(Debug)]
struct Map {
    from: RangeInclusive<u64>,
    to: RangeInclusive<u64>,
}

impl Map {
    fn map_to(&self, i: u64) -> Option<u64> {
        if self.from.contains(&i) {
            Some(self.to.start() + i - self.from.start())
        } else {
            None
        }
    }

    fn sub_range(&self, r: &RangeInclusive<u64>) -> (Option<RangeInclusive<u64>>, Option<RangeInclusive<u64>>, Option<RangeInclusive<u64>>) {
        let ss = self.from.start();
        let se = self.from.end();
        if r.start() <= se && r.end() >= ss {
            //overlapping return converted range and possibly overhanging ranges
            let from = r.start().clamp(ss, se);
            let to = r.end().clamp(ss, se);

            let mapped_from = self.to.start() + from - ss;
            let mapped_to = mapped_from + to - from;

            let before = if r.start() < ss {
                Some(*r.start()..=ss-1)
            } else {
                None
            };

            let after = if r.end() > se {
                Some(se+1..=*r.end())
            } else {
                None
            };

            (before, Some(mapped_from..=mapped_to), after)

        } else {
            (None, None, None)
        } 

    }
}

fn parse_seeds(s: &str) -> IResult<&str, Vec<u64>> {
    delimited(tag("seeds: "), separated_list1(space1, positive_integer), line_ending)(s)
}

fn parse_maplines(s: &str) -> IResult<&str, Vec<Map>> {
     let rmap = tuple((
            terminated(positive_integer::<u64>, space1),
            terminated(positive_integer::<u64>, space1),
            terminated(positive_integer::<u64>, line_ending),
     ));

     fold_many1(rmap, Vec::new, |mut acc, (ds, ss, l)| {
         acc.push(Map { from: ss..=ss+l-1, to: ds..=ds+l-1 });
         acc
     })(s)
}

fn parse_map(s: &str) -> IResult<&str, ((String, String), Vec<Map>)> {
    let (s, (from, to)) = delimited(
        opt(line_ending),
        separated_pair(alpha1, tag("-to-"), alpha1),
        terminated(tag(" map:"), line_ending))(s)?;

    
    let (s, maps) = preceded(opt(line_ending), parse_maplines)(s)?;

    Ok((s, ((from.to_owned(), to.to_owned()), maps)))
}

impl Problem for Solution {
    fn parse(&mut self, s: &str) -> Result<()> {
        let (s, seeds) = parse_seeds(s).map_err(|e| anyhow!("Parse error {e}"))?;

        let (_s, maps) = fold_many1(parse_map, HashMap::new, |mut acc, (fromto, m)| {
            acc.insert(fromto, m);
            acc
        })(s).map_err(|e| anyhow!("Parse error {e}"))?;

        self.seeds = seeds;
        self.maps = maps;

        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let order = ["seed","soil","fertilizer","water","light","temperature", "humidity", "location"];

        let min = self.seeds.iter().copied().map(|mut id| {
            for pos in 0..7 {
                let start = &order[pos];
                let to = &order[pos+1];
                let maps = self.maps.get(&(start.to_string(), to.to_string())).unwrap();
                // Find the first hit in the two map (no overlaps right?)
                id = match maps.iter().find_map(|m| m.map_to(id)) {
                    Some(v) => v,
                    None => id,
                };
            }
            id
        }).min().unwrap();

        Ok(min.to_string())
    }

    fn part2(&self) -> Result<String> {
        let seedranges: Vec<RangeInclusive<u64>> = self.seeds.chunks_exact(2).map(|v| {
            v[0]..=v[0]+v[1]-1
        }).collect();

        let order = ["seed","soil","fertilizer","water","light","temperature", "humidity", "location"];

        let mut min = u64::MAX;

        let mut done = Vec::new();
        let mut unmapped = Vec::new();

        for r in seedranges.into_iter() {
            let mut search = vec![r];
            for pos in 0..7 {
                let start = &order[pos];
                let to = &order[pos+1];
                let maps = self.maps.get(&(start.to_string(), to.to_string())).unwrap();

                for map in maps {
                    while let Some(q) = search.pop() {
                        let (before, mapped, after) = map.sub_range(&q);
                        if let Some(m) = mapped {
                            done.push(m);
                            if let Some(b) = before {
                                unmapped.push(b);
                            }
                            if let Some(a) = after {
                                unmapped.push(a);
                            }
                        } else {
                            // continue searching with the original range
                            unmapped.push(q.clone());
                        }
                    }
                    // add the remaining unmapped ranges to the result
                    search.append(&mut unmapped);
                }
                // add the mapped stuff for the next round
                search.append(&mut done);
            }
            min = min.min(search.iter().map(|r| *r.start()).min().unwrap());
        }

        Ok(min.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range() {
        let map = Map { from: 50..=50+48, to: 52..=52+48 };
        assert_eq!(map.sub_range(&(77..=92)), (None, Some(79..=94), None));
        assert_eq!(map.sub_range(&(40..=105)), (Some(40..=49), Some(52..=100), Some(99..=105)));
    }
}


