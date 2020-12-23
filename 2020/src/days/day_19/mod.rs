use std::collections::{HashMap, HashSet};
use std::str;

use anyhow::{anyhow, Result};

use crate::Problem;
#[derive(Default)]
pub struct Solution {
    input: Data
}

type Set = Vec<usize>;
#[derive(Debug, Clone)]
enum Rule {
    Char(char),
    Seq(Set),
    SeqOr((Set, Set))
}

#[derive(Clone, Default, Debug)]
struct Rules(HashMap<usize, Rule>);

#[derive(Default, Debug)]
struct Data {
    rules: Rules,
    messages: Vec<String>
}

fn rule_substrs<'a>(s: &'a str, rule: usize, rules: &Rules) -> Vec<&'a str> {
    match &rules.0[&rule] {
        Rule::Char(c) => {
            let mut res = Vec::new();
            if let Some(t) = s.chars().next() {
                if t == *c {
                    res.push(&s[1..]);
                }
            }
            res
        },
        Rule::Seq(v) => {
            let mut q = vec![s];
            for ri in v {
                let newsubs: Vec<_> = q.iter().flat_map(|subs| rule_substrs(subs, *ri, rules)).collect();
                if newsubs.is_empty() {
                    return Vec::new();
                }
                q = newsubs;
            }
            q
        },
        Rule::SeqOr((v1, v2)) => {
            let mut q = vec![s];
            for ri in v1 {
                let newsubs: Vec<_> = q.iter().flat_map(|subs| rule_substrs(subs, *ri, rules)).collect();
                q = newsubs;
            }
            let res1 = q;

            // OR

            q = vec![s];
            for ri in v2 {
                let newsubs: Vec<_> = q.iter().flat_map(|subs| rule_substrs(subs, *ri, rules)).collect();
                q = newsubs;
            }

            res1.into_iter().chain(q.into_iter()).collect()
        }
    }
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char, line_ending, space1},
    combinator::map,
    multi::{many1, separated_list1},
    combinator::all_consuming,
    sequence::{delimited,separated_pair, terminated, tuple},
    IResult
};
use crate::parsers::positive_integer;

fn seq(i: &str) -> IResult<&str, Set> {
    separated_list1(space1, positive_integer)(i)
}

fn rules(i: &str) -> IResult<&str, Rules> {
    let num = terminated(positive_integer, tag(": "));
    let orseq = map(separated_pair(seq, tag(" | "), seq), |t| Rule::SeqOr(t));
    let letter = map(delimited(char('"'), anychar, char('"')), |c| Rule::Char(c));
    let rule = tuple((num, alt((letter, orseq, map(seq, Rule::Seq)))));
    let (i, rules) = terminated(many1(terminated(rule, line_ending)), line_ending)(i)?;
    Ok((i, Rules(rules.into_iter().collect())))
}

fn parse(i: &str) -> IResult<&str, Data> {
    let (i, rules) = rules(i)?;
    let (i, messages) = all_consuming(many1(map(terminated(alpha1, line_ending), |s: &str| s.to_owned())))(i)?;
    Ok((i, Data { rules, messages }))
}


impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(str::from_utf8(i)?).map_err(|e| anyhow!(e.to_string()))?;
        self.input = result.1;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let valid = &self.input.messages
            .iter()
            .filter(|m| rule_substrs(m, 0, &self.input.rules).iter().any(|remain| remain.len() == 0))
            .count();
        Ok(format!("{}", valid))
    }

    fn part2(&self) -> Result<String> {
        // rule 0 : 8 11
        // rule 8 : 42
        // rule 11: 42 31
        //
        // new rule 8: 42 | 42 8 
        // new rule 11: 42 31 | 42 11 31
        //
        // First test multiples of 42 to satisfy recursive rule 8
        // Rule 11 stars with 42 which has already been consumed
        // Recursive rule 11 adds more 42 which has already been consumed
        // Finish with 1+ number of 31. Rule 8 makes sure there's no imbalance
        let rules = &self.input.rules; 
        let mut valid = 0;
        for m in &self.input.messages {
            let matches = rule_substrs(&m[..], 42, rules);
            let q: HashSet<&str> = matches.iter().flat_map(|subs| rule_substrs(subs, 42, rules)).collect();
            let mut n42 = vec![q];
            let mut c42 = 0;
            loop {
                let previous = &n42[c42];
                let newsubs: HashSet<_> = previous.iter()
                    .flat_map(|subs| rule_substrs(subs, 42, rules))
                    .filter(|s| !previous.contains(s))
                    .collect();
                
                if newsubs.is_empty() { break; }

                c42 += 1;
                n42.push(newsubs);
            }

            //add 31
            'out: for (n42, matches) in n42.iter().enumerate() {
                // match at least one 31
                let mut m31: HashSet<_> = matches.iter().flat_map(|subs| rule_substrs(subs, 31, rules)).collect();
                //add more 31, up to the number of 42's matched
                for _ in 0..=n42 {
                    if m31.iter().any(|s| s.is_empty()) {
                        valid += 1;
                        break 'out;
                    }
                    //more macthes for 31
                    let newsubs: HashSet<_> = m31.iter()
                        .flat_map(|subs| rule_substrs(subs, 31, rules))
                        .filter(|s| !m31.contains(s))
                        .collect();
                    if newsubs.is_empty() { break; }
                    m31 = newsubs;
                }
            }
        }
        Ok(format!("{}", valid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let msg = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"#;
        let result = parse(msg);
        println!("{:?}", result);
        let data = result.unwrap().1;
        let valid = data.messages.iter()
            .filter(|m| rule_substrs(m, 0, &data.rules).iter().any(|s| s.len() == 0))
            .count();
        assert_eq!(valid, 2);
    }
}
