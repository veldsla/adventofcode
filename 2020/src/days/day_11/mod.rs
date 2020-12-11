use anyhow::{anyhow, Result};

use crate::Problem;
use crate::grid::Grid;

#[derive(Default)]
pub struct Solution {
    input: Grid<u8>
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
// from day 3
// Might move to helper parsers, seems useful
fn parse(i: &[u8]) -> IResult<&[u8], Grid<u8>> {
    let gridline = terminated(is_a("L."), alt((line_ending, eof)));
    let grid = fold_many1(gridline, (Vec::new(), 0usize, 0usize), |(mut v, _x, y), line: &[u8]| {
        //dimnension checks?
        let x = line.len();
        v.extend(line.iter().copied());
        (v, x, y+1)
    });
    map(all_consuming(grid), |(elements, dim_x, dim_y)| Grid::new(elements, dim_x, dim_y))(i)
}


#[inline] fn has_left(p: usize, dimx: usize) -> bool { p % dimx != 0 }
#[inline] fn has_right(p: usize, dimx: usize) -> bool { (p + 1) % dimx != 0 }
#[inline] fn has_up(p: usize, dimx: usize) -> bool { p >= dimx }
#[inline] fn has_down(p: usize, dimx: usize, l: usize) -> bool { p < l - dimx }

#[inline]
fn get_l(mut p: usize, g: &Grid<u8>) -> Option<usize> {
    while has_left(p, g.dim_x) {
        p -= 1;
        if g.elements[p] != b'.' {
            return Some(p);
        }
    }
    None
}

#[inline]
fn get_lu(mut p: usize, g: &Grid<u8>) -> Option<usize> {
    while has_left(p, g.dim_x) && has_up(p, g.dim_x) {
        p -= 1 + g.dim_x;
        if g.elements[p] != b'.' {
            return Some(p);
        }
    }
    None
}

#[inline]
fn get_u(mut p: usize, g: &Grid<u8>) -> Option<usize> {
    while has_up(p, g.dim_x) {
        p -= g.dim_x;
        if g.elements[p] != b'.' {
            return Some(p);
        }
    }
    None
}

#[inline]
fn get_ru(mut p: usize, g: &Grid<u8>) -> Option<usize> {
    while has_right(p, g.dim_x) && has_up(p, g.dim_x) {
        p -= g.dim_x - 1;
        if g.elements[p] != b'.' {
            return Some(p);
        }
    }
    None
}

#[inline]
fn get_r(mut p: usize, g: &Grid<u8>) -> Option<usize> {
    while has_right(p, g.dim_x) {
        p += 1;
        if g.elements[p] != b'.' {
            return Some(p);
        }
    }
    None
}

#[inline]
fn get_rd(mut p: usize, g: &Grid<u8>) -> Option<usize> {
    while has_right(p, g.dim_x) && has_down(p, g.dim_x, g.elements.len()) {
        p += 1 + g.dim_x;
        if g.elements[p] != b'.' {
            return Some(p);
        }
    }
    None
}

#[inline]
fn get_d(mut p: usize, g: &Grid<u8>) -> Option<usize> {
    while has_down(p, g.dim_x, g.elements.len()) {
        p += g.dim_x;
        if g.elements[p] != b'.' {
            return Some(p);
        }
    }
    None
}

#[inline]
fn get_ld(mut p: usize, g: &Grid<u8>) -> Option<usize> {
    while has_left(p, g.dim_x) && has_down(p, g.dim_x, g.elements.len()) {
        p += g.dim_x - 1;
        if g.elements[p] != b'.' {
            return Some(p);
        }
    }
    None
}

fn get_adjacent(p: usize, dimx: usize, l: usize) -> Vec<usize> {
    let mut a = Vec::new();
    let has_left = has_left(p, dimx);
    let has_up = has_up(p, dimx);
    let has_right = has_right(p, dimx);
    let has_down = has_down(p, dimx, l);

    if has_left { a.push(p - 1); }
    if has_left && has_up { a.push(p - 1- dimx); }
    if has_up { a.push(p - dimx); }
    if has_up && has_right { a.push(p - dimx + 1); }
    if has_right { a.push(p + 1); }
    if has_right && has_down { a.push(p + 1 + dimx); }
    if has_down { a.push(p + dimx); }
    if has_down && has_left { a.push(p + dimx - 1); }
    a
}

fn adjacent(g: &Grid<u8>) -> Vec<Vec<usize>> {
    let l = g.elements.len();
    let dimx = g.dim_x;
    (0..l).map(|p| get_adjacent(p, dimx, l)).collect()
}

fn adjacent_skip_floor(g: &Grid<u8>) -> Vec<Vec<usize>> {
    let l = g.elements.len();
    (0..l).map(|p| {
        let mut a = Vec::new();
        if let Some(e) = get_l(p, g) { a.push(e) }
        if let Some(e) = get_lu(p, g) { a.push(e) }
        if let Some(e) = get_u(p, g) { a.push(e) }
        if let Some(e) = get_ru(p, g) { a.push(e) }
        if let Some(e) = get_r(p, g) { a.push(e) }
        if let Some(e) = get_rd(p, g) { a.push(e) }
        if let Some(e) = get_d(p, g) { a.push(e) }
        if let Some(e) = get_ld(p, g) { a.push(e) }
        a
    }).collect()
}

type Change = (usize, u8);
fn life_it(g: &mut Grid<u8>, skip_floor: bool, allowed: usize) {
    let adjacent =  if skip_floor { adjacent_skip_floor(g) } else { adjacent(g) };
    loop {
        let changes: Vec<Change> = g.elements.iter().enumerate().filter(|(_, &e)| e != b'.').filter_map(|(i, &e)| {
            match e {
                b'L' => {
                    //empty,occupy if no occupied  adjacent
                    if !adjacent[i].iter().any(|&a| g.elements[a] == b'#') {
                        Some((i, b'#'))
                    } else {
                        None
                    }
                },
                b'#' => {
                    //occupied, leave if 4+ adjacent also occupied
                    if adjacent[i].iter().filter(|&&a| g.elements[a] == b'#').count() >= allowed {
                        Some((i, b'L'))
                    } else {
                        None
                    }
                },
                _ => None
            }
        }).collect();

        if changes.is_empty() {
            break;
        }

        for c in changes {
            g.elements[c.0] = c.1;
        }
    }
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(i).map_err(|e| anyhow!(e.to_string()))?;
        self.input = result.1;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let mut grid = self.input.clone();
        life_it(&mut grid, false, 4);
        let occ = grid.elements.iter().filter(|&&e| e == b'#').count();

        Ok(format!("{}", occ))
    }

    fn part2(&self) -> Result<String> {
        let mut grid = self.input.clone();
        life_it(&mut grid, true, 5);
        let occ = grid.elements.iter().filter(|&&e| e == b'#').count();
        Ok(format!("{}", occ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &[u8] = b"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

    #[test]
    fn p1() {
        let result = parse(TEST);
        assert!(result.is_ok());
        let mut grid = result.unwrap().1;
        println!("{}", grid);
        life_it(&mut grid, false, 4);
        assert_eq!(grid.elements.iter().filter(|&&e| e == b'#').count(), 37);
    }

    #[test]
    fn p2() {
        let result = parse(TEST);
        assert!(result.is_ok());
        let mut grid = result.unwrap().1;
        println!("{}", grid);
        life_it(&mut grid, true, 5);
        assert_eq!(grid.elements.iter().filter(|&&e| e == b'#').count(), 26);
    }


}
