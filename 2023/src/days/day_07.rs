use std::collections::HashMap;
use anyhow::{anyhow, Result};
use nom::{
    character::complete::{anychar, line_ending, space1},
    multi::{many_m_n, separated_list1},
    sequence::preceded,
    IResult
};

use crate::parsers::positive_integer;
use crate::Problem;

#[derive(Default)]
pub struct Solution {
    hands: Vec<Game>,
}

impl Solution {
    fn sort_hands(&self, joker: bool) -> Vec<Game> {
        let mut hands = self.hands.clone();
        hands.sort_unstable_by(|a, b| {
            let vc = if joker {
                a.joker_value.cmp(&b.joker_value)
            } else {
                a.value.cmp(&b.value)
            };

            if vc != std::cmp::Ordering::Equal {
                vc
            } else {
                a.hand.0.iter().zip(b.hand.0.iter()).find_map(|(c1, c2)| {
                    let c = card_value(c1, joker).cmp(&card_value(c2, joker));
                    if c != std::cmp::Ordering::Equal {
                        Some(c)
                    } else {
                        None
                    }
                }).unwrap()
            }
        });

        hands
    }
}

#[derive(Debug, Clone)]
struct Game {
    hand: Hand,
    value: u64,
    joker_value: u64,
    bid: u64,
}

#[derive(Debug, Clone)]
struct Hand(Vec<char>);

fn parse_game(s: &str) -> IResult<&str, Game> {
    let (s, cards) = many_m_n(5,5, anychar)(s)?;
    let (s, bid) = preceded(space1, positive_integer)(s)?;

    let hand = Hand(cards);
    let value = hand_value(&hand);
    let joker_value = hand_value_joker(&hand);

    Ok((s, Game { hand, value, joker_value, bid }))
}

fn hand_value(hand: &Hand) -> u64 {
    let mut v = HashMap::new();
    for c in &hand.0 {
        let count = v.entry(c).or_insert(0);
        *count += 1;
    }

    match v.len() {
        5 => 1, // first card only
        4 => 2, // one pair
        3 => {
            if v.values().any(|&x| x == 2) {
                3 // two pair
            } else {
                4 // three of a kind
            }
        },
        2 => {
            if v.values().any(|&x| x == 3) {
                5 // full house
            } else {
                6 // four of a kind
            }
        },
        1 => 7, // five of a kind
        _ => panic!("Invalid hand"),
    }
}

fn hand_value_joker(hand: &Hand) -> u64 {
    if hand.0.contains(&'J') {
        let mut jokers = 0;
        let m = hand.0.iter().fold(HashMap::new(), |mut acc, c| {
            if *c == 'J' {
                jokers += 1;
            } else {
                let count = acc.entry(c).or_insert(0);
                *count += 1;
            }
            acc
        });
        
        match (jokers, m.len()) {
            (1, 4) => 2, //j + nothing => one pair
            (1, 3) => 4, //j + one pair => three of a kind
            (1, 2) => {
                if m.values().any(|&x| x == 2) {
                    5 // j + two pair => full house
                } else {
                    6 // j + three of a kind => four of a kind
                }
            },
            (1, 1) => 7, // j + four of a kind => five of a kind
            (2, 3) => 4, // 2j + nothing => three of a kind
            (2, 2) => 6, // 2j + one pair => four of a kind
            (2, 1) => 7, // 2j + three of a kind => five of a kind
            (3, 2) => 6, // 3j + nothing => four of a kind
            (3, 1) => 7, // 3j + one pair => five of a kind
            (4, 1) => 7, // 4j + nothing => five of a kind
            (5, 0) => 7, //
            _ => panic!("Invalid hand"),
        }
    } else {
        hand_value(hand)
    }
}

fn card_value(c: &char, joker: bool) -> u64 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => if joker { 1 } else { 11 },
        'T' => 10,
        _ => c.to_digit(10).unwrap() as u64,
    }
}

impl Problem for Solution {
    fn parse(&mut self, s: &str) -> Result<()> {
        let (_s, games) = separated_list1(line_ending, parse_game)(s).map_err(|e| anyhow!("parse error: {}", e))?;
        self.hands = games;

        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let hands = self.sort_hands(false);
        let sum:u64 = hands.into_iter()
            .enumerate()
            .map(|(i, hand)| {
                (i as u64 + 1) * hand.bid
            }).sum();

        Ok(sum.to_string())
    }

   fn part2(&self) -> Result<String> {
        let hands = self.sort_hands(true);
        let sum:u64 = hands.into_iter()
            .enumerate()
            .map(|(i, hand)| {
                (i as u64 + 1) * hand.bid
            }).sum();

        Ok(sum.to_string())
   }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parts() {
        let s = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;
        let (_s, hands) = separated_list1(line_ending, parse_game)(s).map_err(|e| anyhow!("parse error: {}", e)).unwrap();
        assert_eq!(hands[0].value, 2);
        assert_eq!(hands[1].value, 4);
        assert_eq!(hands[2].value, 3);
        assert_eq!(hands[3].value, 3);
        assert_eq!(hands[4].value, 4);

        // part 2
        assert_eq!(hands[0].joker_value, 2);
        assert_eq!(hands[1].joker_value, 6);
        assert_eq!(hands[2].joker_value, 3);
        assert_eq!(hands[3].joker_value, 6);
        assert_eq!(hands[4].joker_value, 6);
   }
}
