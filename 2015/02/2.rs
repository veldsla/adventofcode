use std::str::FromStr;
use std::cmp;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn calc_area(s: String) -> i32 {
    let dims: Vec<i32> = s.split("x").map(|n| i32::from_str(n).unwrap()).collect();

    assert_eq!(dims.len(), 3);

    let (a,b,c) = (dims[0] * dims[1], dims[1] * dims[2], dims[0] * dims[2]);
    let min = cmp::min(cmp::min(a,b), c);
    (a + b + c)*2 + min
}

fn calc_ribbon(s: String) -> i32 {
    let mut dims: Vec<i32> = s.split("x").map(|n| i32::from_str(n).unwrap()).collect();

    assert_eq!(dims.len(), 3);

    dims.sort();

    let ribbon = dims[0] + dims[0] + dims[1] + dims[1];
    let bow = dims[0] * dims[1] * dims[2];

    ribbon + bow
}

fn main() {
    //println!("test {} ", calc_area("2x3x4".to_owned()));
    //println!("test {} ", calc_area("1x1x10".to_owned()));
    
    //toggle for 2a / 2b
    let day2a = false;

    let f = File::open(&"2_in.txt").unwrap();
    let reader = BufReader::new(f);

    let total = reader.lines().map(|l| {
        let line = l.ok().unwrap();
        match day2a {
            true => calc_area(line),
            false => calc_ribbon(line)
        }
    }).fold(0, |sum, x| sum + x);

    println!("Elves need {} {} ft {}", total, if day2a { "square" } else {""},if day2a { "wrapping papper" } else {"ribbon"}) ;
    
}
