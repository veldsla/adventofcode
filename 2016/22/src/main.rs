use std::str::FromStr;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Grid {
    nodes: Vec<Node>
}

impl Grid {
    fn new() -> Grid {
        Grid { nodes: Vec::new() }
    }

    fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    fn viable_pairs(&self) -> usize {
        let mut viable = 0;
        for x in 0..self.nodes.len() {
            for y in x+1..self.nodes.len() {
                if self.nodes[x].used <= self.nodes[y].avail && self.nodes[y].used <= self.nodes[x].avail {
                    viable += 1;
                }
            }
        }
        viable
    }
}

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,
    used: usize,
    avail: usize
}

#[derive(Debug)]
enum NodeError {
    ParseError(std::num::ParseIntError),
    NotValid
}

impl From<std::num::ParseIntError> for NodeError {
    fn from(err: std::num::ParseIntError) -> NodeError {
        NodeError::ParseError(err)
    }
}

impl FromStr for Node {
    type Err = NodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("/dev/grid") {
            Err(NodeError::NotValid )
        } else {
            let p: Vec<&str> = s.split(" ").filter(|s| s.len() > 0).collect();
            let xy: Vec<&str> = p[0].split("-").skip(1).collect();
            let avail = try!(p[3][..p[3].len()-1].parse());
            let used = try!(p[2][..p[2].len()-1].parse());
            let x = try!(xy[0][1..].parse());
            let y = try!(xy[1][1..].parse());

            Ok(Node { x: x, y: y, used: used, avail: avail })
        }
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();

    let mut grid = Grid::new();

    for l in BufReader::new(f).lines() {
        let line = l.unwrap();
        match line.parse() {
            Ok(node) => grid.add_node(node),
            Err(e) => println!("Skipping line: {}, because: {:?}", line, e)
        }
    }

    println!("The grid has {} viable pairs", grid.viable_pairs());

    //For the move task there are some interesting properties to the data
    //A line of much data (500+ TB) at y=20 execpt y=20,x=0, we will have to pass this node
    //there is a nice sink of 88TB at x=3 y=28. this is also the only node that can accept data
    //(from all other nodes except the y=28 line).
    //And we have to move it around to get the target node to the destination
    
    //so we move x=2,y=28 up and then x=1 and x=0 for 3 moves
    //then we move the empty node x=0, y=0 for 28 moves
    
    //then adjacent to our target node at x=31,y=0 for 31 moves
    //move the target node to x=31,y=0 the empty node in now behind the target. It takes four moves
    //to clear the node in front of the target so 5 moves for one step down
    // we need to go 31 steps down to get next to 0,0 and then the final step to solve the puzzle
    //
    // So 3 + 28 + 31+ 31*5 + 1 = 218
}
