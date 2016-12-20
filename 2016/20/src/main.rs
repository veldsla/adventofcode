extern crate rle_vec;

use std::fs::File;
use std::io::{BufReader, BufRead};


use rle_vec::{RleVec, Run};

#[derive(Debug,Ord, PartialOrd, Eq, PartialEq)]
struct RangeInc {
    start: usize,
    end: usize
}

fn main() {
    let f = File::open("input.txt").unwrap_or_else(|_| panic!("Error opening input.txt"));
    let mut v = RleVec::new();
    let mut r = Vec::new();
    for l in BufReader::new(f).lines() {
        let line = l.unwrap();
        let p = line.split("-").collect::<Vec<_>>();
        assert_eq!(p.len(), 2);
        r.push(RangeInc {
            start: p[0].parse().unwrap(),
            end: p[1].parse::<usize>().unwrap() + 1
        });
    }

    //sort ranges by start coordinate
    r.sort();

    let mut start = 0;
    for range in &r {
        if range.start > v.len() {
            //padd rle data with true for unblocked ips
            let n = range.start - v.len();
            v.push_run(Run {value: false, length: n })
        }
        //add the block defined by this run
        if range.end > v.len() {
            let n = range.end - v.len();
            v.push_run(Run {value: true, length:n});
        }
    }
    
    let mut pos = 0;
    let mut unblocked = 0;
    for run in v.iter_runs() {
        if !run.value {
            println!("Unblocked from {} to {}, size = {}", pos, pos+run.length, run.length);
            unblocked += run.length;
        }
        pos += run.length;
    }

    println!("A total of {} ip's is unblocked", unblocked);
}
