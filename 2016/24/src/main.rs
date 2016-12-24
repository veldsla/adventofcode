extern crate permutohedron;

use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::mem;

use permutohedron::Heap;

#[derive(Debug, Eq, PartialEq)]
enum Point {
    Wall,
    Empty,
    Waypoint(char)
}

struct Maze {
    points: Vec<Point>,
    width: usize
}

impl Maze {
    fn from_lines(v: &Vec<String>) -> Maze {
        let mut p = Vec::new();
        let mut width = 0;
        for l in v {
            width = l.len();
            p.extend(l.chars().map(|c| {
                match c {
                    '0'...'9' => Point::Waypoint(c),
                    '#' => Point::Wall,
                    '.' => Point::Empty,
                    _ => panic!("Error in maze definition")
                }
            }));
        }
        Maze { points: p, width: width }
    }

    fn route_waypoints(&self, return_home: bool) -> usize {
        let start = self.points.iter().position(|p| p == &Point::Waypoint('0')).unwrap();
        let waypoints: Vec<usize> = (0..self.points.len()).filter(|i| {
            match self.points[*i] {
                Point::Waypoint(_) => true, 
                _ => false
            }
        }).collect();
        println!("Found {} waypoints", waypoints.len());

        let mut pathmap = HashMap::new();
        for a in 0..waypoints.len() {
            for b in a+1..waypoints.len() {
                let p = self.shortest_path(waypoints[a],waypoints[b]);
                pathmap.insert((waypoints[a],waypoints[b]), p);
            }
        }
        println!("Stored {} shortest paths", pathmap.len());

        let mut to_route: Vec<_> = waypoints.iter().filter(|&&p| p != start).collect();
        let perms = Heap::new(&mut to_route);
        let mut min_distance = None;
        for route in perms {
            let mut distance = 0;
            for i in 0..route.len() {
                let from = if i == 0 { start } else { *route[i-1] };
                let to = *route[i];
                let p = if from < to { (from, to) } else { ( to, from) };
                distance += *pathmap.get(&p).unwrap();
                if min_distance.is_some() && min_distance.unwrap() < distance { break; } 
            }

            if return_home {
                let from = **route.last().unwrap();
                let to = start;
                let p = if from < to { (from, to) } else { ( to, from) };
                distance += *pathmap.get(&p).unwrap();
            }

            if let Some(dist) = min_distance {
                if dist > distance {
                    min_distance = Some(distance);
                }
            } else {
                min_distance = Some(distance);
            }

        }

        min_distance.unwrap()
    }

    fn shortest_path(&self, a: usize, b: usize) -> usize {

        let mut queue = Vec::new();
        let mut next_queue = Vec::new();
        let mut seen = HashSet::with_capacity(self.points.len());

        queue.push(a);
        let mut distance = 0;
        loop {
            while let Some(pos) = queue.pop() {
                if seen.contains(&pos) {
                    continue;
                }
                if pos == b {
                    return distance;
                }
                seen.insert(pos);
                let mut next: Vec<_> = self.moves(pos).into_iter().filter(|p| !seen.contains(&p)).collect();
                next_queue.append(&mut next);
            }
            distance += 1;
            mem::swap(&mut queue, &mut next_queue);
        }
    }

    fn moves(&self, i: usize) -> Vec<usize> {
        vec![-(self.width as i64), -1, 1, self.width as i64].into_iter()
            .map(|v| i as i64 + v)
            .filter(|&r| r >= 0 && r < self.points.len() as i64)
            .map(|i| i as usize)
            .filter(|&p| self.points[p] != Point::Wall)
            .collect()
    }
}

fn main() {
    let f = File::open("input.txt").unwrap_or_else(|_| panic!("Error opening input.txt"));    
    let lines = BufReader::new(f).lines().map(|l| l.unwrap()).collect();
    let  maze = Maze::from_lines(&lines);
    println!("The shortest route throught the waypoints = {} moves", maze.route_waypoints(false));
    println!("The shortest route throught the waypoints and returning home = {} moves", maze.route_waypoints(true));

}

#[test]
fn test() {
    let input = "\
###########
#0.1.....2#
#.#######.#
#4.......3#
###########
";
    let l = input.lines().map(|l| String::from(l)).collect();
    let maze = Maze::from_lines(&l);
    assert_eq!(maze.points.len(), 55);
    assert_eq!(maze.width, 11);
    assert_eq!(maze.moves(12), vec![13,23]);
    assert_eq!(maze.shortest_path(12, 35), 3);
    assert_eq!(maze.route_waypoints(false), 14);
    assert_eq!(maze.route_waypoints(true), 20);
}
