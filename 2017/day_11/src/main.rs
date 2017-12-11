use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Hex {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" => Ok(Direction::North),
            "ne" => Ok(Direction::NorthEast),
            "se" => Ok(Direction::SouthEast),
            "s" => Ok(Direction::South),
            "sw" => Ok(Direction::SouthWest),
            "nw" => Ok(Direction::NorthWest),
            _ => Err(format!("Error parsing direction from {}", s))
        }
    }
}



impl Hex {
    /// assum x=0 from the hex grid has the 'higher' hex, then x=1 y=0 is southeast of 0,0 
    fn walk(&mut self, d: Direction) {
        let high = self.is_high();
        match d {
            Direction::North => self.y -= 1,
            Direction::NorthEast if high  => { self.x += 1; self.y -= 1 },
            Direction::NorthEast => { self.x += 1; self.y -= 0 },
            Direction::SouthEast if high => { self.x += 1; self.y += 0 },
            Direction::SouthEast => { self.x += 1; self.y += 1 },
            Direction::South => self.y += 1,
            Direction::SouthWest if high => { self.x -= 1; self.y += 0 },
            Direction::SouthWest => { self.x -= 1; self.y += 1 },
            Direction::NorthWest if high => { self.x -= 1; self. y-= 1 },
            Direction::NorthWest => { self.x -= 1; self. y-= 0 },
        }

    }

    #[inline]
    fn is_high(&self) -> bool {
        self.x % 2 == 0
    }

    fn dist_home(&self) -> i32 {
        //walk diagonally to x=0
        //add remaning y
        self.x.abs() + self.y.abs() - self.x.abs()/2
        
    }
}

fn main() {
    let f = File::open("input.txt").unwrap_or_else(|e| panic!("Error opening file:\n\t{}", e));
    let path: Vec<Direction> = BufReader::new(f).lines().nth(0).unwrap().unwrap().split(',').map(|s| s.parse().unwrap()).collect();

    let mut dist = 0;
    let mut h = Hex {x: 0, y: 0};
    for d in path {
        h.walk(d);
        dist = std::cmp::max(dist, h.dist_home());
    }
    println!("Route home is {} is steps", h.dist_home());
    println!("Max home dist {} was steps", dist);
}
