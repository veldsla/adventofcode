use std::cmp::{Ord, Ordering};
use std::collections::HashSet;
use std::fs::File;
use std::fmt;
use std::io::{self, Read};
use std::path::Path;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

fn ioerror<E: fmt::Display>(e: E) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, format!("{}", e))
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Group {
    group_type: Army,
    units: u32,
    hp: u32,
    immune_to: Vec<AttackType>,
    weak_to: Vec<AttackType>,
    attack_type: AttackType,
    damage: u32,
    initiative: u32
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum AttackType {
    Bludgeoning,
    Cold,
    Fire,
    Raditation,
    Slashing,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Army {
    Immune,
    Infection
}

#[derive(Debug, Clone)]
struct Body(Vec<Group>);

impl FromStr for AttackType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bludgeoning" => Ok(AttackType::Bludgeoning),
            "cold" => Ok(AttackType::Cold),
            "fire" => Ok(AttackType::Fire),
            "radiation" => Ok(AttackType::Raditation),
            "slashing" => Ok(AttackType::Slashing),
            _ => Err("Uknown attack type")

        }
    }
}

impl FromStr for Group {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref GM: Regex = Regex::new(r"(?P<units>\d+) units each with (?P<hp>\d+) hit points (?:\((?P<wi>.*?)\) )?with an attack that does (?P<at>\d+) (?P<att>\w+) damage at initiative (?P<in>\d+)").unwrap();
            static ref WEAK: Regex = Regex::new(r"weak to (?P<wi>(.*?))(;|$)").unwrap();
            static ref IMMUNE: Regex = Regex::new(r"immune to (?P<wi>(.*?))(;|$)").unwrap();
        }

        let caps = GM.captures(s).ok_or("Cannot parse group line")?;

        let units = caps.name("units").ok_or("Cannot parse unit in group line")
            .and_then(|s| s.as_str().parse().map_err(|_| "units not a number"))?;
        let hp = caps.name("hp").ok_or("Cannot parse hp in group line")
            .and_then(|s| s.as_str().parse().map_err(|_| "hp not a number"))?;
        let damage = caps.name("at").ok_or("Cannot parse attack damage in  group line")
            .and_then(|s| s.as_str().parse().map_err(|_| "attack damage not a number"))?;
        let initiative = caps.name("in").ok_or("Cannot parse initiative in group line")
            .and_then(|s| s.as_str().parse().map_err(|_| "initiative not a number"))?;
        let attack_type = caps.name("att").ok_or("Cannot parse attacktype in group line")
            .and_then(|s| s.as_str().parse().map_err(|_| "Unknown attack type"))?;

        let weak_to = if let Some(wi) = caps.name("wi") {
            if let Some(wk) = WEAK.captures(wi.as_str()) {
                let weaknesses = wk.name("wi").ok_or("no weaknesses?")?;
                weaknesses.as_str().split(", ")
                     .map(|s| s.parse().map_err(|_| "Unknown attack type in weak to"))
                     .collect::<Result<Vec<AttackType>, _>>()?
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        let immune_to = if let Some(im) = caps.name("wi") {
            if let Some(im) = IMMUNE.captures(im.as_str()) {
                let immunities = im.name("wi").ok_or("no immunities?")?;
                immunities.as_str().split(", ")
                     .map(|s| s.parse().map_err(|_| "Unknown attack type in weak to"))
                     .collect::<Result<Vec<AttackType>, _>>()?
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
       

        Ok(Group { group_type: Army::Immune, units, hp, immune_to, weak_to, attack_type, damage, initiative })
    }
}

impl fmt::Display for Army {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Army::Immune => "Immune system",
            Army::Infection => "Infection",
        })
    }
}

