use std::fs::File;
use std::io::{BufReader, BufRead};

struct Triangle((u32,u32,u32));

impl Triangle {
    fn new(s1: u32, s2: u32, s3: u32) -> Result<Triangle, ()> {
        if (s1 + s2) > s3 && (s1 + s3) > s2 && (s2 + s3) > s1 {
            Ok(Triangle((s1,s2,s3)))
        } else {
            Err(())
        }
    }
}



fn main() {
    let b = File::open("input.txt").and_then(|f| Ok(BufReader::new(f)))
        .unwrap_or_else(|_| panic!("Error opening input data"));

    let mut num = 0;
    for l in b.lines() {
        let line = l.unwrap();
        let sides: Vec<u32> = line.split(" ").filter(|p| !p.is_empty())
            .map(|p| p.trim().parse::<u32>().unwrap_or_else(|e| panic!(format!("Error parsing input ('{}'): {}",line, e))))
            .collect();
        match Triangle::new(sides[0], sides[1], sides[2]) {
            Ok(_) => num += 1,
            Err(_) => {}
        }
    }

    println!("There are {} valid triangles", num);
}

#[test]
fn test() {
    assert!(Triangle::new(5,10,25).is_err());
    assert!(Triangle::new(5,10,10).is_ok());
}
