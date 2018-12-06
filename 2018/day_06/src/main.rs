extern crate itertools;

use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::i32;
use std::path::Path;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
struct Cell {
    x: i32,
    y: i32
}

impl FromStr for Cell {
    type Err = &'static str;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut p = line.split(',');
        let x = p.next().ok_or("Err parsing x").and_then(|s| s.trim().parse().map_err(|_| "Error parsing x"))?;
        let y = p.next().ok_or("Err parsing y").and_then(|s| s.trim().parse().map_err(|_| "Error parsing y"))?;
        Ok(Cell {x, y})
    }
}

impl Cell {
    fn dist(&self, other: &Cell) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
struct Field {
    targets: Vec<Cell>,
    x0: i32,
    x1: i32,
    y0: i32,
    y1: i32
}

impl Field {
    fn from_file<P: AsRef<Path>>(p: P) -> io::Result<Field> {
        let f = File::open(p)?;
        let b = BufReader::new(f);
        let targets = b.lines()
            .map(|l| l.and_then(|l| l.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Error parsing coord"))))
            .collect::<Result<Vec<Cell>, _>>()?;

        // get the field dimensions from the list
        let mut x0 = i32::MAX;
        let mut x1 = i32::MIN;
        let mut y0 = i32::MAX;
        let mut y1 = i32::MIN;
        for t in &targets {
            x0 = cmp::min(x0, t.x);
            x1 = cmp::max(x1, t.x);
            y0 = cmp::min(y0, t.y);
            y1 = cmp::max(y1, t.y);
        }

        Ok(Field { targets, x0, x1, y0, y1 })
    }

    fn largest_area_from_target(&self) -> usize {
        let mut target_size: HashMap<usize, usize> = (0..self.targets.len()).map(|p| (p, 0)).collect();

        for (y, x) in (self.y0..=self.y1).cartesian_product(self.x0..=self.x1) {
            if let Some(closest) = self.closest_target(Cell { x, y }) {
                //if we are on the field border this target will grow to infinite
                if x == self.x0 || x == self.x1 || y == self.y0 || y == self.y1 {
                    target_size.remove(&closest);
                } else if let Some(s) = target_size.get_mut(&closest) {
                    *s += 1;
                }
            }
        }
        target_size.values().max().unwrap() + 1
    }

    fn safest_space_size(&self, max_dist: i32) -> i32 {
        // it is possible a cluster of targset sits at the edge
        // the distance < max_dist would extend the current borders of the field
        // and the count would be off. let's see if this is a thing (panic!)
        let sd: Vec<i32> = (self.y0..=self.y1)
            .cartesian_product(self.x0..=self.x1)
            .map(|(y, x)| {
                if let Some(tot) = self.total_distance_targets(Cell { x, y }, max_dist) {
                    if tot < 10000 && (x == self.x0 || x == self.x1 || y == self.y0 || y == self.y1) {
                        panic!("Not implemented");
                    }
                    1
                } else {
                    0
                }
            }).collect();

        //sd is now a map of the valid region
        //I think it makes sense that this one is always connected
        sd.iter().sum()

    }

    fn closest_target(&self, cell: Cell) -> Option<usize> {
        let mut min = i32::MAX;
        let mut min_pos = None;
        for (pos, dist) in self.targets.iter()
            .enumerate()
            .map(|(i, t)| (i, t.dist(&cell)))
        {
            if dist == 0 { return None }
            if dist > 0 && dist < min {
                min = dist;
                min_pos = Some(pos)
            } else if dist == min {
                //tie
                min_pos = None
            }
        }
        min_pos
    }

    fn total_distance_targets(&self, cell: Cell, max_dist: i32) -> Option<i32> {
        let mut dist =0;
        for target in &self.targets {
            dist += target.dist(&cell);
            if dist >= max_dist {
                return None;
            }
        }
        Some(dist)
    }
}

fn main() -> io::Result<()> {
    let f = Field::from_file("input.txt")?;
    println!("6a: Largest field for a target is {}", f.largest_area_from_target());
    println!("6b: Largest safe field is {}", f.safest_space_size(10_000));
    Ok(())
}

#[test]
fn field() {
    let f = Field::from_file("test.txt").unwrap();
    assert_eq!(f.targets.len(), 6);
    assert_eq!(f.x0, 1);
    assert_eq!(f.x1, 8);
    assert_eq!(f.y0, 1);
    assert_eq!(f.y1, 9);
}

#[test]
fn largest() {
    let f = Field::from_file("test.txt").unwrap();
    println!("{:?}", f);
    assert_eq!(f.largest_area_from_target(), 17);
}

#[test]
fn safest() {
    let f = Field::from_file("test.txt").unwrap();
    println!("{:?}", f);
    assert_eq!(f.safest_space_size(32), 16);
}

