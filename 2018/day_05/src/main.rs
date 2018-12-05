use std::fs::File;
use std::io::Read;

use std::time::Instant;

fn trim_units(s: &str, ignore: Option<char>) -> usize {
    //we assume ascii
    if let Some(c) = ignore {
        assert!(c.is_ascii());
    }
    assert!(s.is_ascii());

    //convert s to bytes
    let b: Vec<u8> = s.chars().map(|c| c as u8).collect();

    //prepare the visit vec
    let mut keep: Vec<usize> = if let Some(rm) = ignore {
        let rmb = rm as u8;
        b.iter().enumerate()
            .filter_map(|(pos, &c)| {
                let res = c ^ rmb;
                if res == 0 || res == 32 {
                    None
                } else {
                    Some(pos)
                }
            }).collect()

    } else {
        (0..b.len()).collect()
    };

    // the keep bookmark
    let mut pos = 0;

    while pos < keep.len() - 1 {
        let pos1 = keep[pos];
        let pos2 = keep[pos+1];

        //xor-ing the bytes exposes the ascii case bit
        if b[pos1] ^ b[pos2] == 32 {
            //this an opposite polarity pair
            //remove from keep
            keep.remove(pos);
            keep.remove(pos);
            if pos > 0 {
                pos -= 1;
            }
        } else {
            pos +=1;
        }
    }

    keep.len()
}


fn main() -> std::io::Result<()> {
    let mut f = File::open("input.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let s = s.trim_end();

    let now = Instant::now();
    let part_one =  trim_units(&s, None);
    let time = now.elapsed();
    println!("4a: reacted polymer has length {} ({:.3}s)",
        part_one,
        time.as_secs() as f64 + time.subsec_millis() as f64  / 1000.0);

    let now = Instant::now();
    // no iterator over char ranges?
    let (part_two, min_char) = (97..123)
        .map(|rm| {
            let c = char::from(rm);
            (
                trim_units(&s, Some(c)),
                c
            )
        })
        .min_by_key(|e| e.0).unwrap();
    let time = now.elapsed();
    println!("4a: minimum reacted polymer has length {} after removing {} ({:.3}s)",
        part_two,
        min_char,
        time.as_secs() as f64 + time.subsec_millis() as f64  / 1000.0);

    Ok(())
}
