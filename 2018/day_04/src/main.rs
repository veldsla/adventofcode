use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::str::FromStr;

struct Sleep {
    from: u8,
    to: u8
}

struct Duty(Vec<Sleep>);

struct GuardShift {
    guards: HashMap<u32, Duty>
}

impl Duty {
    fn max_slept_minute(&self) -> (u8, u32) {
        let mut m = [0u32; 60];
        for s in &self.0 {
            for t in s.from..s.to {
                m[t as usize] += 1;
            }
        }
        let max = m.iter().enumerate().max_by_key(|e| e.1).expect("Guard without sleep?");
        (max.0 as u8, *max.1)
    }

    fn slept(&self) -> u32 {
        self.0.iter()
            .map(|s| (s.to - s.from) as u32)
            .sum()
    }
}

impl GuardShift {
    // The horror
    fn from_file<P: AsRef<Path>>(p: P) -> io::Result<GuardShift> {
        let f = File::open(p)?;
        let b = BufReader::new(f);
        let mut lines = b.lines().collect::<Result<Vec<String>, _>>()?;
        //sort lines to group events
        lines.sort();

        let mut guards = HashMap::new();

        let mut ei = lines.iter().peekable();
        //the first line is a begin shift line:
        //then any number of fall asleep followed by wake up
        while let Some(guardline) = ei.next() {
            //parse the ID...Forgive me
            let id = guardline.split(|c| c== ' ' || c == '#').nth(4).unwrap().parse::<u32>().unwrap();
            let duty = guards.entry(id).or_insert(Duty(Vec::new()));
            loop {
                //peek at next line to see if we are done
                match ei.peek() {
                    Some(s) if s.ends_with("begins shift") => break,
                    None => break,
                    _ => {},
                }
                let sleep = ei.next().expect("data error");
                let from = sleep.split(|c| c == ':' || c == ']').nth(1).unwrap().parse::<u8>().unwrap();
                let wake = ei.next().expect("data error");
                let to = wake.split(|c| c == ':' || c == ']').nth(1).unwrap().parse::<u8>().unwrap();
                duty.0.push( Sleep { from, to} );
            }
        } 

        Ok(GuardShift {guards })
        
    }

    //find the guard that sleeps the most
    fn laziest_guard(&self) -> u32 {
        let lazy = self.guards.iter().max_by(|a, b| {
            a.1.slept().cmp(&b.1.slept())
        }).expect("No guards?");
        *lazy.0
    }

    fn most_slept_minute(&self) -> (u32, u8) {
        let g = self.guards.iter().max_by(|a, b| {
            a.1.max_slept_minute().1.cmp(&b.1.max_slept_minute().1)
        }).expect("No guards");
        let stats = g.1.max_slept_minute();
        (*g.0, stats.0)
    }
}

fn main() -> io::Result<()> {
    let gs = GuardShift::from_file("input.txt")?;
    let laziest = gs.laziest_guard();
    let slept = gs.guards[&laziest].max_slept_minute();
    println!("laziest guard: {}, slept {} times at minute {}.", laziest, slept.1, slept.0);
    println!("4a: {}", laziest * slept.0 as u32);

    let mfa = gs.most_slept_minute();
    println!("Guard {} was most frequently asleep at {}.", mfa.0, mfa.1);
    println!("4b: {}", mfa.0 * mfa.1 as u32);

    Ok(())
}
