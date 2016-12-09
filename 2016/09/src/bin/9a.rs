extern crate regex;

use std::fs::File;
use std::io::{BufReader, BufRead};

use regex::Regex;

fn dlen(s: &str) -> usize {
    let m = Regex::new(r"\((\d+)x(\d+)\)").unwrap();

    let mut len = 0;
    let mut valid_from = 0;

    for cap in m.captures_iter(s) {
        //are we in a previous extension?
        let (start, end) = cap.pos(0).unwrap();
        if valid_from <= start {
            //add previous bytes to len
            len += start - valid_from;
            //expand current tag
            if let Some(take) = cap.at(1) {
                if let Some(repeat) = cap.at(2) {
                    let take: usize = take.parse().unwrap();
                    let repeat: usize = repeat.parse().unwrap();

                    len += take * repeat;
                    valid_from = take + end;
                } else {
                    panic!("Error parsing");
                }
            } else {
                panic!("Error parsing");
            }
        }
    }
    //add remaining characters 
    len += s.len() - valid_from;
    len
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
    assert_eq!(dlen("A(1x5)BC"), 7);
    assert_eq!(dlen("(3x3)XYZ"), 9);
    assert_eq!(dlen("A(2x2)BCD(2x2)EFG"), 11);
    assert_eq!(dlen("(6x1)(1x3)A"), 6);
    assert_eq!(dlen("X(8x2)(3x3)ABCY"), 18);
    assert_eq!(dlen("X(8x2)(3x3)ABCY(2x10)AA"), 38);
    assert_eq!(dlen("X(8x2)A(3x3)ABCY"), 19);
    assert_eq!(dlen("(2x8)AA(3x3)ABCY"), 26);
    assert_eq!(dlen("(2x8)AAA(3x3)ABCY"), 27);
}
