use std::collections::HashSet;
use std::iter::FromIterator;
use anyhow::{anyhow, Result};

use nom::{
    bytes::complete::{tag},
    character::complete::{line_ending, space0, space1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult
};

use crate::parsers::positive_integer;
use crate::Problem;

#[derive(Default)]
pub struct Solution {
    data: Vec<Card>,
}

struct Card {
    num: u32,
    list1: HashSet<u32>,
    list2: HashSet<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let hits = self.matching();
        if hits > 0 {
            2u32.pow(hits - 1)
        } else {
            0
        }
    }

    fn matching(&self) -> u32 {
        self.list1.intersection(&self.list2).count() as u32
    }
}

fn list(s: &str) -> IResult<&str, HashSet<u32>> {
    map(separated_list1(space1, positive_integer),
    |v: Vec<u32>| HashSet::from_iter(v.into_iter()))(s)
}

fn parse_card(s: &str) -> IResult<&str, Card> {
    let (s, num) = delimited(
        terminated(tag("Card"), space1),
        positive_integer,
        terminated(tag(":"), space1))(s)?;

    let (s, (list1, list2)) = separated_pair(&list, delimited(space0, tag("|"), space0), list)(s)?;

    Ok((s, Card { num, list1, list2 }))
}


impl Problem for Solution {
    fn parse(&mut self, s: &str) -> Result<()> {
        let (_s, cards) = many1(terminated(parse_card, line_ending))(s)
            .map_err(|e| anyhow!("Error parsing cards: {}", e))?;

        self.data = cards;

        Ok(())

    }

    fn part1(&self) -> Result<String> {
        Ok(self.data.iter().map(|c| c.score()).sum::<u32>().to_string())
    }

    fn part2(&self) -> Result<String> {
        let scores: Vec<_> = self.data.iter().map(|c| c.matching() as usize).collect();

        let len = self.data.len();
        let mut have_cards = vec![1u64; len];

        for pos in 0..len {
            let score = scores[pos];
            let num = have_cards[pos];
            if score > 0 {
                let from = (pos + 1).clamp(0, len);
                let to = (pos + score + 1).clamp(0, len);
                have_cards[from..to].iter_mut().for_each(|h| *h += num);
            }
        }

        let n: u64 = have_cards.iter().sum();
        Ok(n.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse() -> Vec<Card> {
        let s = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;
        many1(terminated(parse_card, line_ending))(s).unwrap().1
    }

    #[test]
    fn p1() {
        let cards = parse();

        let scores: Vec<_> = cards.iter().map(|c| c.score()).collect();
        assert_eq!(scores, vec![8,2,2,1,0,0])

    }

    #[test]
    fn p2() {
        let cards = parse();

        let scores: Vec<_> = cards.iter().map(|c| c.matching() as usize).collect();
        let len = cards.len();
        let mut have_cards = vec![1u64; len];

        for pos in 0..len {
            let score = scores[pos];
            let num = have_cards[pos];
            if score > 0 {
                let from = (pos + 1).clamp(0, len);
                let to = (pos + score + 1).clamp(0, len);
                have_cards[from..to].iter_mut().for_each(|h| *h += num);
            }
        }

        let n: u64 = have_cards.iter().sum();
        assert_eq!(n, 30);
    }
}

