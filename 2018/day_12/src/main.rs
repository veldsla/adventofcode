use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;


const POWS: [usize; 5] = [1, 2, 4, 8, 16];

#[derive(Debug)]
struct Pots {
    pots: Vec<bool>,
    zero: usize,
    rules: Vec<bool>
}

fn rule_from_str(s: &str) -> (usize, bool) {
    let p: Vec<_> = s.split(" => ").collect();
    //interpret the #.#... pattern as binary
    let mut num = 0;
    for (bit, value) in p[0].chars().rev().enumerate() {
        if value == '#' {
            num |= POWS[bit];
        }
    }
    (num, p[1] == "#")
}

fn bools_to_num(v: &[bool]) -> usize {
    let mut num = 0;
    for (pos, _) in v.iter().rev().enumerate().filter(|b| *b.1) {
        num |= POWS[pos];
    }
    num
}

impl fmt::Display for Pots {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}:{}",
               std::iter::repeat(' ').take(20-self.zero).collect::<String>(),
               self.pots.iter().map(|&p| if p { '#' } else { '.' }).collect::<String>(),
               self.plant_number_sum()
               )
    }
}

impl Pots {
    fn from_file<P: AsRef<Path>>(p: P) -> io::Result<Pots> {
        let f = File::open(p)?;
        let b = BufReader::new(f);
        let mut lines = b.lines();
        // initial field
        let header = lines.next()
            .and_then(|l| l.ok()).ok_or(io::Error::new(io::ErrorKind::InvalidData, "cannot parse header"))?;
        let pots = header.trim().chars().skip(15).map(|c| c == '#').collect();

        // empty line
        let _ = lines.next().ok_or(io::Error::new(io::ErrorKind::InvalidData, "cannot parse header"))?;

        // rules
        let mut rules = vec![false; 32];
        for l in lines {
            let l = l?;
            let (num, grow) = rule_from_str(&l);
            rules[num] = grow;
        }

        Ok(Pots { pots, rules, zero: 0 })
    }

    fn evolve(&mut self) {
        let mut v = Vec::new();
        //the head can yield extra plants if a suitable rule exists
        if self.rules[bools_to_num(&self.pots[0..1])] {
            v.push(true);
            self.zero += 2;
            if self.rules[bools_to_num(&self.pots[0..2])] {
                v.push(true);
            }
        } else if self.rules[bools_to_num(&self.pots[0..2])] {
            v.push(true);
            self.zero += 1;
        }

        //the non extending head
        v.extend((3..5).map(|p| {
            self.rules[bools_to_num(&self.pots[0..p])]
        }));

        // now walk the windows
        v.extend(self.pots.windows(5)
            .map(bools_to_num)
            .map(|i| self.rules[i])
            );

        // and check the tail values
        let len = self.pots.len();
        for t in (1..=4).rev() {
            if self.rules[bools_to_num(&self.pots[len-t..]) << (5-t)] {
                v.push(true);
            } else if t > 2 {
                v.push(false);
            }
        }
        std::mem::replace(&mut self.pots, v);
    }

    fn plant_number_sum(&self) -> isize {
        self.pots.iter()
            .enumerate()
            .filter(|p| *p.1)
            .map(|p| p.0 as isize - self.zero as isize).sum()
    }
}

fn main() {
    let mut p = Pots::from_file("input.txt").unwrap();
    for _generation in 0..20 {
        p.evolve();
        println!("{}", p);
    }
    println!("12a: sum of numbers of pots with plants: {}", p.plant_number_sum());

    let mut p = Pots::from_file("input.txt").unwrap();
    let mut seen = HashMap::new();
    let mut gen = 0u64;
    let target_gen = 50_000_000_000u64;
    loop {
        p.evolve();
        gen += 1;
        // extract the left trimmed pot sequence and store it
        // remember the offset and the generation
        if let Some(offset) = p.pots.iter().position(|p| *p) {
            if let Some((p_offset, p_gen)) = seen.insert(p.pots[offset..].to_vec(), (offset, gen)) {
                println!("pattern detected from generation {} to {}, offset was {}, is now {}", p_gen, gen, p_offset, offset);
                if (target_gen - gen ) % (gen - p_gen) == 0 {
                    //this repeat brings us to the target generation
                    let n_steps = (target_gen - gen ) / (gen - p_gen);
                    //get the pot numbers for this generation
                    //shift them by the number of steps as sum it
                    let sum: u64 = p.pots.iter().enumerate().filter(|p| *p.1).map(|p| p.0 as u64 + n_steps).sum();
                    println!("12b: The pot number sums after {} generations is {}", target_gen, sum); 
                    break;
                }
            }
        }
    }
}

#[test]
fn test() {
    let mut p = Pots::from_file("test.txt").unwrap();
    println!("{:?}", p);
    println!("{}", p);
    for _ in 0..20 {
    p.evolve();
    println!("{}", p);
    }
    assert_eq!(p.zero, 2);
    assert_eq!(p.pots.len(), 37);
    assert_eq!(p.plant_number_sum(), 325);
}
