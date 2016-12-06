use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, BTreeMap};

struct RepetitionDecoder {
    letters: BTreeMap<usize, HashMap<char, usize>>
}

impl RepetitionDecoder {
    fn new() -> RepetitionDecoder {
        RepetitionDecoder { letters: BTreeMap::new() }
    }
    
    fn add_word(&mut self, w: &str) {
        for (i, c) in w.chars().enumerate() {
            let mut m = self.letters.entry(i).or_insert(HashMap::new());
            let mut count = m.entry(c).or_insert(0);
            *count +=1;
        }
    }

    fn get_most_frequent(&self) -> String {
        self.letters.values().map(|m| {
            let mut counts: Vec<_> = m.iter().map(|t| (t.1, t.0)).collect();
            counts.sort();
            *counts.last().unwrap().1
        }).collect()
    }

    fn get_least_frequent(&self) -> String {
        self.letters.values().map(|m| {
            let mut counts: Vec<_> = m.iter().map(|t| (t.1, t.0)).collect();
            counts.sort();
            *counts.first().unwrap().1
        }).collect()
    }
}

fn main() {
    let f = File::open("input.txt").unwrap_or_else(|_| panic!("Error opening input.txt"));
    
    let mut rd = RepetitionDecoder::new();
    for l in BufReader::new(f).lines() {
        rd.add_word(l.unwrap().as_ref());
    }

    println!("Most frequent letter decoding: {}", rd.get_most_frequent());
    println!("Using least frequent letter decoding: {}", rd.get_least_frequent());


}
