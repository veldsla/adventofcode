use std::collections::HashMap;
use std::fmt;
use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of, space1},
    multi::{many1, separated_list1},
    combinator::{all_consuming, eof, map},
    sequence::{separated_pair, terminated},
    IResult
};

use crate::parsers::positive_integer;
use crate::Problem;

#[derive(Default)]
pub struct Solution {
    data: Vec<Springs>,
}

#[derive(Debug)]
struct Springs {
    data: Vec<Status>,
    broken: Vec<usize>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

impl Springs {
    fn count_arrangements(&self) -> usize {
        let mut cache = HashMap::new();
        count_arrangements(&self.data, 0, &self.broken, 0, &mut cache)
    }
}


fn count_arrangements(status: &[Status], pos: usize, broken: &[usize], broken_id: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(res) = cache.get(&(pos, broken_id)) {
        return *res;
    }

    let mut res = 0;

    for p in pos..status.len() {
        if p == 0 || status[p-1] != Status::Damaged {
            if let Some(used) = test_pattern_nbroken(&status[p..], broken[broken_id]) {
                let newpos = p + used;
                if broken_id == broken.len() - 1 && status[p+used..].iter().all(|&e| e != Status::Damaged) {
                    res += 1;
                } else if broken_id < broken.len() - 1 {
                    res += count_arrangements(status, newpos, broken, broken_id + 1, cache);
                }
            }
        } else {
            break;
        }
    }

    cache.insert((pos, broken_id), res);

    res
}

fn test_pattern_nbroken(slice: &[Status], n: usize) -> Option<usize> {
    if slice.len() < n {
        return None
    }

    let mut it = slice.iter();

    for _ in 0..n {
        if it.next().is_some_and(|&e| e == Status::Operational) {
            return None
        }
    }

    match it.next() {
        None => Some(n),
        Some(status) =>  {
            if status != &Status::Damaged {
                Some(n + 1)
            } else {
                None
            }
        }
    }
}

fn parse_line(s: &str) -> IResult<&str, Springs> {
    let data = many1(map(one_of("#.?"), |c| match c {
        '.' => Status::Operational,
        '#' => Status::Damaged,
        '?' => Status::Unknown,
        _ => unreachable!(),
    }));
    let damaged = separated_list1(tag(","), positive_integer::<usize>);

    map(terminated(separated_pair(data, space1, damaged), line_ending),
        |(data, broken)| Springs { data, broken })(s)
}

impl Problem for Solution {
    fn parse(&mut self, s: &str) -> Result<()> {
        let (_s, springs) = many1(parse_line)(s)
            .map_err(|e| anyhow!("parse error: {e}"))?;

        //dbg!(&springs);
        eprintln!("max: {:?}", springs.iter().map(|s| s.data.len()).max());
        self.data = springs;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        Ok(self.data.iter().map(|s| s.count_arrangements()).sum::<usize>().to_string())
    }

    fn part2(&self) -> Result<String> {
        // stick 5 together
        let newsprings:Vec<_> = self.data.iter().map(|s| {
            let mut springs:Vec<_> = (0..5).fold(Vec::new(), |mut acc, _i| {
                acc.extend(&s.data);
                acc.push(Status::Unknown);
                acc
            });
            springs.pop();

            let broken = (0..5).fold(Vec::new(), |mut acc, _i| {
                acc.extend(&s.broken);
                acc
            });
            Springs {data: springs, broken}

        }).collect();

        Ok(newsprings.iter().map(|s| s.count_arrangements()).sum::<usize>().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count() {
        let s = ".??..??...?##. 1,1,3\n";
        let (s, springs) = parse_line(&s).unwrap();

        assert_eq!(test_pattern_nbroken(&springs.data, 1), None);
        assert_eq!(test_pattern_nbroken(&springs.data[1..], 1), Some(2));
        assert_eq!(test_pattern_nbroken(&springs.data[2..], 1), Some(2));
        assert_eq!(test_pattern_nbroken(&springs.data[1..], 2), Some(3));
        assert_eq!(test_pattern_nbroken(&springs.data[2..], 2), None);
        assert_eq!(springs.count_arrangements(), 4);

        let (_, springs) = parse_line("?###???????? 3,2,1\n").unwrap();
        assert_eq!(springs.count_arrangements(), 10);

        let (_, springs) = parse_line("????.######..#####. 1,6,5\n").unwrap();
        assert_eq!(springs.count_arrangements(), 4);

        let (_, springs) = parse_line("???.### 1,1,3\n").unwrap();
        assert_eq!(springs.count_arrangements(), 1);

        let (_, springs) = parse_line("?#?#?#?#?#?#?#? 1,3,1,6\n").unwrap();
        assert_eq!(springs.count_arrangements(), 1);

        let (_, springs) = parse_line("????.#...#... 4,1,1\n").unwrap();
        assert_eq!(springs.count_arrangements(), 1);

        let (_, springs) = parse_line("?.?????????? 2,5\n").unwrap();
        assert_eq!(springs.count_arrangements(), 6);
    }
}
