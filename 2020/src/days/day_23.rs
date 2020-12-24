use std::str;

use anyhow::{anyhow, Result};

use crate::Problem;

use nom::{
    branch::alt,
    character::complete::{line_ending, one_of},
    multi::many1,
    combinator::{all_consuming, map, eof},
    sequence::terminated,
    IResult
};

#[derive(Default)]
pub struct Solution {
    cups: Vec<u32>,
}

// naive solution used in day1
fn play(v: &[u32], n: usize) -> Vec<u32> {
    let max = v.iter().copied().max().unwrap_or(0);

    let mut cups: Vec<u32> = v.iter().copied().collect();
    let mut next_round: Vec<u32> = Vec::with_capacity(cups.len());

    for _ in 0..n {
        ////println!("cups {:?}", cups);
        //a round
        let current = cups[0];
        //find target
        let mut target = current;
        let target_pos = loop {
            target = if target == 1 { max } else { target - 1};
            //println!("target {}", target);
            if let Some(pos) = cups[4..].iter().position(|&v| v == target) {
                break pos;
            }
        };
        //println!("target pos {}", target_pos);
        next_round.extend(&cups[4..5+target_pos]);
        next_round.extend(&cups[1..4]);
        next_round.extend(&cups[5+target_pos..]);
        next_round.push(current);
        //println!("next {:?}", next_round);
        std::mem::swap(&mut cups, &mut next_round);
        next_round.clear();
    }
    cups
}
fn string_from_one(v: &[u32]) -> String {
    let pos = v.iter().position(|&v| v == 1).expect("No one?");
    v[pos+1..].iter().chain(&v[0..pos]).map(|v| format!("{}", v)).collect()
}


#[derive(Debug)]
struct MyLL {
    data: Vec<u32>,
    links: Vec<usize>,
    map: Vec<usize>,
    origin: usize,
}


impl std::iter::FromIterator<u32> for MyLL {
    fn from_iter<I: IntoIterator<Item=u32>>(i: I) -> MyLL {
        let data: Vec<_> = i.into_iter().collect();
        let mut links: Vec<usize> = Vec::with_capacity(data.len());
        let mut  map = vec![0; data.len()+1];
        for (i, &v) in data.iter().enumerate() {
            links.push(if i == data.len() - 1 { 0 } else { i+1 });
            map[v as usize] = i;
        }
        MyLL { data, links, map, origin: 0 }
    }
}

impl MyLL {
    fn find_target(&mut self) ->(usize, usize) {
        //if any of the next 3 are the target lower it
        let mut target = self.data[self.origin] - 1;
        if target == 0 { target = self.data.len() as u32; }
        let ao = self.links[self.origin];

        let a = self.data[ao];
        let aa = self.links[ao];

        let b = self.data[aa];
        let ab = self.links[aa];

        let c = self.data[ab];

        while target == a || target == b || target == c {
            target -= 1;
            if target == 0 { target = self.data.len() as u32; }
        }

        let t_index = self.map[target as usize];

        (ab, t_index)
    }
    
    fn move_cups(&mut self, c_index: usize, target_index: usize) {
        // move three cups head to after third cup
        //
        let after_head_old = self.links[self.origin];
        let after_target_old = self.links[target_index];
        let after_c_old = self.links[c_index];
        // after target the moved cups
        self.links[target_index] = after_head_old;
        //the c cups links to what's behind the target
        self.links[c_index] = after_target_old;
        // after head comes what was after c cup
        self.links[self.origin] = after_c_old;
        // active moves next
        self.origin = self.links[self.origin];

    }

    fn play_round(&mut self) {
        let (ic, target_index) = self.find_target();
        self.move_cups(ic, target_index);
    }

    fn mul_after_one(&self) -> u64 {
        let pos = self.map[1];
        let a1 = self.links[pos];
        let a2 = self.links[a1];
        self.data[a1] as u64 *self.data[a2] as u64
    }
}

fn play_big(v: &[u32], max: u32, n: usize) -> u64 {
    let l = v.len() as u32;
    let mut ll: MyLL = v.iter().copied().chain((l + 1)..=max).collect();
    for _r in 0..n {
        ll.play_round();
    }
    ll.mul_after_one()
}

fn parse(i: &str) -> IResult<&str, Vec<u32>> {
    all_consuming(terminated(
            many1(map(one_of("123456789"), |c: char| c.to_digit(10).expect("not a number")  )),
            alt((line_ending, eof))
        ))(i)
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(str::from_utf8(i)?).map_err(|e| anyhow!(e.to_string()))?;
        self.cups = result.1;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let res = play(&self.cups, 100);
        Ok(string_from_one(&res))
    }

    fn part2(&self) -> Result<String> {
        Ok(format!("{}", play_big(&self.cups, 1_000_000, 10_000_000)))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let result = parse("389125467\n");
        println!("{:?}", result);
        assert!(result.is_ok());
        let cups = result.unwrap().1;
        assert_eq!(cups, vec![3,8,9,1,2,5,4,6,7]);
        let res = play(&cups, 10);
        assert_eq!(string_from_one(&res), "92658374");
        let res = play(&cups, 100);
        assert_eq!(string_from_one(&res), "67384529");
    }

    #[test]
    fn p2() {
        let result = parse("389125467\n");
        println!("{:?}", result);
        assert!(result.is_ok());
        let cups = result.unwrap().1;
        //let res = play_big(&cups, 9, 10);
        let res = play_big(&cups, 1_000_000, 10_000_000);
        assert_eq!(res, 149245887792);
    }
 }
