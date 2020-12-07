use std::collections::{HashSet, HashMap};
use std::str;

use anyhow::{anyhow, Result};

use crate::Problem;

type Rules = HashMap<Color, Vec<(usize, Color)>>;

#[derive(Default)]
pub struct Solution {
    rules: HashMap<Color, Vec<(usize, Color)>>
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Color {
    modifier: String,
    color: String,
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, alpha1, line_ending, space1},
    multi::{many1, separated_list1},
    combinator::{all_consuming, eof, map},
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

//separate function because the use in numcol caused a move
fn color(i: &str) -> IResult<&str, Color> {
    map(separated_pair(alpha1, space1, alpha1), |(m, c): (&str, &str)| {
        Color { modifier: m.to_owned(), color: c.to_owned() }
    })(i)
}

fn parse(i: &str) -> IResult<&str, Vec<(Color, Vec<(usize, Color)>)>> {
    let outer = terminated(color, tag(" bags contain "));
    let numcol = map(terminated(separated_pair(digit1, space1, color), alt((tag(" bags"), tag(" bag")))),
        |(n, c)| (n.parse::<usize>().unwrap(), c));
    let inner = alt((separated_list1(tag(", "), numcol),
        map(tag("no other bags"), |_| Vec::new())));
    let rule = tuple((outer, terminated(inner, char('.'))));
    all_consuming(many1(terminated(rule, alt((line_ending, eof)))))(i)
}

fn can_contain(bag: &Color, l: &Rules) -> usize {
    let mut seen = HashSet::new();
    let mut queue = vec![bag];
    loop {
        if let Some(want) = queue.pop() {
            if !seen.insert(want) {
                continue;
            }
            for rule in l {
                if rule.1.iter().any(|(_, c)| c == want) {
                    queue.push(&rule.0);
                }
            }
        } else {
            return seen.len() - 1;
        }
    }
}

fn contains(bag: &Color, l: &Rules) -> usize {
    let mut queue = vec![(bag, 1)];
    let mut count = 0;
    loop {
        if let Some((want, n)) = queue.pop() {
            if let Some(contains) = l.get(want) {
                for (num, color) in contains {
                    count += n*num;
                    queue.push((color, *num * n));
                }
            }
        } else {
            return count;
        }
    }

}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(str::from_utf8(i)?).map_err(|e| anyhow!(e.to_string()))?;
        self.rules = result.1.into_iter().collect();
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let wanted = Color { modifier: "shiny".to_owned(), color: "gold".to_owned() };
        Ok(format!("{}", can_contain(&wanted, &self.rules)))
    }

    fn part2(&self) -> Result<String> {
        let wanted = Color { modifier: "shiny".to_owned(), color: "gold".to_owned() };
        Ok(format!("{}", contains(&wanted, &self.rules)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

    const P2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";
    #[test]
    fn parts() {
        let result = parse(TEST);
        assert!(result.is_ok());
        let l: Rules = result.unwrap().1.into_iter().collect();

        assert_eq!(l.len(), 9);
        let wanted = Color { modifier: "shiny".to_owned(), color: "gold".to_owned() };
        assert_eq!(can_contain(&wanted, &l), 4);

        assert_eq!(contains(&wanted, &l), 32);

        let l: Rules = parse(P2).unwrap().1.into_iter().collect();
        assert_eq!(contains(&wanted, &l), 126);
    }
}
