use std::fs::File;
use std::io::{BufRead, BufReader};

fn steps_a(mut v: Vec<i32>) -> usize {
    let mut pos = 0;
    let mut n = 0;
    loop {
        n +=1;
        let o = v[pos as usize];
        v[pos as usize] += 1;
        pos += o;
        if pos < 0 || pos as usize >= v.len() {
            break;
        }
    }
    n
}

fn steps_b(mut v: Vec<i32>) -> usize {
    let mut pos = 0;
    let mut n = 0;
    loop {
        n +=1;
        let o = v[pos as usize];
        v[pos as usize] += if o > 2 { -1 } else { 1 };
        pos += o;
        if pos < 0 || pos as usize >= v.len() {
            break;
        }
    }
    n
}


fn main() {
    let f = File::open("input.txt").unwrap_or_else(|e| panic!("Error opening file:\n\t{}", e));
    let data: Vec<i32> = BufReader::new(f).lines().map(|l| {
        let line = l.unwrap_or_else(|_| panic!("Error reading line"));
        line.parse().unwrap()
    }).collect();

    println!("Steps for 5a: {}", steps_a(data.clone()));
    println!("Steps for 5b: {}", steps_b(data.clone()));
}

