use std::cmp;
use std::fmt::{self, Display};
use std::fs::File;
use std::i32;
use std::io::{self, BufReader, BufRead};
use std::iter::repeat;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

struct PointField {
    points: Vec<Point>,
}

#[derive(Debug,Eq, PartialEq)]
struct Point {
    position: Coord,
    velocity: Coord
}

#[derive(Debug,Eq, PartialEq)]
struct Coord {
    x: i32,
    y: i32
}

impl FromStr for Point {
    type Err = &'static str;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        //position=<15,  0> velocity=<-2,  0>
        let mut e = line.split(|c| (c as u8) < 45 || (c as u8) > 58).filter(|s|!s.is_empty());
        let px = e.next().ok_or("Error_parsing x").and_then(|s| s.parse().map_err(|_| "Error_parsing x"))?;
        let py = e.next().ok_or("Error_parsing y").and_then(|s| s.parse().map_err(|_| "Error_parsing y"))?;
        let dx = e.next().ok_or("Error_parsing dx").and_then(|s| s.parse().map_err(|_| "Error_parsing dx"))?;
        let dy = e.next().ok_or("Error_parsing dy").and_then(|s| s.parse().map_err(|_| "Error_parsing dy"))?;
        Ok(Point {position: Coord {x: px, y: py}, velocity: Coord { x: dx, y: dy }})
    }
}

impl PointField {
    fn from_file<P: AsRef<Path>>(p: P) -> io::Result<PointField> {
        let f = File::open(p)?;
        let b = BufReader::new(f);
        let points = b.lines()
            .map(|l| l.and_then(|l| l.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData,format!("{}",e)))))
            //.inspect(|i| println!("{:?}", i))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(PointField { points })
    }

    fn limits(&self) -> (i32, i32, i32, i32) {
        let mut minx = i32::MAX;
        let mut miny = i32::MAX;
        let mut maxx = i32::MIN;
        let mut maxy = i32::MIN;

        for c in &self.points {
            minx = cmp::min(minx, c.position.x);
            miny = cmp::min(miny, c.position.y);
            maxx = cmp::max(maxx, c.position.x);
            maxy = cmp::max(maxy, c.position.y);
        }
        (minx, miny, maxx, maxy)
    }

    fn to_string(&self) -> String {
        let (minx, miny, maxx, maxy) = self.limits();
        let dimx = (maxx - minx).abs()+1;
        let dimy = (maxy - miny).abs()+1;

        let mut s: Vec<char> = repeat('.').take((dimx * dimy) as usize).collect();
        for p in &self.points {
            let pos = ((p.position.y - miny) * dimx + p.position.x - minx) as usize;
            s[pos] = '#';
        }

        s.chunks(dimx as usize).flat_map(|v| v.iter().chain(repeat(&'\n').take(1))).collect()
    }

    fn do_move(&mut self) {
        self.points.iter_mut().for_each(|p| p.do_move());
    }

    fn undo_move(&mut self) {
        self.points.iter_mut().for_each(|p| p.undo_move());
    }

    fn area(&self) -> u64 {
        let (minx, miny, maxx, maxy) = self.limits();
        let dimx = ((maxx - minx).abs()+1) as u64;
        let dimy = ((maxy - miny).abs()+1) as u64;
        dimx * dimy
        
    }
}

impl Point {
    fn do_move(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    fn undo_move(&mut self) {
        self.position.x -= self.velocity.x;
        self.position.y -= self.velocity.y;
    }
}

impl Display for PointField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


fn main() -> io::Result<()> {
    let mut field = PointField::from_file("input.txt")?;
    let mut seconds = 0;
    let mut grid_area = field.area();
    loop {
        field.do_move();
        if field.area() > grid_area {
            field.undo_move();
            println!("Grid at second {}\n{}", seconds, field);
            break
        }
        grid_area = field.area();
        seconds += 1;
    }

    Ok(())
}


