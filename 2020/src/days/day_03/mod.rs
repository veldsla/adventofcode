use anyhow::{anyhow, Result};

use crate::Problem;
use crate::grid::Grid;

#[derive(Default)]
pub struct Solution {
    input: Option<Grid<bool>>
}

use nom::{
    branch::alt,
    bytes::complete::is_a,
    character::complete::line_ending,
    multi::fold_many1,
    combinator::{all_consuming, eof, map},
    sequence::terminated,
    IResult
};
// Might move to helper parsers, seems useful
fn parse(i: &[u8]) -> IResult<&[u8], Grid<bool>> {
    let gridline = terminated(is_a("#."), alt((line_ending, eof)));
    let grid = fold_many1(gridline, (Vec::new(), 0usize, 0usize), |(mut v, _x, y), line: &[u8]| {
        //dimnension checks?
        let x = line.len();
        v.extend(line.iter().map(|&b| b == b'#'));
        (v, x, y+1)
    });
    map(all_consuming(grid), |(elements, dim_x, dim_y)| Grid::new(elements, dim_x, dim_y))(i)
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(i).map_err(|e| anyhow!(e.to_string()))?;
        self.input = Some(result.1);
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let grid = self.input.as_ref().ok_or(anyhow!("not parsed"))?;
        let walker = grid.walk_fixed((0,0), 3, 1, true, false);
        let trees = walker.filter(|&c| grid[c]).count();
        Ok(format!("{}", trees))
    }

    fn part2(&self) -> Result<String> {
        let grid = self.input.as_ref().ok_or(anyhow!("not parsed"))?;
        let trees:usize = vec![(1,1), (3, 1), (5, 1), (7, 1), (1, 2)].into_iter()
            .map(|slope| {
                let walker = grid.walk_fixed((0,0), slope.0, slope.1, true, false);
                walker.filter(|&c| grid[c]).count()
            }).product();
        Ok(format!("{}", trees))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FOREST: &[u8] = b"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn p1() {
        let grid = parse(FOREST).unwrap().1;
        assert_eq!(grid.dim_x, 11);
        assert_eq!(grid.dim_y, 11);
        
        let walker = grid.walk_fixed((0,0), 3, 1, true, false);
        let trees = walker.filter(|&c| grid[c]).count();
        assert_eq!(trees, 7);
    }

    #[test]
    fn p2() {
        let grid = parse(FOREST).unwrap().1;
        let trees:usize = vec![(1,1), (3, 1), (5, 1), (7, 1), (1, 2)].into_iter()
            .map(|slope| {
                let walker = grid.walk_fixed((0,0), slope.0, slope.1, true, false);
                walker.filter(|&c| grid[c]).count()
            }).product();
        assert_eq!(trees, 336);
    }
}
