use std::collections::{HashMap, VecDeque};
use anyhow::{anyhow, Result};
use nom::{
    IResult,
    bytes::complete::tag,
    branch::alt,
    character::complete::{alpha1, line_ending, one_of},
    combinator::{map, opt, peek},
    multi::{fold_many1, many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
};


use crate::Problem;

#[derive(Default)]
pub struct Solution {
    modules: HashMap<String, Module>,
}

#[derive(Clone, Debug)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster(Broadcaster),
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
enum Pulse {
    #[default]
    Low,
    High,
}

#[derive(Clone, Copy, Debug, Default)]
enum State {
    #[default]
    Off,
    On,
}

#[derive(Clone, Debug, Default)]
struct FlipFlop {
    state: State,
    outputs: Vec<String>,
}

#[derive(Clone, Debug, Default)]
struct Conjunction {
    input_states: HashMap<String, Pulse>,
    outputs: Vec<String>,
}

#[derive(Clone, Debug, Default)]
struct Broadcaster {
    outputs: Vec<String>,
}

impl State {
    fn toggle(&mut self) {
        match self {
            State::Off => *self = State::On,
            State::On => *self = State::Off,
        }
    }
}

impl FlipFlop {
    fn pulse(&mut self, input: Pulse) -> Option<Pulse> {
        match input {
            Pulse::Low => {
                self.state.toggle();
                match self.state {
                    State::Off => Some(Pulse::Low),
                    State::On => Some(Pulse::High),
                }
            }
            Pulse::High => None,
        }
    }
}

impl Conjunction {
    fn pulse(&mut self, input: Pulse, from: String) -> Option<Pulse> {
        //remember input state
        self.input_states.insert(from, input);
        // check all memory stater are high
        let all_high = self.input_states.iter().all(|(_, state)| *state == Pulse::High);

        if all_high {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }
}

impl Broadcaster {
    fn pulse(&mut self) -> Option<Pulse> {
        Some(Pulse::Low)
    }
}

impl Module {
    fn pulse(&mut self, input: Pulse, from: String) -> Option<Pulse> {
        match self {
            Module::FlipFlop(flip_flop) => flip_flop.pulse(input),
            Module::Conjunction(conjunction) => conjunction.pulse(input, from),
            Module::Broadcaster(broadcaster) => broadcaster.pulse(),
        }
    }

    fn outputs(&self) -> impl Iterator<Item = &String> {
        match self {
            Module::FlipFlop(flip_flop) => flip_flop.outputs.iter(),
            Module::Conjunction(conjunction) => conjunction.outputs.iter(),
            Module::Broadcaster(broadcaster) => broadcaster.outputs.iter(),
        }
    }
}


fn parse_module(s: &str) -> IResult<&str, (String, Module)> {
    let broacaster = map(separated_pair(tag("broadcaster"), tag(" -> "), separated_list1(tag(", "), alpha1)),
    |(name, outputs): (&str, Vec<&str>)| {
        let mut broadcaster = Broadcaster::default();
        broadcaster.outputs = outputs.iter().map(|s| s.to_string()).collect();
        (name.to_owned(), Module::Broadcaster(broadcaster))
    });

    let flip_flop = map(separated_pair(preceded(tag("%"), alpha1), tag(" -> "), separated_list1(tag(", "), alpha1)),
    |(name, outputs): (&str, Vec<&str>)| {
        let mut flip_flop = FlipFlop::default();
        flip_flop.outputs = outputs.iter().map(|s| s.to_string()).collect();
        (name.to_owned(), Module::FlipFlop(flip_flop))
    });

    let conjunction = map(separated_pair(preceded(tag("&"), alpha1), tag(" -> "), separated_list1(tag(", "), alpha1)),
    |(name, outputs): (&str, Vec<&str>)| {
        let mut conjunction = Conjunction::default();
        
        conjunction.outputs = outputs.iter().map(|s| s.to_string()).collect();
        (name.to_owned(), Module::Conjunction(conjunction))
    });


    alt((broacaster, flip_flop, conjunction))(s)
}


impl Problem for Solution {
    fn parse(&mut self, s: &str) -> Result<()> {
        let (_s, modules) = fold_many1(
            terminated(parse_module, line_ending),
            HashMap::new,
            |mut modules, (name, module)| {
                modules.insert(name, module);
                modules
            },
        )(s).map_err(|e| anyhow!("parse error: {:?}", e))?;

        self.modules = modules;

        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let mut modules = self.modules.clone();

        // initialize the inputs for the conjunctions
        let connect_to_conjunctions = modules.iter()
            .map(|(name, module)| {
                module.outputs().filter_map(|output| {
                    if let Some(Module::Conjunction(_conjunction)) = modules.get(output) {
                        Some((name.clone(), output.clone()))
                    } else {
                        None
                    }
                })
            }).flatten().collect::<Vec<_>>();

        connect_to_conjunctions.iter().for_each(|(name, conj)| {
            if let Some(Module::Conjunction(ref mut conjunction)) = modules.get_mut(conj) {
                conjunction.input_states.insert(name.clone(), Pulse::Low);
            }
        });



        let mut queue = VecDeque::new();
        let mut pulse_counts = [0,0];

        for _ in 0..1000 {
            // push the broadcaster button
            queue.push_back(("button".to_owned(), "broadcaster".to_owned(), Pulse::default()));
            // count the button as a low `pulse
            pulse_counts[Pulse::Low as usize] += 1;
            while let Some((from, name, input)) = queue.pop_front() {
                if !modules.contains_key(&name) {
                    // just count the pulses
                    //pulse_counts[input as usize] += 1;
                } else if let Some(pulse) = modules.get_mut(&name).unwrap().pulse(input, from.clone()) {
                    // count the pulses

                    // get the oututs and append to the queue
                    for output in modules.get(&name).unwrap().outputs() {
                        pulse_counts[pulse as usize] += 1;
                        queue.push_back((name.to_owned(), output.to_owned(), pulse));
                    }
                }
            }
        }

        Ok(pulse_counts.iter().product::<u64>().to_string())

    }

   fn part2(&self) -> Result<String> {
        let mut modules = self.modules.clone();

        // initialize the inputs for the conjunctions
        let connect_to_conjunctions = modules.iter()
            .map(|(name, module)| {
                module.outputs().filter_map(|output| {
                    if let Some(Module::Conjunction(_conjunction)) = modules.get(output) {
                        Some((name.clone(), output.clone()))
                    } else {
                        None
                    }
                })
            }).flatten().collect::<Vec<_>>();

        connect_to_conjunctions.iter().for_each(|(name, conj)| {
            if let Some(Module::Conjunction(ref mut conjunction)) = modules.get_mut(conj) {
                conjunction.input_states.insert(name.clone(), Pulse::Low);
            }
        });

        let mut queue = VecDeque::new();

        // find the repearing occurernces of
        // &mf -> rx
        //
        // &bh -> mf
        // &sh -> mf
        // &mz -> mf
        // &jf -> mf
        
        let mut it = 0usize;
        let mut rep = [0; 4];

        'outer: loop {
            it += 1;
            queue.push_back(("button".to_owned(), "broadcaster".to_owned(), Pulse::default()));
            while let Some((from, name, input)) = queue.pop_front() {
                if name == "rx" && input == Pulse::Low {
                    break 'outer;
                }
                if !modules.contains_key(&name) {
                    // just count the pulses
                    //pulse_counts[input as usize] += 1;
                } else if let Some(pulse) = modules.get_mut(&name).unwrap().pulse(input, from.clone()) {
                    // count the pulses
                    if name == "bh" && pulse == Pulse::High {
                        rep[0] = it;
                    }
                    if name == "sh" && pulse == Pulse::High {
                        rep[1] = it;
                    }
                    if name == "mz" && pulse == Pulse::High {
                        rep[2] = it;
                    }
                    if name == "jf" && pulse == Pulse::High {
                        rep[3] = it;
                    }

                    if rep.iter().all(|&r| r > 0) {
                        break 'outer;
                    }

                    // get the oututs and append to the queue
                    for output in modules.get(&name).unwrap().outputs() {
                        queue.push_back((name.to_owned(), output.to_owned(), pulse));
                    }
                }
            }
        }

        Ok(rep.iter().skip(1).fold(rep[0], |acc, &v| num::integer::lcm(acc, v)).to_string())

   }
}

