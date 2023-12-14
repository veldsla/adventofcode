use std::convert::TryFrom;
use std::fmt;
use std::ops::{Add, Index};

#[derive(Debug, Clone, Default)]
pub struct Grid<T> {
    pub dim_x: usize,
    pub dim_y: usize,
    pub elements: Vec<T>
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl From<(isize, isize)> for Coord {
    fn from(t: (isize, isize)) -> Coord {
        Coord { x: t.0, y: t.1 }
    }
}

impl Add for Coord {
    type Output = Coord;
    fn add(mut self, rhs: Coord) -> Coord {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl<T> Grid<T> {
    pub fn new<I: IntoIterator<Item=T>>(v: I, dim_x: usize, dim_y: usize) -> Grid<T> {
        let elements = v.into_iter().collect();
        Grid { dim_x, dim_y, elements }
    }

    pub fn walk_fixed<C: Into<Coord>>(&self, from: C, dx: isize, dy: isize, wrap_x: bool, wrap_y: bool) -> Walker {
        Walker {position: from.into(), dx, dy, max_x:self.dim_x, max_y: self.dim_y, wrap_x, wrap_y }
    }

    pub fn idx_to_coord(&self, idx: usize) -> Coord {
        let x = (idx % self.dim_x) as isize; 
        let y = (idx / self.dim_x) as isize;
        Coord { x , y }
    }

    pub fn iter_row(&self, y: usize) -> impl Iterator<Item=&T> {
        let start = y * self.dim_x;
        let end = start + self.dim_x;
        self.elements[start..end].iter()
    }

    pub fn iter_col(&self, x: usize) -> impl Iterator<Item=&T> {
        self.elements.iter().skip(x).step_by(self.dim_x)
    }

}

impl<T> Grid<T> where T: Eq {
    pub fn compare_rows(&self, row1: usize, row2: usize) -> bool {
        self.iter_row(row1).zip(self.iter_row(row2)).all(|(a, b)| a == b)
    }

    pub fn compare_cols(&self, col1: usize, col2: usize) -> bool {
        self.iter_col(col1).zip(self.iter_col(col2)).all(|(a, b)| a == b)
    }
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    //FIXME needs better oob check
    fn index(&self, c: Coord) -> &Self::Output {
        let pos = c.x + c.y * self.dim_x as isize;
        &self.elements[usize::try_from(pos).expect("Cannot index negative coords")]
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    //FIXME needs better oob check
    fn index(&self, p: usize) -> &Self::Output {
        &self.elements[p]
    }
}

impl fmt::Display for Grid<u8> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        for p in (0..self.elements.len()).step_by(self.dim_x) {
            for &e in &self.elements[p..p + self.dim_x] {
                write!(f, "{} ",char::from(e))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct Walker {
    position: Coord,
    dx: isize,
    dy: isize,
    max_x: usize,
    max_y: usize,
    wrap_x: bool,
    wrap_y: bool
}

fn wrap_or_oob(x:isize, max: isize, wrap: bool) -> Option<isize> {
    if x < 0 {
        if wrap {
            Some(x.rem_euclid(max))
        } else {
            None
        }
    } else if x >= max {
        if wrap {
            Some(x % max)
        } else {
            None
        }
    } else {
        Some(x)
    }
}

impl Iterator for Walker {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        let new_x = wrap_or_oob(self.position.x + self.dx, self.max_x as isize, self.wrap_x)?;
        let new_y = wrap_or_oob(self.position.y + self.dy, self.max_y as isize, self.wrap_y)?;
        self.position = (new_x, new_y).into();
        Some(Coord::from((new_x, new_y)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coords() {
        let grid = Grid::new(vec![1,1,1,1,1,2,2,2,2,2,3,3,3,3,3], 5, 3);
        assert_eq!(grid.idx_to_coord(0), (0,0).into());
        assert_eq!(grid.idx_to_coord(1), (1,0).into());
        assert_eq!(grid.idx_to_coord(5), (0,1).into());
        assert_eq!(grid.idx_to_coord(24), (4,4).into());
    }

    #[test]
    fn iter_row_col() {
        let grid = Grid::new(vec![1,2,3,4,5,6,7,8,9], 3, 3);
        assert_eq!(grid.iter_row(0).collect::<Vec<_>>(), vec![&1,&2,&3]);
        assert_eq!(grid.iter_row(1).collect::<Vec<_>>(), vec![&4,&5,&6]);
        assert_eq!(grid.iter_row(2).collect::<Vec<_>>(), vec![&7,&8,&9]);
        assert_eq!(grid.iter_col(0).collect::<Vec<_>>(), vec![&1,&4,&7]);
        assert_eq!(grid.iter_col(1).collect::<Vec<_>>(), vec![&2,&5,&8]);
        assert_eq!(grid.iter_col(2).collect::<Vec<_>>(), vec![&3,&6,&9]);
    }

    #[test]
    fn compare() {
        let grid = Grid::new(vec![1,1,1,2,2,2,1,1,1], 3, 3);
        assert!(grid.compare_rows(0, 2));
        assert!(!grid.compare_rows(0, 1));
        assert!(grid.compare_cols(0, 1));
    }

    #[test]
    fn walk_nowrap() {
        let grid = Grid::new(vec![1,1,1,1,1,2,2,2,2,2,3,3,3,3,3], 5, 3);
        let start = (0,0);
        let mut walker = grid.walk_fixed(start, 2, 1, false, false);
        assert_eq!(walker.next(), Some((2,1).into()));
        assert_eq!(walker.next(), Some((4,2).into()));
        assert_eq!(walker.next(), None);
    }
    #[test]
    fn walk_diagonal() {
        let grid = Grid::new(vec![1,1,1,1,1,2,2,2,2,2,3,3,3,3,3,4,4,4,4,4,5,5,5,5,5], 5, 5);
        let start = (0,0);
        let mut walker = grid.walk_fixed(start, 1, 1, false, false);
        assert_eq!(walker.next(), Some((1,1).into()));
        assert_eq!(walker.next(), Some((2,2).into()));
        assert_eq!(walker.next(), Some((3,3).into()));
        assert_eq!(walker.next(), Some((4,4).into()));
        assert_eq!(walker.next(), None);
    }

    #[test]
    fn walk_diagonal_wrap() {
        let grid = Grid::new(vec![1,1,1,1,1,2,2,2,2,2,3,3,3,3,3,4,4,4,4,4,5,5,5,5,5], 5, 5);
        let start = (0,0);
        let mut walker = grid.walk_fixed(start, 1, 1, true, true);
        assert_eq!(walker.next(), Some((1,1).into()));
        assert_eq!(walker.next(), Some((2,2).into()));
        assert_eq!(walker.next(), Some((3,3).into()));
        assert_eq!(walker.next(), Some((4,4).into()));
        assert_eq!(walker.next(), Some((0,0).into()));
    }


    #[test]
    fn walk_wrap_x() {
        let grid = Grid::new(vec![1,1,1,1,1,2,2,2,2,2,3,3,3,3,3], 5, 3);
        let start = (0,0);
        let mut walker = grid.walk_fixed(start, 4, 1, true, false);
        assert_eq!(walker.next(), Some((4,1).into()));
        assert_eq!(walker.next(), Some((3,2).into()));
        assert_eq!(walker.next(), None);

        //big step
        let mut walker = grid.walk_fixed(start, 8, 1, true, false);
        assert_eq!(walker.next(), Some((3,1).into()));

    }

    #[test]
    fn walk_wrap_x_y() {
        let grid = Grid::new(vec![1,1,1,1,1,2,2,2,2,2,3,3,3,3,3], 5, 3);
        let start = (0,0);
        let mut walker = grid.walk_fixed(start, 5, 1, true, false);
        assert_eq!(walker.next(), Some((0,1).into()));
        assert_eq!(walker.next(), Some((0,2).into()));
        assert_eq!(walker.next(), None);
    }
}
