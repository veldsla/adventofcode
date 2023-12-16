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


use crate::Problem;
use crate::grid::{Grid, Coord};

#[derive(Default)]
pub struct Solution {
    grid: Grid<Mirror>,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
enum Mirror {
    Horizontal,
    Vertical,
    Diagonal,
    Antidiagonal,
    #[default]
    NoMirror,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
struct Beam {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Mirror {
    // Generate the new directions or direction for a beam hitting a mirror
    fn beam_hit(self, mut beam: Beam) -> (Beam, Option<Beam>) {
        match self {
            Mirror::Horizontal => {
                match beam.direction {
                    Direction::Up => { let b = beam.split(); (b.0, Some(b.1)) },
                    Direction::Down => { let b = beam.split(); (b.0, Some(b.1)) },
                    Direction::Left => (beam, None),
                    Direction::Right => (beam, None),
                }
            },
            Mirror::Vertical => {
                match beam.direction {
                    Direction::Up => (beam, None),
                    Direction::Down => (beam, None),
                    Direction::Left => { let b = beam.split(); (b.0, Some(b.1)) },
                    Direction::Right => { let b = beam.split(); (b.0, Some(b.1)) },
                }
            },
            Mirror::Diagonal => {
                match beam.direction {
                    Direction::Up | Direction::Down => { beam.left(); (beam, None) },
                    Direction::Left | Direction::Right => { beam.right(); (beam, None) },
                }
            },
            Mirror::Antidiagonal => {
                match beam.direction {
                    Direction::Up | Direction::Down => { beam.right(); (beam, None) },
                    Direction::Left | Direction::Right => { beam.left(); (beam, None) },
                }
            },
            Mirror::NoMirror => (beam, None),
        }
    }

}

impl Beam {
    fn new(x: isize, y: isize, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    // split the beam into two beam going in orhtogonal directions
    fn split(self) -> (Beam, Beam) {
        match self.direction {
            Direction::Up => (Beam { x: self.x, y: self.y, direction: Direction::Left },
                              Beam { x: self.x, y: self.y, direction: Direction::Right }),
            Direction::Down => (Beam { x: self.x, y: self.y, direction: Direction::Left },
                                Beam { x: self.x, y: self.y, direction: Direction::Right }),
            Direction::Left => (Beam { x: self.x, y: self.y, direction: Direction::Up },
                                Beam { x: self.x, y: self.y, direction: Direction::Down }),
            Direction::Right => (Beam { x: self.x, y: self.y, direction: Direction::Up },
                                 Beam { x: self.x, y: self.y, direction: Direction::Down }),
        }
    }

    fn reverse(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Up,
            Direction::Left => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Left,
        }
    }

    fn right(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
        }
    }

    fn left(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Left,
            Direction::Down => self.direction = Direction::Right,
            Direction::Left => self.direction = Direction::Down,
            Direction::Right => self.direction = Direction::Up,
        }
    }

    fn travel(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

impl Direction {
    fn to_bits(&self) -> u8 {
        match self {
            Direction::Up => 0b0001,
            Direction::Down => 0b0010,
            Direction::Left => 0b0100,
            Direction::Right => 0b1000,
        }
    }

    fn matches(&self, bits: u8) -> bool {
        self.to_bits() & bits != 0
    }
}

fn parse_grid(s: &str) -> IResult<&str, Grid<Mirror>> {
    let gridline = terminated(is_a(".|-/\\"), alt((line_ending, eof)));
    let grid = fold_many1(gridline, || (Vec::new(), 0usize, 0usize), |(mut v, _x, y), line: &str| {
        //dimnension checks?
        let x = line.len();
        v.extend(line.chars().map(|c| match c {
            '.' => Mirror::NoMirror,
            '|' => Mirror::Vertical,
            '-' => Mirror::Horizontal,
            '\\' => Mirror::Diagonal,
            '/' => Mirror::Antidiagonal,
            _ => unreachable!(),
        }));
        (v, x, y+1)
    });
    map(all_consuming(grid), |(elements, dim_x, dim_y)| Grid::new(elements, dim_x, dim_y))(s)
}



impl Problem for Solution {
    fn parse(&mut self, s: &str) -> Result<()> {
        let (_s, grid) = parse_grid(s).map_err(|e| anyhow!("parse error: {e}"))?;
        self.grid = grid;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        let beam = Beam::new(-1, 0, Direction::Right);
        let mut beams = vec![beam];
        let mut newbeams = Vec::new();

        let mut seen_directions = vec![0; self.grid.elements.len()];
        //seen_directions[0] = Direction::Right.to_bits();

        while !beams.is_empty() {
            for mut beam in beams.drain(..) {
                beam.travel();
                if beam.x < 0 || beam.x >= self.grid.dim_x as isize || beam.y < 0 || beam.y >= self.grid.dim_y as isize {
                    // exit the grid
                    continue;
                }

                let idx = self.grid.coord_to_idx((beam.x, beam.y).into());

                if beam.direction.matches(seen_directions[idx]) {
                    // we've already been here
                    continue;
                }

                seen_directions[idx] |= beam.direction.to_bits();

                let mirror = &self.grid[idx];
                let (beam, new_beam) = mirror.beam_hit(beam);
                newbeams.push(beam);
                if let Some(new_beam) = new_beam {
                    newbeams.push(new_beam);
                }
            }
            std::mem::swap(&mut beams, &mut newbeams);
        }

        Ok(seen_directions.iter().filter(|&&b| b != 0).count().to_string())
    }

   fn part2(&self) -> Result<String> {
        // test beams from all the edges
        let starts: Vec<Beam> = (0..self.grid.dim_x as isize).map(|x| Beam::new(x, -1, Direction::Down))
            .chain((0..self.grid.dim_x as isize).map(|x| Beam::new(x, self.grid.dim_y as isize, Direction::Up)))
            .chain((0..self.grid.dim_y as isize).map(|y| Beam::new(-1, y, Direction::Right)))
            .chain((0..self.grid.dim_y as isize).map(|y| Beam::new(self.grid.dim_x as isize, y, Direction::Left)))
            .collect();

        // I could create a cache for the seen directions and energized tiles....but hey
        
        let max = starts.into_iter().map(|start| {
            let mut beams = vec![start];
            let mut newbeams = Vec::new();

            let mut seen_directions = vec![0; self.grid.elements.len()];
            //seen_directions[0] = Direction::Right.to_bits();

            while !beams.is_empty() {
                for mut beam in beams.drain(..) {
                    beam.travel();
                    if beam.x < 0 || beam.x >= self.grid.dim_x as isize || beam.y < 0 || beam.y >= self.grid.dim_y as isize {
                        // exit the grid
                        continue;
                    }

                    let idx = self.grid.coord_to_idx((beam.x, beam.y).into());

                    if beam.direction.matches(seen_directions[idx]) {
                        // we've already been here
                        continue;
                    }

                    seen_directions[idx] |= beam.direction.to_bits();

                    let mirror = &self.grid[idx];
                    let (beam, new_beam) = mirror.beam_hit(beam);
                    newbeams.push(beam);
                    if let Some(new_beam) = new_beam {
                        newbeams.push(new_beam);
                    }
                }
                std::mem::swap(&mut beams, &mut newbeams);
            }

            seen_directions.iter().filter(|&&b| b != 0).count()

        }).max().unwrap();

        Ok(max.to_string())


   }
}