impl Ord for Group {
    fn cmp(&self, other: &Group) -> Ordering {
        self.eff_power().cmp(&other.eff_power()).then(self.initiative.cmp(&other.initiative))
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Group) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Body {
    fn from_file<P: AsRef<Path>>(p: P) -> io::Result<Body> {
        let mut f = File::open(p)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        let mut parts = s.split("\n\n");
        let mut immune_lines = parts.next().ok_or_else(|| ioerror("invalid immune data")).and_then(|s| Ok(s.split('\n')))?;
        if immune_lines.next() != Some("Immune System:") {
            return Err(ioerror("No immune system in file"));
        }
        let mut groups = immune_lines.map(|l| l.parse().map_err(|e| ioerror(e)))
            .collect::<Result<Vec<Group>, _>>()?;

        let mut infection_lines = parts.next().ok_or_else(|| ioerror("invalid infection data")).and_then(|s| Ok(s.split_terminator('\n')))?;
        if infection_lines.next() != Some("Infection:") {
            return Err(ioerror("No infection in file"));
        }
        groups.extend(infection_lines.map(|l| l.parse().map_err(|e| ioerror(e)).and_then(|mut g: Group| { g.group_type = Army::Infection; Ok(g) }))
            .collect::<Result<Vec<Group>, _>>()?);

        Ok(Body(groups))
    }

    fn prep_fights(&mut self) -> Vec<(u32, usize, usize)> {
        // target selection
        let mut groups: Vec<_> = self.0.iter().enumerate().collect();
        groups.sort_by_key(|e| e.1);

        let mut targeted = HashSet::new();
        let mut fights: Vec<_> = groups.iter().rev()
            .filter(|(_, at)| at.alive())
            .filter_map(|(ai, at)| {
                //find defender
                let defender = groups.iter()
                    .filter(|g| g.1.alive() && g.1.is_enemy(at) && !targeted.contains(&g.0))
                    .map(|(i, d)| (d.damage(at), d.eff_power(), d.initiative, i))
                    .filter(|t| t.0 > 0)
                    .max();
                if let Some(defender) = defender {
                    targeted.insert(*defender.3);
                    Some((at.initiative, *ai, *defender.3))
                } else {
                    None
                }
            }).collect();
        fights.sort_by(|a, b| a.0.cmp(&b.0).reverse());
        fights
    }

    fn fight(&mut self) {
        loop {
            let mut damage_dealt = false;
            let fights = self.prep_fights();
            if fights.is_empty() {
                break;
            }
            for (_, attack, defend) in fights {
                if self.0[attack].alive() && self.0[defend].alive() {
                    let damage = self.0[defend].damage(&self.0[attack]);
                    let losses = self.0[defend].receive_damage(damage);
                    if losses {damage_dealt = true}
                }
            }
            if !damage_dealt {
                //deadlock
                break;
            }
        }
    }

    fn boost(&mut self, n: u32) {
        self.0.iter_mut()
            .filter(|g| g.group_type == Army::Immune)
            .for_each(|g| g.damage += n);
    }

    fn winner(&self) -> Option<Army> {
        if self.0.iter().filter(|g| g.alive()).all(|g| g.group_type == Army::Immune) {
            Some(Army::Immune)
        } else if self.0.iter().filter(|g| g.alive()).all(|g| g.group_type == Army::Infection) {
            Some(Army::Infection)
        } else {
            None
        }
    }

    fn sum_alive(&self) -> u32 {
        self.0.iter().map(|g| g.units).sum()
    }
}

impl Group {
    fn eff_power(&self) -> u32 {
         self.units * self.damage
    }

    /// How much damage will be dealt by other
    fn damage(&self, other: &Group) -> u32 {
        if self.immune_to.contains(&other.attack_type) {
            0
        } else {
            let factor = if self.weak_to.contains(&other.attack_type) { 2 } else { 1 };
            other.eff_power() * factor
        }
    }

    fn receive_damage(&mut self, damage: u32) -> bool {
        self.units = self.units.saturating_sub(damage / self.hp);
        damage / self.hp > 0
    }

    fn is_enemy(&self, other: &Group) -> bool {
        match self.group_type {
            Army::Infection => other.group_type == Army::Immune,
            Army::Immune => other.group_type == Army::Infection,
        }
    }

    fn alive(&self) -> bool {
        self.units > 0
    }
}


fn main() -> io::Result<()> {
    let body = Body::from_file("input.txt")?;
    let mut body_1 = body.clone();
    body_1.fight();
    println!("24a: fight over winner is {} sum units remaining {}", body_1.winner().unwrap(), body_1.sum_alive());
    for boost in 1.. {
        let mut body_2 = body.clone();
        body_2.boost(boost);
        body_2.fight();
        if body_2.winner() == Some(Army::Immune) {
            println!("24b: Fight won with immune system boosted with {} sum units remaining {}", boost, body_2.sum_alive());
            break;
        }
    }
    Ok(())
}

#[test]
fn test() {
    let mut body = Body::from_file("test.txt").unwrap();
    body.fight();
    assert_eq!(body.sum_alive(), 5216);
}
#[test]
fn test2() {
    let mut body = Body::from_file("test.txt").unwrap();
    body.boost(1570);
    body.fight();
    assert_eq!(body.sum_alive(), 51);
}
