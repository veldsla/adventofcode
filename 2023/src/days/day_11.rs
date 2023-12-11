use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::is_a,
    character::complete::line_ending,
    multi::fold_many1,
    combinator::{all_consuming, eof, map},
    sequence::terminated,
    IResult
};
use itertools::Itertools;

use crate::grid::{Grid, Coord};

use crate::Problem;

#[derive(Default)]
pub struct Solution {
    grid: Grid<u8>,
}

impl Solution {
    fn paired_lengths(&self, age: usize) -> Vec<usize> {
        let mut result = Vec::new();

        // find the coordinates of all '#' characters
        let galaxy_coords: Vec<_> = self.grid.elements.iter().enumerate()
            .filter_map(|(i, &c)| { if c == b'#' { Some(self.grid.idx_to_coord(i)) } else { None } }).collect();

        // find the y coords of the empty lines;
        let empty_rows =(0isize..self.grid.dim_y as isize)
            .map(|y| (0isize..self.grid.dim_x as isize).all(|x| self.grid[Coord::from((x, y))] == b'.'))
            .collect::<Vec<_>>();

        //find the x coords of the empty columns
        let empty_cols = (0isize..self.grid.dim_x as isize)
            .map(|x| (0isize..self.grid.dim_y as isize).all(|y| self.grid[Coord::from((x, y))] == b'.'))
            .collect::<Vec<_>>();

        // calculate the distance between each pair of coordinates
        // add on age for each empty row/col between them
        for c in galaxy_coords.iter().combinations(2) {
            let (c1, c2) = (c[0], c[1]);
            let dist = usize::try_from((c1.x - c2.x).abs() + (c1.y - c2.y).abs()).unwrap();
            let empty_rows_between = (c1.y.min(c2.y)+1..c1.y.max(c2.y)).filter(|&y| empty_rows[y as usize]).count();
            let empty_cols_between = (c1.x.min(c2.x)+1..c1.x.max(c2.x)).filter(|&x| empty_cols[x as usize]).count();

            result.push(dist + ((empty_rows_between + empty_cols_between) * age));
        }
        
        result
    }
}

// from 2020
fn parse(s: &str) -> IResult<&str, Grid<u8>> {
    let gridline = terminated(is_a("#."), alt((line_ending, eof)));
    let grid = fold_many1(gridline, || (Vec::new(), 0usize, 0usize), |(mut v, _x, y), line: &str| {
        //dimnension checks?
        let x = line.len();
        v.extend(line.as_bytes().iter().copied());
        (v, x, y+1)
    });
    map(all_consuming(grid), |(elements, dim_x, dim_y)| Grid::new(elements, dim_x, dim_y))(s)
}



impl Problem for Solution {
    fn parse(&mut self, s: &str) -> Result<()> {
        let (s, result) = parse(s).map_err(|e| anyhow!("parse error: {e}"))?;
        self.grid = result;
        Ok(())

    }

    fn part1(&self) -> Result<String> {
        let paired_lengths:usize = self.paired_lengths(1)
            .into_iter()
            .sum();
        Ok(paired_lengths.to_string())
    }

    fn part2(&self) -> Result<String> {
        let paired_lengths:usize = self.paired_lengths(999_999)
            .into_iter()
            .sum();
        Ok(paired_lengths.to_string())
    }
}

