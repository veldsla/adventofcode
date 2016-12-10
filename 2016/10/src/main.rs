use std::collections::HashMap;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Bot {
    id: u8,
    chip_1: Option<Chip>,
    chip_2: Option<Chip>
}

type Chip = u8;

impl Bot {
    fn new(id: u8) ->Bot {
        Bot {id: id, chip_1: None, chip_2: None}
    }

    fn can_give(&self) -> bool {
        self.chip_1.is_some() && self.chip_2.is_some()
    }

    fn can_receive(&self) -> bool {
        self.chip_1.is_none() || self.chip_2.is_none()
    }

    fn receive_value(&mut self, v: Chip) -> Result<(), String> {
        if self.chip_1.is_none() {
            self.chip_1 = Some(v);
            Ok(())
        } else if self.chip_2.is_none() {
            self.chip_2 = Some(v);
            Ok(())
        } else {
            Err("Assigning to full bot".to_string())
        }
    }

    fn take(&mut self) -> Result<(Chip, Chip), String> {
        if self.can_give() {
            println!("{} is comparing {} to {}", self.id, self.chip_1.unwrap(),self.chip_2.unwrap()); 
             match self.chip_1.unwrap().cmp(&self.chip_2.unwrap()) {
                Ordering::Less => Ok((self.chip_1.take().unwrap(), self.chip_2.take().unwrap())),
                Ordering::Greater | Ordering::Equal => Ok((self.chip_2.take().unwrap(), self.chip_1.take().unwrap()))
            }
        } else {
            Err("Cannot give".to_string())
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    AssignChip { value: Chip,  bot: u8 },
    GiveTo {from_bot: u8, low: To, high: To }
}

impl From<String> for Instruction {
    fn from(s: String) -> Instruction {
        let p = s.split(" ").collect::<Vec<&str>>();
        match p[0] {
            "value" => {
                let chip: Chip = p[1].parse().unwrap();
                let bot: u8 = p[5].parse().unwrap();
                Instruction::AssignChip {value: chip, bot: bot}
            },
            "bot" => {
                let botid: u8 = p[1].parse().unwrap();
                assert_eq!(p[3], "low");
                Instruction::GiveTo {from_bot: botid, low: p[5..7].into() , high: p[10..12].into()}
            },
            _ => panic!("error parsing instructions")
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum To {
    Output(u8),
    Bot(u8)
}

impl<'a> From<&'a [&'a str]> for To {
    fn from(p: &[&str]) -> To {
        match p[0] {
            "bot" => To::Bot(p[1].parse().unwrap()),
            "output" => To::Output(p[1].parse().unwrap()),
            _ => panic!("error parsing instructions")
        }
    }
}

#[derive(Debug)]
struct Output {
    id: u8,
    values: Vec<Chip>
}

impl Output {
    fn new(id: u8) -> Output {
        Output { id: id, values: Vec::new() }
    }
}

#[derive(Debug)]
struct Factory {
    bots: HashMap<u8,Bot>,
    outputs: HashMap<u8, Output>,
    assignments: Vec<Assignment>
}

#[derive(Debug)]
struct Assignment {
    from_bot: u8,
    low: To,
    high: To
}

impl Factory {
    fn new() -> Factory {
        Factory { bots: HashMap::new(), outputs: HashMap::new(), assignments: Vec::new() }
    }

    fn add_instruction<S: Into<Instruction>>(&mut self, s: S) {
        let instruction = s.into();
        match instruction {
            Instruction::AssignChip { value: c, bot: b } => {
                let bot = self.bots.entry(b).or_insert(Bot::new(b));
                bot.receive_value(c).unwrap();
            },
            Instruction::GiveTo {from_bot: b, low: low, high: high } => {
                self.assignments.push(Assignment { from_bot: b, low: low.clone(), high: high.clone() });
                if !self.bots.contains_key(&b) {
                    self.bots.insert(b, Bot::new(b));
                }

                for to in vec![low, high].into_iter() {
                    match to {
                        To::Output(n) => {
                            if !self.outputs.contains_key(&n) {
                                self.outputs.insert(n, Output::new(n));
                            }
                        },
                        To::Bot(n) => {
                            if !self.bots.contains_key(&n) {
                                self.bots.insert(n, Bot::new(n));
                            }
                        }
                    }
                }
            }
        }
    }

    fn run(&mut self) {
        loop {
            let mut stable = true;
            for i in &self.assignments {
                //bot can give?
                if let Some(bot) = self.bots.get(&i.from_bot) {
                    if !bot.can_give() {
                        continue;
                    }
                } else {
                    panic!("Assignment for unknown bot");
                }

                //make sure To's can receive
                for to in [i.low, i.high].iter() {
                    match to {
                        &To::Output(n) => {},
                        &To::Bot(n) => {
                            if !self.bots.get(&n).unwrap().can_receive() {
                                continue;
                            }
                        }
                    }
                }

                //proceed
                let (low, high)= self.bots.get_mut(&i.from_bot).unwrap().take().unwrap();

                match i.low {
                    To::Output(n) => { 
                        self.outputs.get_mut(&n).unwrap().values.push(low);
                    },
                    To::Bot(n) => {
                        self.bots.get_mut(&n).unwrap().receive_value(low);
                    }
                }

                match i.high {
                    To::Output(n) => { 
                        self.outputs.get_mut(&n).unwrap().values.push(high)
                    },
                    To::Bot(n) => {
                        self.bots.get_mut(&n).unwrap().receive_value(high);
                    }
                }
                stable = false;
            }

            if stable {
                break;
            }
        }
    }
}


fn main() {
    let f = File::open("input.txt").unwrap();
    let mut factory = Factory::new();
    for l in BufReader::new(f).lines() {
        factory.add_instruction(l.unwrap());
    }
    factory.run();

    let prd = (0..3).map(|i| factory.outputs.get(&i).unwrap().values[0] as u32).fold(1, |acc, x| acc * x);
    println!("product of output 0 / 1 / 2 = {}", prd);

}

#[test]
fn test() {
    let input = "\
value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

    let mut f = Factory::new();
    for l in input.lines() {
        f.add_instruction(l.to_string());
    }
    f.run();
    println!("{:?}", f);
    assert!(false);

}
