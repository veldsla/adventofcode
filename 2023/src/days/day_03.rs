#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::{tag},
    character::complete::{line_ending, none_of},
    IResult
};

use crate::parsers::positive_integer;
use crate::Problem;

type Numbers = HashMap<(u32, u32), (usize, u32)>;
type Symbols = HashMap<(u32, u32), char>;

#[derive(Default)]
pub struct Solution {
    numbers: Numbers,
    symbols: Symbols,
}

impl Solution {
    fn numbers_adj_symbols(&self) -> u32 {
        let mut seen = HashSet::new();
        let mut sum = 0;

        for (x, y) in self.symbols.keys() {
            seen.clear();
            for c in self.adj_coords(*x, *y) {
                if let Some((id, num)) = self.numbers.get(&c) {
                    if !seen.contains(id) {
                        seen.insert(*id);
                        sum += num;
                    }
                }
            }
        }

        sum
    }

    fn gear_ratios(&self) -> u32 {
        let mut seen = HashSet::new();
        let mut sum = 0;

        'symbol: for ((x, y), _) in self.symbols.iter().filter(|(_, c)| **c == '*') {
            seen.clear();
            let mut count = 0;
            let mut prod = 1;
            for (id, num) in self.adj_coords(*x, *y).filter_map(|c| self.numbers.get(&c)) {
                if seen.contains(id) {
                    continue;
                }
                count += 1;
                if count > 2 {
                    break 'symbol;
                }
                seen.insert(*id);
                prod = prod * num;
            }
            if count == 2 {
                sum += prod;
            }
        }

        sum
    }

    /// This generates illegal coordinates
    fn adj_coords(&self, x: u32, y: u32) -> impl Iterator<Item=(u32, u32)> + '_ {
        (x-1..x+2).cartesian_product(y-1..y+2)
    }

}


fn parse(mut s: &str) -> IResult<&str, (Numbers, Symbols)> {
    let mut numbers = Numbers::new();
    let mut symbols = Symbols::new();
    
    let mut y = 0;
    let mut x = 0;
    let mut numid = 0;

    loop {
        if let Ok((rem, _)) = line_ending::<&str, nom::error::Error<&str>>(s) {
            y += 1;
            x = 0;
            s = rem;
        } else if let Ok((rem, num)) = positive_integer::<u32>(s) {
            let l = num.ilog10() + 1;
            for d in 0..l {
                numbers.insert((x+d, y), (numid, num));
            }
            x += l;
            numid += 1;
            s = rem;
        } else if let Ok((rem, sym)) = none_of::<&str, &str, nom::error::Error<&str>>(".0123456789")(s) {
            symbols.insert((x, y), sym);
            x += 1;
            s = rem;
        } else if let Ok((rem, _)) = tag::<&str, &str, nom::error::Error<&str>>(".")(s) {
            x += 1;
            s = rem;
        } else {
            break;
        }
    }

    Ok((s, (numbers, symbols)))
}



impl Problem for Solution {
    fn parse(&mut self, i: &str) -> Result<()> {
        let (_rem, (numbers, symbols)) = parse(i).unwrap();
        self.numbers = numbers;
        self.symbols = symbols;

        Ok(())
    }

    fn part1(&self) -> Result<String> {
        Ok(self.numbers_adj_symbols().to_string())
    }

    fn part2(&self) -> Result<String> {
        Ok(self.gear_ratios().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

        let (rem, (numbers, symbols)) = parse(input).unwrap();
        assert_eq!(rem, "");
        assert_eq!(symbols.len(), 6);

        assert_eq!(numbers.get(&(0, 4)), Some(&(4, 617)));
        assert_eq!(symbols.get(&(3, 4)), Some(&'*'));

        let sol = Solution { numbers, symbols };
        assert_eq!(sol.numbers_adj_symbols(), 4361);
        assert_eq!(sol.gear_ratios(), 467835);
    }
}

