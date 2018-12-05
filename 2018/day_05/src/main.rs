use std::fs::File;
use std::io::Read;

use std::time::Instant;

fn trim_units(b: &[u8], ignore: Option<u8>) -> usize {
    //prepare the visit queue
    let mut keep: Vec<usize> = if let Some(rm) = ignore {
        b.iter().enumerate()
            .filter_map(|(pos, &c)| {
                let res = c ^ rm;
                if res == 0 || res == 32 {
                    None
                } else {
                    Some(pos)
                }
            }).rev().collect()

    } else {
        (0..b.len()).rev().collect()
    };

    let mut visited = Vec::new();
    while keep.len() > 1 {
        let pos1 = keep.pop().unwrap();
        let pos2 = keep.pop().unwrap();

        //xor-ing the bytes exposes the ascii case bit
        if b[pos1] ^ b[pos2] == 32 {
            // this an opposite polarity pair
            // re-check the last character
            if let Some(p) = visited.pop() {
                keep.push(p);
            }
        } else {
            keep.push(pos2);
            visited.push(pos1);
        }
    }

    visited.len() + 1
}


fn main() -> std::io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let s = s.trim_end();

    //convert string to byte vec if validated ascii only
    assert!(s.is_ascii());
    let b: Vec<u8> = s.bytes().collect();

    let now = Instant::now();
    let part_one =  trim_units(&b, None);
    let time = now.elapsed();
    println!("4a: Reacted polymer has length {:.4} ({}ms)",
        part_one,
        time.subsec_micros() as f64 / 1000.0);

    let now = Instant::now();
    // no iterator over char ranges?
    let (part_two, min_char) = (b'a' ..= b'z')
        .map(|rm| {
            (
                trim_units(&b, Some(rm)),
                char::from(rm)
            )
        })
        .min_by_key(|e| e.0).unwrap();
    let time = now.elapsed();
    println!("4b: Minimum reacted polymer has length {} after removing {} ({:.4}ms)",
        part_two,
        min_char,
        time.subsec_micros() as f64 / 1000.0);

    Ok(())
}

#[test]
fn part_one() {
    assert_eq!(trim_units("dabAcCaCBAcCcaDA", None), 10);
}

#[test]
fn part_two() {
    assert_eq!(trim_units("dabAcCaCBAcCcaDA", Some('a')), 6);
    assert_eq!(trim_units("dabAcCaCBAcCcaDA", Some('b')), 8);
    assert_eq!(trim_units("dabAcCaCBAcCcaDA", Some('c')), 4);
    assert_eq!(trim_units("dabAcCaCBAcCcaDA", Some('d')), 6);
}