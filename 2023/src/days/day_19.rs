use std::collections::HashMap;
use std::ops::RangeInclusive;
use anyhow::{anyhow, Result};
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, one_of},
    combinator::{map, opt, peek},
    multi::{fold_many1, many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
};

use crate::parsers::positive_integer;
use crate::Problem;

#[derive(Default)]
pub struct Solution {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

#[derive(Copy, Clone, Debug, Default)]
struct Part {
    values: [u32; 4],
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum RatingType {
    Cool = 0,
    Musical = 1,
    Aerodynamic = 2,
    Shiny = 3,
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    default_target: String,
}

#[derive(Debug)]
struct Rule {
    rating_type: RatingType,
    condition: Condition,
    target_workflow: String,
}

#[derive(Debug)]
enum Condition {
    LessThan(u32),
    GreaterThan(u32),
}

#[derive(Clone, Debug)]
struct RangeSet {
    target: String,
    value_ranges: [RangeInclusive<u32>; 4],
}

impl Workflow {
    fn get_target(&self, part: &Part) -> &str {
        for rule in &self.rules {
            match rule.condition {
                Condition::LessThan(value) => {
                    if part.values[rule.rating_type as usize] < value {
                        return &rule.target_workflow;
                    }
                },
                Condition::GreaterThan(value) => {
                    if part.values[rule.rating_type as usize] > value {
                        return &rule.target_workflow;
                    }
                },
            }
        }
        &self.default_target
    }
}

impl RangeSet {
    fn apply_rule(&self, rule: &Rule) -> (Option<RangeSet>, Option<RangeSet>) {
        match rule.condition {
            Condition::LessThan(value) => {
                if *self.value_ranges[rule.rating_type as usize].end() < value {
                    // all values are less than the rule limit
                    let mut m = self.clone();
                    m.target = rule.target_workflow.clone();
                    (Some(m), None)
                } else if *self.value_ranges[rule.rating_type as usize].start() >= value {
                    // all values are greater than the rule limit
                    (None, Some(self.clone()))
                } else {
                    // split the range into matching and non-matching
                    let mut m = self.clone();
                    m.target = rule.target_workflow.clone();
                    m.value_ranges[rule.rating_type as usize] = *self.value_ranges[rule.rating_type as usize].start()..=value-1;

                    let mut nm = self.clone();
                    nm.value_ranges[rule.rating_type as usize] = value..=*self.value_ranges[rule.rating_type as usize].end();

                    (Some(m), Some(nm))
                }
            },
            Condition::GreaterThan(value) => {
                if *self.value_ranges[rule.rating_type as usize].start() > value {
                    // all values are greater than the rule limit
                    let mut m = self.clone();
                    m.target = rule.target_workflow.clone();
                    (Some(m), None)
                } else if *self.value_ranges[rule.rating_type as usize].end() <= value {
                    // all values are less than the rule limit, no hit
                    (None, Some(self.clone()))
                } else {
                    // split the range into matching and non-matching
                    let mut m = self.clone();
                    m.target = rule.target_workflow.clone();
                    m.value_ranges[rule.rating_type as usize] = value+1..=*self.value_ranges[rule.rating_type as usize].end();

                    let mut nm = self.clone();
                    nm.value_ranges[rule.rating_type as usize] = *self.value_ranges[rule.rating_type as usize].start()..=value;

                    (Some(m), Some(nm))
                }
            },
        }
    }

