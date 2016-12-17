extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;
use std::mem;

struct Maze<'a> {
    hasher: Md5,
    seed: &'a str
}

struct Walk {
    path: String,
    x: usize,
    y: usize
}

impl<'a>Maze<'a> {
    fn new(s: &str) -> Maze {
        Maze { hasher: Md5::new(), seed: s }
    }

    fn find_shortest_path(&mut self, x: usize, y: usize) -> String {
        let mut queue = Vec::new();
        let mut next_queue = Vec::new();

        queue.push(Walk { x: x, y: y, path: String::from("") });
        //let mut seen = HashSet::new();

        loop {
            while let Some(walk) = queue.pop() {
                let next_walks = self.open_doors(walk);
                for w in next_walks {
                    if w.x == 4 && w.y == 4 {
                        return w.path;
                    }
                    next_queue.push(w);
                }
            }
            mem::swap(&mut queue, &mut next_queue)
        }
    }

    fn find_longest_path(&mut self, x: usize, y: usize) -> usize {
        let mut queue = Vec::new();
        let mut finished = Vec::new();

        queue.push(Walk { x: x, y: y, path: String::from("") });

        while let Some(walk) = queue.pop() {
            let next_walks = self.open_doors(walk);
            for w in next_walks {
                if w.x == 4 && w.y == 4 {
                    finished.push(w);
                } else {
                    queue.push(w);
                }
            }
        }

        finished.iter().max_by_key(|w| w.path.len()).unwrap().path.len()
        
    }

    fn open_doors(&mut self, w: Walk) -> Vec<Walk> {
        self.hasher.reset();
        self.hasher.input_str(&self.seed);
        self.hasher.input_str(&w.path);
        //first 4 chars indicate open up, down, left, and right if b >= char <= f
        self.hasher.result_str().chars().enumerate().take(4).filter_map(|c| {
            if c.1 >= 'b' && c.1 <= 'f' {
                //check for walls
                let mut p = w.path.clone();
                match c.0 {
                    0 if w.y != 1 => {
                        p.push('U');
                        Some(Walk { x:w.x, y: w.y-1, path: p })
                    },
                    1 if w.y != 4 => {
                        p.push('D');
                        Some(Walk { x:w.x, y: w.y+1, path: p })
                    },
                    2 if w.x != 1 => {
                        p.push('L');
                        Some(Walk { x:w.x-1, y: w.y, path: p })
                    },
                    3 if w.x != 4 => {
                        p.push('R');
                        Some(Walk { x:w.x+1 ,y: w.y, path: p })
                    },
                    _ => None
                }
            } else {
                None
            }
        }).collect()
    }
}

fn main() {
    let mut m = Maze::new("pxxbnzuo");
    println!("The shortest path through the maze = '{}'", m.find_shortest_path(1,1));
    println!("The longest path through the maze takes {} steps", m.find_longest_path(1,1));
}

#[test]
fn test() {
    let mut m = Maze::new("ihgpwlah");
    assert_eq!(m.find_shortest_path(1,1), "DDRRRD");
    let mut m = Maze::new("kglvqrro");
    assert_eq!(m.find_shortest_path(1,1), "DDUDRLRRUDRD");
    let mut m = Maze::new("ulqzkmiv");
    assert_eq!(m.find_shortest_path(1,1), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
}

#[test]
fn test2() {
    let mut m = Maze::new("ihgpwlah");
    assert_eq!(m.find_longest_path(1,1), 370);
    let mut m = Maze::new("kglvqrro");
    assert_eq!(m.find_longest_path(1,1), 492);
    let mut m = Maze::new("ulqzkmiv");
    assert_eq!(m.find_longest_path(1,1), 830);
}
