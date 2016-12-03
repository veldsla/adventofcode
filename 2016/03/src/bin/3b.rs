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
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    let mut v3 = Vec::new();
    for l in b.lines() {
        let line = l.unwrap();
        let mut sides: Vec<u32> = line.split(" ").filter(|p| !p.is_empty())
            .map(|p| p.trim().parse::<u32>().unwrap_or_else(|e| panic!(format!("Error parsing input ('{}'): {}",line, e))))
            .collect();
        v3.push(sides.pop().unwrap_or_else(|| panic!("Need three elements per line")));
        v2.push(sides.pop().unwrap_or_else(|| panic!("Need three elements per line")));
        v1.push(sides.pop().unwrap_or_else(|| panic!("Need three elements per line")));
    }

    assert_eq!(v1.len() % 3, 0);
    assert_eq!(v2.len() % 3, 0);
    assert_eq!(v3.len() % 3, 0);
    v1.extend(v2);
    v1.extend(v3);
    for sides in v1.chunks(3) {
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