    fn sum(&self) -> u64 {
        self.value_ranges.iter().map(|r| *r.end() as u64 - *r.start() as u64 + 1).product()
    }
}

impl Part {
    fn sum(&self) -> u32 {
        self.values.iter().sum()
    }
}

fn parse_rule(s: &str) -> IResult<&str, Rule> {
    // peek the rating type and the operator togeter to avoid parsing part of the
    // default workflow name
    peek(tuple((one_of("xmas"), one_of("><"))))(s)?;
    map(tuple((one_of("xmas"), one_of("><"), positive_integer, preceded(tag(":"), alpha1))),
    |(rating_type, condition, value, destination)| {
        Rule {
            rating_type: match rating_type {
                'x' => RatingType::Cool,
                'm' => RatingType::Musical,
                'a' => RatingType::Aerodynamic,
                's' => RatingType::Shiny,
                _ => panic!("Invalid part type"),
            },
            condition: match condition {
                '<' => Condition::LessThan(value),
                '>' => Condition::GreaterThan(value),
                _ => panic!("Invalid condition"),
            },
            target_workflow: destination.to_string(),
        }
    })(s)
}

fn parse_workflow(s: &str) -> IResult<&str, (String, Workflow)> {
    let rules = separated_list1(tag(","), parse_rule);
    let default_target = preceded(tag(","), alpha1);
    map(terminated(tuple((alpha1, delimited(tag("{"), tuple((rules, default_target)), tag("}")))), line_ending),
        |(name, (rules, default_target))| (name.to_string(), Workflow { rules, default_target: default_target.to_string() }))(s)
}

fn parse_parts(s: &str) -> IResult<&str, Part> {
    let parse_part = fold_many1(terminated(separated_pair(one_of("xmas"), tag("="), positive_integer), opt(tag(","))),
    Part::default,
    |mut part, (part_type, value)| 
    {
        match part_type {
            'x' => part.values[RatingType::Cool as usize] = value,
            'm' => part.values[RatingType::Musical as usize] = value,
            'a' => part.values[RatingType::Aerodynamic as usize] = value,
            's' => part.values[RatingType::Shiny as usize] = value,
            _ => panic!("Invalid part type"),
        }
        part
    });

    delimited(tag("{"),  parse_part, tag("}"))(s)
}

impl Problem for Solution {
    fn parse(&mut self, s: &str) -> Result<()> {
        let (s, workflows) = many1(parse_workflow)(s)
            .map_err(|e| anyhow!("Parser error {e}"))?;
        let (_s, parts) = preceded(line_ending, separated_list1(line_ending, parse_parts))(s)
            .map_err(|e| anyhow!("Parser error {e}"))?;
        self.workflows = workflows.into_iter().collect();
        self.parts = parts;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        // count accepted parts
        Ok(self.parts.iter().filter_map(|part| {
            let mut workflow = self.workflows.get("in").unwrap();
            loop {
                let target = workflow.get_target(part);
                if target == "A" {
                    return Some(part);
                } else if target == "R" {
                    return None;
                } else {
                    workflow = self.workflows.get(target).unwrap();
                }   
            }
        }).map(|p| p.sum()).sum::<u32>().to_string())
    }

    fn part2(&self) -> Result<String> {

        let start = RangeSet {
            target: "in".to_owned(),
            value_ranges: [1..=4000, 1..=4000, 1..=4000, 1..=4000],
        };

        let mut queue = vec![start];
        let mut accepted = Vec::new();
        let mut rejected = Vec::new();

        let mut test = Vec::new();
        let mut testnextrule = Vec::new();

        while let Some(start) = queue.pop() {
            if start.target == "A" {
                accepted.push(start);
                continue;
            } else if start.target == "R" {
                rejected.push(start);
                continue;
            }
            let workflow = self.workflows.get(&start.target).unwrap();
            test.push(start);
            for rule in &workflow.rules {
                while let Some(range) = test.pop() {
                    match range.apply_rule(rule) {
                        (Some(accepted), Some(rejected)) => {
                            queue.push(accepted);
                            testnextrule.push(rejected);
                        },
                        (Some(accepted), None) => {
                            queue.push(accepted);
                        },
                        (None, Some(rejected)) => {
                            testnextrule.push(rejected);
                        },
                        (None, None) => {
                            panic!("impossible")
                        },
                    }
                }
                std::mem::swap(&mut test, &mut testnextrule);
            }
            // add all the failed ranges to the default target
            queue.extend(test.drain(..).map(|mut r| {
                r.target = workflow.default_target.clone();
                r
            }));
        }

        Ok(accepted.iter().map(|r| r.sum()).sum::<u64>().to_string())
    }
}

