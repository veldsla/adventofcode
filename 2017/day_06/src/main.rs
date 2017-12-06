use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::iter::repeat;

struct Mem {
    banks: Vec<usize>,
}

/// find the max, resolve ties by choosing the lowest index. This behaviour is
/// different from the stdlib
/// The max value is replaced with 0
/// Returns a tuple (index_max, max)
fn take_first_max(d: &mut [usize]) -> (usize, usize) {
    let mut max = 0;
    let mut max_i = 0;

    for (i, v) in d.iter().enumerate() {
        if max < *v {
            max = *v;
            max_i = i
        }
    }
    d[max_i] = 0;

    (max_i, max)
}

impl Mem {
    fn new(v: &[usize]) -> Mem {
        let banks = v.to_vec();
        Mem { banks }
    }

    fn redistribute(&mut self) -> (usize, usize) {
        let mut seen = HashMap::new();
        seen.insert(self.banks.to_vec(), 0);
        let n = self.banks.len();
        let mut count = 0;
        loop {
            let (i, max) = take_first_max(&mut self.banks);
            let take_more_than_one = if max / n > 0 { n } else { 0 };
            let toadd = repeat(max / n).take(take_more_than_one).chain(repeat(1).take(max % n));
            for (i, a) in (i+1..n).chain(0..i+1).cycle().zip(toadd) {
                self.banks[i] += a;
            }
            count +=1;
            let state = self.banks.to_vec();
            if !seen.contains_key(&state){
                seen.insert(state, count);
            } else {
                return (count, *seen.get(&state).unwrap())
            }
        }
    }
}

fn main() {
    let f = File::open("input.txt").unwrap_or_else(|e| panic!("Error opening file:\n\t{}", e));
    let v: Vec<usize> = BufReader::new(f).lines().nth(0).unwrap().unwrap().split('\t').map(|s| s.parse().unwrap()).collect();
    let mut m = Mem::new(&v);
    let (n, i) = m.redistribute();
    println!("Cycle detected after {} (6a) iterations, first occurence at cycle {},  Δ={} (6b)", n, i, n - i);
}

#[test]
fn smax() {
    assert_eq!(take_first_max(&mut [0,1,1,3,4,0]), (4,4));
    assert_eq!(take_first_max(&mut [5,5,1,3,4,0]), (0,5));
    assert_eq!(take_first_max(&mut [4,1,1,3,5,5]), (4,5));
}

#[test]
fn redis() {
    let mut m = Mem::new(&[0,2,7,0]);
    assert_eq!(m.redistribute(), 5);
}
