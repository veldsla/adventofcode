use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Maze {
    data: Vec<char>,
    width: usize,
    height: usize,
    carts: Vec<Cart>
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Cart {
    y: usize,
    x: usize,
    direction: Direction,
    turned: usize,
    removed: bool
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Maze {
    fn from_file<P: AsRef<Path>>(p: P) -> io::Result<Maze> {
        let f = File::open(p)?;
        let b = BufReader::new(f);
        let mut data = Vec::new();
        let mut height = 0;
        let mut width = 0;
        let mut carts = Vec::new();
        for l in b.lines() {
            let l = l?;
            let l = l.trim_end_matches('\n');
            width = l.len();
            data.extend(
                l.chars().enumerate().map(|(x, c)| {
                    match c {
                        '<' => {
                            carts.push( Cart {x, y: height, direction: Direction::Left, turned: 0, removed: false });
                            '-'
                        },
                        '>' => {
                            carts.push( Cart {x, y: height, direction: Direction::Right, turned: 0, removed: false });
                            '-'
                        },
                        '^' => {
                            carts.push( Cart {x, y: height, direction: Direction::Up, turned: 0, removed: false });
                            '|'
                        },
                        'v' => {
                            carts.push( Cart {x, y: height, direction: Direction::Down, turned: 0, removed: false });
                            '|'
                        },
                        _ => c
                    }
                })
            );
            height += 1;
        }
        Ok(Maze{ data, width, height, carts })
    }

    fn run(&mut self) -> (usize, usize) {
        loop {
            //order the carts on x, y coord
            self.carts.sort();
            for i in 0..self.carts.len() {
                if let Some(collission) = self.move_cart(i) {
                    return collission;
                }
            }
        }
    }
    
    fn run_remove_collided(&mut self) -> (usize, usize) {
        loop {
            //order the carts on x, y coord
            self.carts.sort();
            for i in 0..self.carts.len() {
                if self.carts[i].removed {
                    continue;
                }
                if let Some((x, y)) = self.move_cart(i) {
                    //colission
                    self.carts.iter_mut().for_each(|c| if c.x == x && c.y == y { c.removed = true });
                }
            }
            if self.carts.iter().filter(|c| !c.removed).count() == 1 {
                let remaining = self.carts.iter().position(|c| !c.removed).unwrap();
                return (self.carts[remaining].x, self.carts[remaining].y);
            }
        }

    }

    fn move_cart(&mut self, cart: usize) -> Option<(usize, usize)> {
        //get the next element following direction
        if let Some(cart) = self.carts.get_mut(cart) {
            match cart.direction {
                Direction::Down => cart.y += 1,
                Direction::Left => cart.x -= 1,
                Direction::Right => cart.x += 1,
                Direction::Up => cart.y -= 1,
            }
            let newpos = self.width * cart.y + cart.x;
            match self.data[newpos] {
                '+' => {
                    match cart.turned % 3 {
                        0 if cart.direction == Direction::Up => cart.direction = Direction::Left,
                        0 if cart.direction == Direction::Down => cart.direction = Direction::Right,
                        0 if cart.direction == Direction::Left => cart.direction = Direction::Down,
                        0 if cart.direction == Direction::Right => cart.direction = Direction::Up,
                        2 if cart.direction == Direction::Up => cart.direction = Direction::Right,
                        2 if cart.direction == Direction::Down => cart.direction = Direction::Left,
                        2 if cart.direction == Direction::Left => cart.direction = Direction::Up,
                        2 if cart.direction == Direction::Right => cart.direction = Direction::Down,
                        _ => {},
                    }
                    cart.turned += 1;
                },
                '-' => {},
                '|' => {},
                '/' if cart.direction == Direction::Up=> cart.direction = Direction::Right,
                '/' if cart.direction == Direction::Right=> cart.direction = Direction::Up,
                '/' if cart.direction == Direction::Left=> cart.direction = Direction::Down,
                '/' if cart.direction == Direction::Down=> cart.direction = Direction::Left,
                '\\' if cart.direction == Direction::Up=> cart.direction = Direction::Left,
                '\\' if cart.direction == Direction::Right=> cart.direction = Direction::Down,
                '\\' if cart.direction == Direction::Left=> cart.direction = Direction::Up,
                '\\' if cart.direction == Direction::Down=> cart.direction = Direction::Right,
                _ => panic!("Error in maze")
            }
        }
        let newx = self.carts[cart].x;
        let newy = self.carts[cart].y;
        
        if self.carts.iter().enumerate().any(|(i, c)| i != cart && !c.removed && c.x == newx && c.y == newy) {
            Some((newx, newy))
        } else {
            None
        }
    }
}

fn main() -> io::Result<()> {
    let mut maze = Maze::from_file("input.txt")?;
    let (colx, coly) = maze.run();
    println!("13a: Collision at {},{}", colx, coly);

    let mut maze = Maze::from_file("input.txt")?;
    let (colx, coly) = maze.run_remove_collided();
    println!("13b: Remaining cart at {},{}", colx, coly);
    Ok(())
}

#[test]
fn test() {
    let mut maze = Maze::from_file("test.txt").unwrap();
    assert_eq!(maze.run(), (7,3));
}

#[test]
fn test2() {
    let mut maze = Maze::from_file("test2.txt").unwrap();
    assert_eq!(maze.run_remove_collided(), (6,4));
}
