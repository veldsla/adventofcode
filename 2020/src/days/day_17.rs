use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::Problem;

use nom::{
    bytes::complete::is_a,
    character::complete::line_ending,
    combinator::map,
    multi::fold_many1,
    combinator::all_consuming,
    sequence::terminated,
    IResult
};

#[derive(Default)]
pub struct Solution {
    input: Grid
}

#[derive(Clone, Debug, Default)]
struct Grid(HashMap<(i32, i32, i32), u8>);
struct HyperGrid(HashMap<(i32, i32, i32, i32), u8>);

impl Grid {
    fn new() -> Grid {
        Grid(HashMap::new())
    }

    fn set(&mut self,  c: (i32, i32, i32), v: u8) {
        self.0.insert(c, v);
        if v == b'#' {
            //expand the grid around
            self.create_neighbors(c);
        }
    }

    fn create_neighbors(&mut self, c: (i32, i32, i32)) {
        for x in neigh(c.0) {
            for y in neigh(c.1) {
                for z in neigh(c.2) {
                    self.0.entry((x,y,z)).or_insert(b'.');
                }
            }
        }
    }

    fn step_life(&mut self) {
        let changes: Vec<_> = self.0.iter()
            .filter_map(|(c, &v)| {
                //count active
                let active = self.neighbors_active(c);
                match v {
                    b'#' if active != 3 && active != 2 => Some((*c, b'.')),
                    b'.' if active == 3 => Some((*c, b'#')),
                    _ => None
                }
            }).collect();

        for c in changes {
            self.set(c.0, c.1);
        }

    }

    fn neighbors_active(&self, c: &(i32, i32, i32)) -> usize {
        let s = self.0.get(&c).expect("oob");
        let mut count = 0;
        for x in neigh(c.0) {
            for y in neigh(c.1) {
                for z in neigh(c.2) {
                    if let Some(&b'#') = self.0.get(&(x,y,z)) {
                        count += 1;
                    }
                }
            }
        }
        if s == &b'#' { count -= 1; }
        count
    }
    
    fn count_active(&self) -> usize {
        self.0.values().filter(|&&v| v == b'#').count()
    }
}

impl HyperGrid {
    fn from_grid(g: &Grid) -> HyperGrid {
        let mut grid = HyperGrid(HashMap::new());
        for (c, &v) in &g.0 {
            grid.set((c.0, c.1, c.2, 0), v);
        }
        grid
    }

    fn set(&mut self,  c: (i32, i32, i32, i32), v: u8) {
        self.0.insert(c, v);
        if v == b'#' {
            //expand the grid around
            self.create_neighbors(c);
        }
    }

    fn create_neighbors(&mut self, c: (i32, i32, i32, i32)) {
        for x in neigh(c.0) {
            for y in neigh(c.1) {
                for z in neigh(c.2) {
                    for w in neigh(c.3) {
                        self.0.entry((x, y, z, w)).or_insert(b'.');
                    }
                }
            }
        }
    }

    fn step_life(&mut self) {
        let changes: Vec<_> = self.0.iter()
            .filter_map(|(c, &v)| {
                //count active
                let active = self.neighbors_active(c);
                match v {
                    b'#' if active != 3 && active != 2 => Some((*c, b'.')),
                    b'.' if active == 3 => Some((*c, b'#')),
                    _ => None
                }
            }).collect();

        for c in changes {
            self.set(c.0, c.1);
        }

    }

    fn neighbors_active(&self, c: &(i32, i32, i32, i32)) -> usize {
        let s = self.0.get(&c).expect("oob");
        let mut count = 0;
        for x in neigh(c.0) {
            for y in neigh(c.1) {
                for z in neigh(c.2) {
                    for w in neigh(c.3) {
                        if let Some(&b'#') = self.0.get(&(x, y, z, w)) {
                            count += 1;
                        }
                    }
                }
            }
        }
        if s == &b'#' { count -= 1; }
        count
    }
    
    fn count_active(&self) -> usize {
        self.0.values().filter(|&&v| v == b'#').count()
    }
}

#[inline]
fn neigh(i: i32) -> std::ops::Range<i32> {
    i - 1..i + 2
}

fn parse(i: &[u8]) -> IResult<&[u8], Grid> {
    let gridline = terminated(is_a("#."), line_ending);
    let grid = fold_many1(gridline, (Grid::new(), 0i32), |(mut grid, y), line: &[u8]| {
        //dimnension checks?
        for (x, &c) in line.iter().enumerate() {
            grid.set((x as i32, y, 0), c);
        }
        (grid, y+1)
    });
    map(all_consuming(grid), |(elements, _dim_y)| elements)(i)
}



impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(i).map_err(|e| anyhow!(e.to_string()))?;
        self.input = result.1;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let mut grid = self.input.clone();
        for _i in 0..6 {
            grid.step_life();
        }
        Ok(format!("{}", grid.count_active()))
    }

    fn part2(&self) -> Result<String> {
        let mut hypergrid = HyperGrid::from_grid(&self.input);
        for _i in 0..6 {
            hypergrid.step_life();
        }
        Ok(format!("{}", hypergrid.count_active()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {

        let test = b".#.\n..#\n###\n";
        let result = parse(test);
        assert!(result.is_ok());
        let mut grid = result.unwrap().1;
        println!("{:?}", grid);
        for i in 0..6 {
            grid.step_life();
            println!("cycle {}, count = {}", i, grid.count_active());
        }
        assert_eq!(grid.count_active(), 112);
    }
}
