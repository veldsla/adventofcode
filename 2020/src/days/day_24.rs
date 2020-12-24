use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::Problem;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    multi::many1,
    combinator::{all_consuming, map},
    sequence::terminated,
    IResult
};

#[derive(Default)]
pub struct Solution {
    input: Vec<Walk>,
}
type Walk = Vec<Direction>;

#[derive(Debug, Clone, Copy)]
enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE
}

type Floor = HashMap<(i32, i32), bool>;

fn parse(i: &[u8]) -> IResult<&[u8], Vec<Walk>> {
    let e = map(tag("e"), |_| Direction::E);
    let se = map(tag("se"), |_| Direction::SE);
    let sw = map(tag("sw"), |_| Direction::SW);
    let w = map(tag("w"), |_| Direction::W);
    let nw = map(tag("nw"), |_| Direction::NW);
    let ne = map(tag("ne"), |_| Direction::NE);
    let walk = terminated(many1(alt((e, se, sw, w, nw, ne))), line_ending);
    all_consuming(many1(walk))(i)
}

fn walk(w: &Walk) -> (i32, i32) {
    let mut coord = (0,0);
    for d in w {
        let even = coord.1 % 2 == 0;
        match d {
            Direction::E => coord.0 += 1,
            Direction::SE if even => coord.1 += 1,
            Direction::SE => {coord.0 += 1; coord.1 += 1},
            Direction::SW if even => { coord.0 -= 1; coord.1 += 1 },
            Direction::SW => coord.1 += 1,
            Direction::W => coord.0 -= 1,
            Direction::NW if even => { coord.0 -= 1; coord.1 -= 1 },
            Direction::NW => coord.1 -= 1,
            Direction::NE if even => coord.1 -= 1,
            Direction::NE => {coord.0 += 1; coord.1 -= 1},
        }
    }
    coord
}

fn do_flips(instr: &[Walk]) -> Floor {
    let mut floor = HashMap::new();
    for w in instr {
        let coord = walk(w);
        flip_tile(coord, &mut floor);
    }
    floor
}

fn count_flipped<F: std::borrow::Borrow<Floor>>(floor: F) -> usize {
    floor.borrow().values().filter(|&&t| !t).count()
}

fn do_life(instr: &[Walk], ndays: usize) -> usize {
    let floor = (0..ndays).fold(do_flips(instr), |mut floor, _| {
        let changes: Vec<_> = floor.iter().filter_map(|(coord, &t)| {
            let c = count_black_neigh(*coord, &floor);
            if !t && (c == 0 || c > 2) {
                Some(*coord)
            } else if t && c == 2 {
                Some(*coord)
            } else {
                None
            }
        }).collect();
        changes.into_iter().for_each(|c| flip_tile(c, &mut floor));
        floor
    });
    count_flipped(floor)  
}

fn neighbors(c: (i32, i32)) -> Vec<(i32, i32)> {
    //common
    let mut res = Vec::new();
    res.push((c.0-1, c.1));
    res.push((c.0, c.1-1));
    res.push((c.0, c.1+1));
    res.push((c.0+1, c.1));

    if c.1 % 2 == 0 {
        res.push((c.0-1, c.1-1));
        res.push((c.0-1, c.1+1));
    } else {
        res.push((c.0+1, c.1-1));
        res.push((c.0+1, c.1+1));
    }

    res
}

fn count_black_neigh(c: (i32, i32), floor: &Floor) -> usize {
    let mut count = 0;
    for c in neighbors(c) {
        if let Some(false) = floor.get(&c) { count += 1; }
    }

    count
}

fn flip_tile(c: (i32, i32), floor: &mut Floor) {
    let e = floor.entry(c).or_insert(true);
    match &e {
        true => {
            *e = false;
            //add  neighbor whites
            for c in neighbors(c) {
                floor.entry(c).or_insert(true);
            }
        },
        false => *e = true,
    }
}

impl Problem for Solution {
    fn parse(&mut self, i: &[u8]) -> Result<()> {
        let result = parse(i).map_err(|e| anyhow!(e.to_string()))?;
        self.input = result.1;
        Ok(())
    }

    fn part1(&self) -> Result<String> {
        Ok(format!("{}", count_flipped(do_flips(&self.input))))
    }

    fn part2(&self) -> Result<String> {

        Ok(format!("{}", do_life(&self.input, 100)))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &[u8] = b"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
";

    #[test]
    fn p1() {
        let result = parse(TEST);
        println!("{:?}", result);
        assert!(result.is_ok());
        let walks = result.unwrap().1;
        assert_eq!(count_flipped(do_flips(&walks)), 10);
        assert_eq!(do_life(&walks, 100), 2208);
    }

}
