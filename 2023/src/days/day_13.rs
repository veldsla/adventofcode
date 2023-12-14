use std::fmt;
use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::is_a,
    character::complete::line_ending,
    multi::{fold_many1, separated_list1},
    combinator::{all_consuming, eof, map},
    sequence::terminated,
    IResult
};

use crate::Problem;

#[derive(Default)]
pub struct Solution {
    patterns: Vec<Pattern>,
}

#[derive(Debug)]
struct Pattern {
    rows: Vec<u64>,
    cols: Vec<u64>,
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            writeln!(f,"{:#020b}", row)?;
        }
        Ok(())
    }
}

impl Pattern {

    fn dim_x(&self) -> usize {
        self.cols.len()
    }

    fn dim_y(&self) -> usize {
        self.rows.len()
    }

    fn find_mirror_row(&self, budget: usize) -> Option<usize> {
        let last = self.dim_y() - 1;

        let from0 = (1..=last).step_by(2)
            .find_map(|y| is_mirrored(&self.rows[0..=y], budget).then_some(y));

        let fromlast = (0..last).rev().step_by(2)
            .find_map(|y| is_mirrored(&self.rows[y..=last], budget).then_some(y));

        match (from0, fromlast) {
            (Some(a), Some(b)) => panic!("ambiguous mirror: {} or {}", a, b),
            (Some(a), None) => Some(a / 2),
            (None, Some(b)) => Some((b+last) / 2),
            (None, None) =>None,
        }
    }

    fn find_mirror_col(&self, budget: usize) -> Option<usize> {
        let last = self.dim_x() - 1;

        let from0 = (1..=last).step_by(2)
            .find_map(|x| is_mirrored(&self.cols[0..=x], budget).then_some(x));

        let fromlast = (0..last).rev().step_by(2)
            .find_map(|x| is_mirrored(&self.cols[x..=last], budget).then_some(x));

        match (from0, fromlast) {
            (Some(a), Some(b)) => panic!("ambiguous mirror: {} or {}", a, b),
            (Some(a), None) => Some(a / 2),
            (None, Some(b)) => Some((b+last) / 2),
            (None, None) => None,
        }
    }
}

/// tests if the slice is mirrored, requiring `budget` mismatches
/// this function assumes the length is even!
fn is_mirrored(v: &[u64], budget: usize) -> bool {
    debug_assert!(v.len() % 2 == 0);
    let mut budget = budget;
    v.iter().take(v.len() / 2)
        .zip(v.iter().rev())
        .all(|(a, b)| {
            if a == b {
                true
            } else if budget > 0 {
                let d = (a ^ b).count_ones() as usize;
                if d > budget {
                    false
                } else {
                    budget -= d as usize;
                    true
                }
            } else {
                false
            }
        }) && budget == 0
}

fn parse_grid(s: &str) -> IResult<&str, Pattern> {
    let gridline = terminated(is_a("#."), line_ending);

    map(fold_many1(gridline, || (Vec::new(), Vec::new()), |(mut r, mut c), line: &str| {
        let mut rv = 0;
        for (pos, char) in line.chars().enumerate() {
            if pos >= c.len() {
                c.push(0);
            }
            match char {
                '#' => {
                    rv |= 1 << pos;
                    c[pos] |= 1 << r.len();
                },
                _ => (),
            }
        }
        r.push(rv);
        (r, c)
    }), |(rows, cols)| Pattern { rows, cols })(s)
}

fn parse_input(s: &str) -> IResult<&str, Vec<Pattern>> {
    separated_list1(line_ending, parse_grid)(s)
}

impl Problem for Solution {
    fn parse(&mut self, s: &str) -> Result<()> {
        let (s, patterns) = all_consuming(terminated(parse_input, eof))(s)
            .map_err(|e| anyhow!("parse error: {:?}", e))?;
        dbg!(s);

        self.patterns = patterns;

        Ok(())
    }

    fn part1(&self) -> Result<String> {
        Ok(self.patterns.iter().filter_map(|p| {
            let mr = p.find_mirror_row(0);
            let mc = p.find_mirror_col(0);
            match (mr, mc) {
                (Some(x), None) => Some((x + 1 ) * 100),
                (None, Some(x)) => Some(x + 1),
                (Some(x), Some(y)) => panic!("both found?: {} {}", x, y),
                (None, None) => panic!("neither found"),
            }
        }).sum::<usize>().to_string())
    }

    fn part2(&self) -> Result<String> {
        Ok(self.patterns.iter().filter_map(|p| {
            let mr = p.find_mirror_row(1);
            let mc = p.find_mirror_col(1);
            match (mr, mc) {
                (Some(x), None) => Some((x + 1 ) * 100),
                (None, Some(x)) => Some(x + 1),
                (Some(x), Some(y)) => panic!("both found?: {} {}", x, y),
                (None, None) => panic!("neither found"),
            }
        }).sum::<usize>().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirror() {
        let a = vec![1,2,3,3,2,1,5];
        assert!(!is_mirrored(&a[0..7], 0));
        assert!(is_mirrored(&a[0..6], 0));

        let a = vec![1,3,1,4,4,1,1,1];
        assert!(!is_mirrored(&a, 0));
        assert!(is_mirrored(&a, 1));
        assert!(!is_mirrored(&a, 2));
    }
}
