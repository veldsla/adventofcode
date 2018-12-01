use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

struct FrequencyDevice {
    changes: Vec<i32>,
}

impl FrequencyDevice {
    fn from_file<P: AsRef<Path>>(p: P) -> io::Result<FrequencyDevice> {
        let mut f = File::open(p)?;
        let mut data = String::new();
        f.read_to_string(&mut data)?;
        let changes = data
            .split_whitespace()
            .map(|n| {
                n.parse().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{},  input was: {} ", e, n),
                    )
                })
            }).collect::<Result<Vec<i32>, _>>()?;
        Ok(FrequencyDevice { changes })
    }

    fn final_frequency(&self) -> i32 {
        self.changes.iter().fold(0, |x, acc| acc + x)
    }

    fn first_duplicated_frequency(&self) -> i32 {
        let mut h = HashSet::new();
        h.insert(0);
        let mut sum = 0;

        self.changes
            .iter()
            .cycle()
            .map(|x| {
                sum += x;
                sum
            }).find(|s| !h.insert(*s))
            .unwrap()
    }
}

fn main() -> Result<(), String> {
    let f = FrequencyDevice::from_file("input.txt")
        .map_err(|e| format!("Cannot read input.txt: {}", e))?;
    println!("1a: Final frequency is {}.", f.final_frequency());
    println!("1b: First duplicated frequency is {}.", f.first_duplicated_frequency());

    Ok(())
}

#[test]
fn test() {
    let f = FrequencyDevice {
        changes: vec![1, -2, 3, 1],
    };
    // part 1a
    assert_eq!(f.final_frequency(), 3);

    // part 1b
    assert_eq!(f.first_duplicated_frequency(), 2);
    assert_eq!(
        FrequencyDevice {
            changes: vec![1, -1]
        }.first_duplicated_frequency(),
        0
    );
    assert_eq!(
        FrequencyDevice {
            changes: vec![3, 3, 4, -2, -4]
        }.first_duplicated_frequency(),
        10
    );
    assert_eq!(
        FrequencyDevice {
            changes: vec![-6, 3, 8, 5, -6]
        }.first_duplicated_frequency(),
        5
    );
    assert_eq!(
        FrequencyDevice {
            changes: vec![7, 7, -2, -7, -4]
        }.first_duplicated_frequency(),
        14
    );
}
