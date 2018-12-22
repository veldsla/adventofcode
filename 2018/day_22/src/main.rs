use std::collections::HashMap;
use std::fmt;

use ndarray::{Array2, s};
use itertools::Itertools;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Region {
    Rock = 0,
    Wet = 1,
    Narrow = 2
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Equipment {
    Torch,
    Gear,
    Neither
}


impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Region::Rock => '.',
            Region::Wet => '=',
            Region::Narrow => '|'
        })
    }
}

struct Cave {
    map: Array2<Region>,
}

impl Cave {
    fn new(depth: i64, target: &(usize, usize), extend: usize) -> Cave {
        let mut erosion_level = Array2::<i64>::zeros((target.1 as usize + 1 + extend, target.0 as usize + 1 + extend));
        erosion_level.row_mut(0)
            .iter_mut()
            .enumerate()
            .skip(1)
            .for_each(|(y, e)| *e = (y as i64 * 16807 + depth ) % 20183 );

        erosion_level.column_mut(0)
            .iter_mut()
            .enumerate()
            .skip(1)
            .for_each(|(x, e)| *e = (x as i64 * 48271 + depth ) % 20183 );

        //fill the remainder
        (1..=target.1+extend).cartesian_product(1..=target.0+extend)
            .for_each(|(row, col)| { erosion_level[[row, col]] = ((erosion_level[[row-1, col]] * erosion_level[[row, col - 1]]) + depth ) % 20183});

        //force target back to zero 
        erosion_level[[target.1, target.0]] = 0;

        //create the map
        let map: Vec<_> = erosion_level.iter().map(|e| {
            match e % 3 {
                0 => Region::Rock,
                1 => Region::Wet,
                2 => Region::Narrow,
                _ => unreachable!()
            }
        }).collect();


        Cave { map: Array2::from_shape_vec(erosion_level.dim(), map).unwrap() }
    }

    fn risk_level(&self, target: &(usize, usize)) -> i64 {
        self.map.slice(s![0..=target.1, 0..=target.0]).iter().map(|e| *e as i64).sum()

    }

    fn climb(&self, target: &(usize, usize)) -> usize {
        let mut queue = Vec::new();
        let mut visited = HashMap::new();
        queue.push((0, 0, 0, Equipment::Torch));
        let mut at_target = Vec::new();

        while let Some((row, col, time, eq)) = queue.pop() {
            if let Some(v) = visited.get(&(row, col, eq)) {
                if *v <= time {
                    continue;
                }
            }

            if col == target.0 && row == target.1 {
                at_target.push((row, col, eq, time));
            }

            visited.insert((row, col, eq), time);
            
            //get possible targets
            let moves = self.get_adjacent(row, col);

            let current = self.map[[row, col]];
            //make the moves only switch if required
            for (crow, ccol) in moves {
                let to = self.map[[crow, ccol]];
                match to {
                    Region::Rock if eq == Equipment::Neither => {
                        if current != Region::Narrow { queue.push((crow, ccol, time + 8, Equipment::Gear)); }
                        if current != Region::Wet { queue.push((crow, ccol, time + 8, Equipment::Torch)); }
                    },
                    Region::Rock => {
                        queue.push((crow, ccol, time + 1, eq))
                    },
                    Region::Wet if eq == Equipment::Torch => {
                        if current != Region::Narrow { queue.push((crow, ccol, time + 8, Equipment::Gear)); }
                        if current != Region::Rock { queue.push((crow, ccol, time + 8, Equipment::Neither)); }
                    },
                    Region::Wet => {
                        queue.push((crow, ccol, time + 1, eq))
                    },
                    Region::Narrow if eq == Equipment::Gear => {
                        if current != Region::Wet { queue.push((crow, ccol, time + 8, Equipment::Torch)); }
                        if current != Region::Rock { queue.push((crow, ccol, time + 8, Equipment::Neither)); }
                    },
                    Region::Narrow => queue.push((crow, ccol, time + 1, eq)),
                }
            }

            // sort queue on time (min heap would be better, but only a max-heap in the stdlib, too
            // lazy to define a struct and implement Ord)
            queue.sort_by(|a, b| a.2.cmp(&b.2).reverse());
        }

        println!("Found target with: {:?}", at_target);
        at_target.into_iter().map(|e| {
            match e.2 {
                Equipment::Torch => e.3,
                _ => e.3 + 7
            }
        }).min().unwrap()
    }


    fn get_adjacent(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut adjacent = Vec::new();
        if row > 0 {
            adjacent.push((row-1, col))
        }
        if col > 0 {
            adjacent.push((row, col-1))
        }
        if row < self.map.rows() - 1 {
            adjacent.push((row + 1, col));
        }
        if col < self.map.cols() - 1 {
            adjacent.push((row, col + 1));
        }
        adjacent
    }
}


fn main() {
    let depth = 7863;
    let target = (14, 760);
    let cave = Cave::new(depth, &target, 30);
    println!("cave has {} rows and {} cols", cave.map.rows(), cave.map.cols());
    println!("22a: Risk level cave {}", cave.risk_level(&target));
    println!("22a: Time to reach target {} minutes", cave.climb(&target));
}


#[test]
fn test() {
    let cave = Cave::new(510, &(10,10), 5);
    //check some chars
    println!("{}", cave.map);
    assert_eq!(cave.map[[0,0]], Region::Rock);
    assert_eq!(cave.map[[0,1]], Region::Wet);
    assert_eq!(cave.map[[0,2]], Region::Rock);
    assert_eq!(cave.map[[0,3]], Region::Narrow);
    assert_eq!(cave.map[[9,1]], Region::Wet);
    assert_eq!(cave.map[[9,2]], Region::Wet);
    assert_eq!(cave.map[[9,3]], Region::Wet);
    assert_eq!(cave.map[[9,4]], Region::Wet);
    assert_eq!(cave.map[[9,5]], Region::Wet);
    
    assert_eq!(cave.risk_level(&(10, 10)), 114);
    assert_eq!(cave.climb(&(10, 10)), 45);
}
