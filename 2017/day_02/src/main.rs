use std::fs::File;
use std::io::{BufRead, BufReader};

fn range_diff(v: &[i32]) -> Option<i32> {
    if let Some(max) = v.iter().max() {
        if let Some(min) = v.iter().min() {
            return Some(max - min);
        }
    }
    None
}

fn div_pair(v: &[i32]) -> Option<i32> {
    for (i, x) in v.iter().enumerate() {
        for y in v.iter().skip(i + 1) {
            if x % y == 0 {
                return Some(x / y);
            } else if y % x == 0 {
                return Some(y / x);
            }
        }
    }
    None
}

fn main() {
    let f = File::open("input.txt").unwrap_or_else(|e| panic!("Error opening file:\n\t{}", e));
    let (one, two) = BufReader::new(f).lines().map(|l| {
        let line = l.unwrap_or_else(|_| panic!("Error reading line"));
        let values: Vec<i32> = line.split('\t')
            .map(|s| s.parse().unwrap_or_else(|e| panic!("error parsing number from: {}\n\t{}", s, e)))
            .collect();
        let r = range_diff(&values).unwrap_or_else(|| panic!("Error getting min and max from line: {}", line));
        let d = div_pair(&values).unwrap_or_else(|| panic!("No divisible pair in line: {}", line));
        (r, d)
    }).fold((0,0), |mut acc, v| {
        acc.0 += v.0;
        acc.1 += v.1;
        acc
    });

    println!("Checksum one = {}", one);
    println!("Checksum two = {}", two);

}
