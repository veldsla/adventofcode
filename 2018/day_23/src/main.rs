use std::cmp::{min, max};
use std::fs::File;
use std::fmt;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Ord, Clone, PartialOrd, Eq, PartialEq)]
struct Bot {
    r: i64,
    x: i64,
    z: i64,
    y: i64,
}

fn ioerror<E: fmt::Display>(e: E) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, format!("{}", e))
}

impl FromStr for Bot {
    type Err = io::Error;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parts = line.split(|c| (c as u8) < 45 || (c as u8) > 57).filter(|s|!s.is_empty());
        let x = parts.next().ok_or(ioerror("error parsing instruction"))
            .and_then(|s| s.parse().map_err(ioerror))?;
        let y = parts.next().ok_or(ioerror("error parsing instruction"))
            .and_then(|s| s.parse().map_err(ioerror))?;
        let z = parts.next().ok_or(ioerror("error parsing instruction"))
            .and_then(|s| s.parse().map_err(ioerror))?;
        let r = parts.next().ok_or(ioerror("error parsing instruction"))
            .and_then(|s| s.parse().map_err(ioerror))?;
        Ok(Bot { x, y, z, r })
    }
}

struct Bots(Vec<Bot>);

impl Bots {
    fn from_file<P: AsRef<Path>>(p: P) -> io::Result<Bots> {
        let f = File::open(p)?;
        let b = BufReader::new(f);
        let bots = b.lines().map(|l| {
            l.and_then(|l| l.parse())
        }).collect::<Result<Vec<Bot>, _>>()?;
        Ok(Bots(bots))
    }    

    fn strongest(&self) -> Option<&Bot> {
        self.0.iter().max()
    }

    fn in_radius(&self, b: &Bot) -> usize {
        self.0.iter()
            .filter(|other| b.dist(other) <= b.r)
            .count()
    }

    fn best_exposure(&self) -> (i64, i64, i64) {
        let (mut minx, mut maxx, mut miny, mut maxy, mut minz, mut maxz) = self.0.iter().fold((0,0,0,0,0,0), |acc, x| {
            ( min(x.x, acc.0), max(x.x, acc.1),
            min(x.y, acc.2), max(x.y, acc.3),
            min(x.z, acc.4), max(x.x, acc.5))
        });

        let mut range = 1;
        while range < maxx - minx {
            range *= 2;
        }

        //thank you reddit
        loop {
            let mut target_count = 0;
            let mut best = (0, 0, 0);
            let mut best_val = 0;

            for x in (minx..=maxx).step_by(range as usize) {
                for y in (miny..=maxy).step_by(range  as usize) {
                    for z in (minz..=maxz).step_by(range as usize) {
                        let count = self.0.iter()
                            .filter(|b| b.contains_coord(x, y, z))
                            .count();
                        if count > target_count {
                            // square with higher count
                            target_count = count;
                            best = (x, y, z);
                        } else if count == target_count {
                            // tie breaks, pick closest to origin
                            if x.abs() + y.abs() + z.abs() < best_val {
                                best_val = x.abs() + y.abs() + z.abs();
                                best = (x, y, z);
                            }
                        }
                    }
                }
            }

            if range == 1 {
                return best;
            }

            minx = best.0 - range;
            maxx = best.0 + range;
            miny = best.1 - range;
            maxy = best.1 + range;
            minz = best.2 - range;
            maxz = best.2 + range;

            range /= 2;
        }

    }
}

impl Bot {
    fn contains_coord(&self, x: i64, y: i64, z: i64) -> bool {
        let centerdist = (self.x - x).abs() + (self.y - y).abs() + (self.z - z).abs();
        let dist = centerdist - self.r;
        dist <= 0
    }

    fn dist(&self, other: &Bot) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}


fn main() -> io::Result<()> {
    let bots = Bots::from_file("input.txt")?;
    let strongest = bots.strongest().ok_or_else(|| ioerror("No bots in input file"))?;
    println!("23a: The strongest bot at {},{} with radius {} has {} bots in range",
             strongest.x, strongest.y, strongest.r, bots.in_radius(strongest));
    let (x, y, z) = bots.best_exposure();
    println!("23b: Best exposure at ({},{},{}), distance from origin: {}", x, y, x, x.abs()+y.abs()+z.abs());
    Ok(())
}

#[test]
fn test() {
    let bots = Bots::from_file("test.txt").unwrap();
    let s = bots.strongest().unwrap();
    assert_eq!(s.r, 4);
    assert_eq!(bots.in_radius(s), 7);
    let bots = Bots::from_file("test2.txt").unwrap();
    assert_eq!(bots.best_exposure(), (12,12,12))
}

