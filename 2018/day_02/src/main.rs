use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct BoxId(String);

impl BoxId {
    fn dist(&self, other: &BoxId) -> usize {
        self.0.len() - self.0.chars()
            .zip(other.0.chars())
            .filter(|(a, b)| a == b)
            .count()
    }

    fn has_letter_dup_trip(&self) -> (bool,bool) {
        let counts = self.count_letters();
        (
            counts.values().any(|&v| v == 2),
            counts.values().any(|&v| v == 3)
        )
    }

    fn count_letters(&self) -> HashMap<char, usize> {
        let mut h = HashMap::new();
        for c in self.0.chars() {
            let entry = h.entry(c).or_insert(0);
            *entry += 1;
        }
        h
    }

    fn overlap(&self, other: &BoxId) -> String {
        self.0.chars()
            .zip(other.0.chars())
            .filter(|(a, b)| a == b)
            .map(|(a, _)| a)
            .collect()
    }
}



fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let b = BufReader::new(f);
    let boxids = b.lines()
        .map(|l| l.and_then(|s| Ok(BoxId(s))))
        .collect::<Result<Vec<_>,_>>()?;

    // 2a
    let dt = boxids
        .iter()
        .map(|b| b.has_letter_dup_trip())
        .fold((0,0), |mut acc, x| {
            if x.0 { acc.0 += 1 }
            if x.1 { acc.1 += 1 }
            acc
        });
    println!("1a: dup trip checksum is {}", dt.0 * dt.1);

    // 2b
    // This will find the first boxID that matches another with distance 1 and print the
    // overlap. No other hits will be reported, so this a assumes a well doctored input.
    let overlap = boxids.iter().find_map(|b| {
        if let Some(other) = boxids.iter().find(|other| b.dist(other) == 1) {
            Some(b.overlap(other))
        } else {
            None
        }
    }).expect("No dist 1 hits in boxID's");
    println!("2b: Overlapping boxID letters: {}", overlap);
    Ok(())
}
