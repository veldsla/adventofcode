use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Port {
    a: u8,
    b: u8,
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    AB,
    BA
}

impl Ord for Port {
    fn cmp(&self, other: &Port) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for Port {
    fn partial_cmp(&self, other: &Port) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Port {
    fn strength(&self) -> u32 {
        self.a as u32 + self.b as u32
    }

    fn connects(&self, ori_a: Orientation, other: &Port) -> Option<Orientation> {
        match ori_a {
            Orientation::AB => {
                if self.b == other.a {
                    Some(Orientation::AB)
                } else if self.b == other.b {
                    Some(Orientation::BA)
                } else {
                    None
                }
            },
            Orientation::BA => {
                if self.a == other.a {
                    Some(Orientation::AB)
                } else if self.a == other.b {
                    Some(Orientation::BA)
                } else {
                    None
                }
            },
        }
    }

    fn zero_pin(&self) -> bool {
        self.a == 0 || self.b == 0
    }
}

fn strongest_bridge(ports: &[Port]) -> (u32, u32) {
    let strongest = |a: &(u32,u32), b: &(u32,u32)| a.1.cmp(&b.1);
    build_bridge(ports, &strongest)
}

fn longest_bridge(ports: &[Port]) -> (u32, u32) {
    let longest = |a: &(u32,u32), b: &(u32,u32)| {
        match  a.0.cmp(&b.0) {
            Ordering::Equal => a.1.cmp(&b.1),
            o => o
        }
    };
    build_bridge(ports, &longest)
}

fn build_bridge(ports: &[Port], comp: &Fn(&(u32,u32), &(u32,u32))->Ordering) -> (u32, u32) {
    ports.iter().enumerate()
        .filter(|p| p.1.zero_pin())
        .map(|(pos, port)| {
            let mut remaining = ports.to_vec();
            let last = remaining.remove(pos);
            let ori = match last.a {
                0 => Orientation::AB,
                _ => Orientation::BA
            };
            next_port(&remaining, last, ori, port.strength(), 0, comp)
        })
        .max_by(comp).unwrap()
}

fn next_port(ports: &[Port], lastport: Port,ori: Orientation, sum: u32, l: u32, comp: &Fn(&(u32,u32), &(u32,u32))->Ordering) -> (u32,u32) {
    if let Some(max) = ports.iter().enumerate()
        .filter_map(|(p, port)| {
            match lastport.connects(ori, port) {
                Some(o) => Some((p, o)),
                None => None
            }
        })
        .map(|(p, ori)| {
            let mut remaining = ports.to_vec();
            let last = remaining.remove(p);
            next_port(&remaining, last, ori, sum + last.strength(), l+1, comp)
        })
        .max_by(comp)
    {
        max
    } else {
        (l, sum)
    }
}

fn main() {
    let f = File::open("input.txt").unwrap_or_else(|e| panic!("Error opening file:\n\t{}", e));
    let ports: Vec<Port> = BufReader::new(f).lines().map(|l| {
        let line = l.unwrap_or_else(|_| panic!("Error reading line"));
        let mut v = line.split('/');
        Port {
            a: v.next().unwrap().parse().unwrap(),
            b: v.next().unwrap().parse().unwrap(),
        }
    }).collect();

    let (strongest_l, strongest_s) = strongest_bridge(&ports);
    let (longest_l, longest_s) = longest_bridge(&ports);
    println!("24a: The strongest bridge has strength {} (and length {})", strongest_s, strongest_l);
    println!("24b: The longest bridge has length {} (and strength {})", longest_l, longest_s);
}
