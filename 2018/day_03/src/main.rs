extern crate itertools;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::str::FromStr;

use itertools::Itertools;
use std::time::Instant;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Rect {
    id: u32,
    x0: u32,
    x1: u32,
    y0: u32,
    y1: u32
}

impl Rect {
    fn points(&self) -> impl Iterator<Item=(u32,u32)> {
        (self.x0..self.x1)
            .cartesian_product(self.y0..self.y1)
    }

    fn overlaps(&self, other: &Rect) -> bool {
        //no overlap if topleft is left or right from other
        //or below or above
        !(self.x0 >= other.x1 || other.x0 >= self.x1 ||
           self.y1 <= other.y0 || other.y1 <= self.y0)
    }
}

impl FromStr for Rect {
    type Err = &'static str;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut s = line.split(|c| c == '#' || c == ' ' || c == ',' || c == ':' || c == 'x' || c== '@')
            .filter(|p| !p.is_empty());
        let id = s.next().ok_or("Error parsing id").and_then(|v| v.parse().map_err(|_| "Error parsing id"))?;
        let x0 = s.next().ok_or("Error parsing x0").and_then(|v| v.parse().map_err(|_| "Error parsing x0"))?;
        let y0 = s.next().ok_or("Error parsing y0").and_then(|v| v.parse().map_err(|_| "Error parsing y0"))?;
        let dx: u32 = s.next().ok_or("Error parsing dx").and_then(|v| v.parse().map_err(|_| "Error parsing dx"))?;
        let dy: u32 = s.next().ok_or("Error parsing dy").and_then(|v| v.parse().map_err(|_| "Error parsing dy"))?;
        Ok(Rect {id, x0, x1: x0+dx, y0, y1: y0+dy })
    }
}

struct Fabric {
    claims: Vec<Rect>,
}

impl Fabric {
    fn from_file<P: AsRef<Path>>(p: P) -> io::Result<Fabric> {
        let f = File::open(p)?;
        let b = BufReader::new(f);
        let claims = b.lines()
            .map(|l| l.and_then(|l| l.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Error parsing rectangle"))))
            .collect::<Result<Vec<Rect>, _>>()?;
        Ok(Fabric { claims })
    }

    fn overlapping_hm(&self) -> usize {
        let mut point_map = HashMap::new();
        self.claims.iter().for_each(|s| {
            s.points().for_each(|t| {
                let e = point_map.entry(t).or_insert(0);
                *e +=1;
            });
        });

        point_map.values().filter(|&p| *p > 1).count()
    }

    fn overlapping_m(&self) -> usize {
        let max_x = self.claims.iter().map(|c| c.x1).max().unwrap() as usize;
        let max_y = self.claims.iter().map(|c| c.x1).max().unwrap() as usize;
        let mut map = vec![0; max_x * max_y];
        self.claims.iter().for_each(|s| {
            s.points().for_each(|t| {
                let pos = t.1 as usize * max_x + t.0 as usize;
                map[pos] += 1;
            });
        });
        map.iter().filter(|&p| *p > 1).count()
    }

    fn non_overlapping(&self) -> u32 {
        let free = self.claims.iter().find(|claim| {
            self.claims.iter()
                .filter(|other| other.id != claim.id)
                .all(|other| !claim.overlaps(other))
        }).expect("No non overlapping rectangles");
        free.id
    }
}


fn main() -> io::Result<()> {
    let fabric = Fabric::from_file("input.txt")?;
    let now = Instant::now();
    println!("3a: Overlapping coordinate count using hashmap: {} ({}ms)",fabric.overlapping_hm(), now.elapsed().subsec_millis());
    let now = Instant::now();
    println!("3a: Overlapping coordinate count using vec: {} ({}µs)",fabric.overlapping_m(), now.elapsed().subsec_micros());
    let now = Instant::now();
    println!("3b: Free rectangle has id {} ({}µs)",fabric.non_overlapping(), now.elapsed().subsec_micros());
    Ok(())
}

#[test]
fn parse() {
    assert_eq!("#1 @ 1,3: 4x4".parse::<Rect>(), Ok(Rect {id: 1, x0: 1, x1: 5, y0: 3, y1: 7}));
}
