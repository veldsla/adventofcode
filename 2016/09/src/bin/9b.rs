extern crate regex;

use std::fs::File;
use std::io::{BufReader, BufRead};

use regex::{Regex, Captures};

fn dlen(s: &str) -> usize {
    let m = Regex::new(r"\((\d+)x(\d+)\)").unwrap();

    let mut char_at_map = vec![0; s.len()];
    let mut char_factor_map = vec![1; s.len()];
    let mut prev_end = 0;

    for cap in m.captures_iter(s) {
        let (start, end) = cap.pos(0).unwrap();
        let (take, repeat) = parse(&cap).unwrap();

        //update factors
        for i in end..end+take {
            char_factor_map[i] *= repeat;
        }

        //mark characters
        for i in prev_end..start {
            char_at_map[i] = 1;
        }

        prev_end = end;
    }

    for i in prev_end..s.len() {
        char_at_map[i] = 1;
    }

    char_at_map.iter().zip(char_factor_map).fold(0, |acc, x| acc + x.0*x.1)
}

fn parse(cap: &Captures) -> Option<(usize,usize)> {
    if let Some(take) = cap.at(1) {
        if let Some(repeat) = cap.at(2) {
            let take: usize = take.parse().unwrap();
            let repeat: usize = repeat.parse().unwrap();
            return Some((take, repeat))
        }
    }
    None
}


fn main() {
    let f = File::open("input.txt").unwrap();
    let mut buf = String::new();
    let _ = BufReader::new(f).read_line(&mut buf).ok().expect("Error reading input file");
    println!("Decompressed size = {}", dlen(buf.trim_right_matches('\n')));
}


#[test]
fn test() {
    assert_eq!(dlen("ADVENT"), 6);
    assert_eq!(dlen("(3x3)XYZ"), 9);
    assert_eq!(dlen("X(8x2)(3x3)ABCY"), 20);
    assert_eq!(dlen("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
    assert_eq!(dlen("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"), 445);
}
