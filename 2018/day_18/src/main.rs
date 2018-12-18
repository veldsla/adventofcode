use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use itertools::Itertools;

#[derive(Debug)]
struct Field {
    data: Vec<char>,
    neighbours: Vec<Vec<usize>>
}

impl Field {
    fn from_file<P: AsRef<Path>>(p: P, dim: usize) -> io::Result<Field> {
        let mut f = File::open(p)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        let data = s.split_whitespace()
            .flat_map(|l| l.chars())
            .collect::<Vec<char>>();
        assert_eq!(data.len(), dim*dim);

        let neighbours = (0..dim).cartesian_product(0..dim).map(|(y, x)| {
            //precalculate the coordinates around x, y
            let ax = if x < dim - 1 { 1 } else { 0 };
            let ay = if y < dim - 1 { 1 } else { 0 };
            (x.saturating_sub(1)..=x+ax)
                .cartesian_product(y.saturating_sub(1)..=y+ay)
                .filter(|(cx, cy)| !(*cx == x && *cy == y))
                .map(|(cx, cy)| cy * dim + cx)
                .collect::<Vec<usize>>()
        }).collect::<Vec<_>>();

        Ok(Field { data, neighbours })
    }

    fn evolve(&mut self) {
        let new_field: Vec<char> = self.neighbours.iter().enumerate().map(|(tile, neigh)| {
            let (lumber, trees) = neigh.iter().fold((0,0), |acc, pos| {
                match self.data[*pos] {
                    '#' => (acc.0 + 1, acc.1),
                    '|' => (acc.0, acc.1 + 1),
                    _ => acc
                }
            });
            match self.data[tile] {
                '.' => if trees >= 3 { '|' } else { '.' },
                '|' => if lumber >= 3 { '#' } else { '|' },
                '#' => if lumber >= 1 && trees >= 1 { '#' } else { '.' },
                _ => unreachable!()
            }
        }).collect();
        std::mem::replace(&mut self.data, new_field);
    }

    fn count(&self) -> (usize, usize) {
        self.data.iter().fold((0,0), |acc, tile| {
            match tile {
                '#' => (acc.0 + 1, acc.1),
                '|' => (acc.0, acc.1 + 1),
                _ => acc
            }
        })
    }
}


fn main() {
    let mut field = Field::from_file("input.txt", 50).unwrap();
    for _ in 0..10 {
        field.evolve();
    }
    let (lumber, trees) = field.count();
    println!("18a: After 10 minutes the field contains {} lumbermills and {} trees for a resource value of {}",
             lumber, trees, lumber * trees);

    let mut field = Field::from_file("input.txt", 50).unwrap();
    let mut seen = HashMap::new();
    let target: usize = 1_000_000_000;
    for t in 0.. {
        field.evolve();
        if let Some(previous) = seen.insert(field.data.clone(), t) {
            let d = t - previous;
            //repeat every d minutes
            let remaining = (target - t) % d;
            for i in 1..remaining {
                field.evolve()
            }
            let (lumber, trees) = field.count();
            println!("18b: After 1e9 minutes the field contains {} lumbermills and {} trees for a resource value of {}",
                lumber, trees, lumber * trees);
            break;
        }
    }
}

#[test]
fn test() {
    let mut field = Field::from_file("test.txt", 10).unwrap();
    println!("{:?}", field);
    for _ in 0..10 {
        field.evolve();
    }
    let (lumber, trees) = field.count();
    assert_eq!(lumber*trees, 1147);
}
