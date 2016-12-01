use std::collections::HashSet;

#[derive(Debug)]
struct Walker {
    x: i32,
    y: i32,
    orientation: Orientation,
    history: HashSet<(i32,i32)>
}

#[derive(Debug)]
enum Orientation {
    North,
    East,
    South,
    West
}

enum Move {
    R(i32),
    L(i32)
}

impl<'a> From<&'a str> for Move {
    fn from(s: &str) -> Move {
        let (d, n) = s.split_at(1);
        match d {
            "R" => Move::R(n.parse::<i32>().unwrap()),
            "L" => Move::L(n.parse::<i32>().unwrap()),
            _ => panic!("Error parsing move")
        }
    }
}

impl Walker {
    fn new() -> Walker {
        let mut hs = HashSet::new();
        hs.insert((0,0));
        Walker { x: 0, y : 0, orientation: Orientation::North, history: hs}
    }

    fn walk<M: Into<Move>>(&mut self, im: M) -> bool {
        let m = im.into();
        let go = match m {
            Move::R(n) => {
                self.orientation = match self.orientation {
                    Orientation::North => Orientation::East,
                    Orientation::East  => Orientation::South,
                    Orientation::South => Orientation::West,
                    Orientation::West  => Orientation::North
                };
                n
            },
            Move::L(n) => {
                self.orientation = match self.orientation {
                    Orientation::North => Orientation::West,
                    Orientation::East  => Orientation::North,
                    Orientation::South => Orientation::East,
                    Orientation::West  => Orientation::South
                };
                n
            }
        };

        match self.orientation {
            Orientation::North => (self.y+1..self.y+go+1).any(|y| { self.y = y; !self.history.insert((self.x, y)) }),
            Orientation::East  => (self.x+1..self.x+go+1).any(|x| { self.x = x; !self.history.insert((x, self.y)) }),
            Orientation::South => (self.y-go..self.y).rev().any(|y| { self.y = y; !self.history.insert((self.x, y)) }),
            Orientation::West  => (self.x-go..self.x).rev().any(|x| { self.x = x; !self.history.insert((x, self.y)) })
        }
    }

    fn distance_to_origin(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn main() {
    let input = "R4, R5, L5, L5, L3, R2, R1, R1, L5, R5, R2, L1, L3, L4, R3, L1, L1, R2, R3, R3, R1, L3, L5, R3, R1, L1, R1, R2, L1, L4, L5, R4, R2, L192, R5, L2, R53, R1, L5, R73, R5, L5, R186, L3, L2, R1, R3, L3, L3, R1, L4, L2, R3, L5, R4, R3, R1, L1, R5, R2, R1, R1, R1, R3, R2, L1, R5, R1, L5, R2, L2, L4, R3, L1, R4, L5, R4, R3, L5, L3, R4, R2, L5, L5, R2, R3, R5, R4, R2, R1, L1, L5, L2, L3, L4, L5, L4, L5, L1, R3, R4, R5, R3, L5, L4, L3, L1, L4, R2, R5, R5, R4, L2, L4, R3, R1, L2, R5, L5, R1, R1, L1, L5, L5, L2, L1, R5, R2, L4, L1, R4, R3, L3, R1, R5, L1, L4, R2, L3, R5, R3, R1, L3";

    let mut w = Walker::new();
    let mut success = false;
    for m in input.split(", ") {
        if w.walk(m) {
            println!("Easter Bunny Headquarters is {} blocks away", w.distance_to_origin());
            success = true;
            break;
        }
    }
    
    if !success {
        println!("Boohoo, I'm lost");
    }
}


#[test]
fn test() {
    let mut w = Walker::new();
    w.walk("R8");
    assert!(!w.walk("R4"));
    assert!(!w.walk("R4"));
    assert!(w.walk("R8")); 
    assert_eq!(w.distance_to_origin(), 4);

}
