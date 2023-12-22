#[allow(unused_imports)]
use anyhow::{anyhow, Result};
use ndarray::{Array3, s};
use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    IResult
};


use crate::Problem;
use crate::parsers::positive_integer;

#[derive(Default)]
pub struct Solution {
    data: Vec<Block>,
}

#[derive(Debug, Clone)]
struct Block {
    id: usize,
    x1: usize,
    y1: usize,
    z1: usize,
    x2: usize,
    y2: usize,
    z2: usize,
}

impl Block {
    // iter coords
    fn iter(&self) -> impl Iterator<Item = (usize, usize, usize)>  +'_{
        (self.x1..=self.x2)
            .flat_map(move |x| (self.y1..=self.y2).map(move |y| (x, y)))
            .flat_map(move |(x, y)| (self.z1..=self.z2).map(move |z| (x, y, z)))
    }
}

fn fall_blocks(m: &mut Array3<usize>, blocks: &mut [Block]) -> usize {
    // iterate trough the blocks from bottom to top
    // mark the blocks that are not resting on anything
    // and move them down one step
    // repeat until no blocks are moved
    
    let mut fallen = vec![false; blocks.len()];
    loop {
        let mut moved = false;
        for (i, block) in blocks.iter_mut().enumerate() {
            if block.z1 == 1 {
                // block is on the bottom
                continue;
            }

            // if all below free, move down
            if block.iter().all(|(x, y, _z)| m[[x, y, block.z1 - 1]] == 0) {
                // block is not resting on anything
                //update the matrix
                for x in block.x1..=block.x2 {
                    for y in block.y1..=block.y2 {
                        assert_eq!(m[[x, y, block.z2]], block.id);
                        m[[x, y, block.z2]] = 0;
                        m[[x, y, block.z1 - 1]] = block.id;
                    }
                }
                // move it down one step
                block.z1 -= 1;
                block.z2 -= 1;

                fallen[i] = true;
                moved = true;
            } 
        }
        if !moved {
            break;
        }
    }
    fallen.iter().filter(|&&f| f).count()
}

fn count_supporting_blocks(m: &Array3<usize>, blocks: &[Block], idx: usize) -> Vec<usize> {
    let mut ids = Vec::new();
    for x in blocks[idx].x1..=blocks[idx].x2 {
        for y in blocks[idx].y1..=blocks[idx].y2 {
            if m[[x, y, blocks[idx].z2 - 1]] != 0 {
                ids.push(m[[x, y, blocks[idx].z2 - 1]]);
            }
        }
    }

    ids.sort();
    ids.dedup();
    ids
}

fn can_delete(m: &Array3<usize>, blocks: &[Block], idx: usize) -> bool {
    // a block can be deleted if it is not supporting any other block
    // or if the other blocks are supported by another block
    let mut above = blocks[idx].iter().filter_map(|(x, y, _z)| {
        if m[[x, y, blocks[idx].z2 + 1]] > 0 {
            Some(m[[x, y, blocks[idx].z2 + 1]])
        } else {
            None
        }
    }).collect::<Vec<_>>();

    above.sort();
    above.dedup();

    for id in above {
        let supporting = count_supporting_blocks(m, blocks, id - 1);
        if supporting.len() <= 1 {
            return false;
        }
    }
    true
}


fn parse_coord(s: &str) -> IResult<&str, (usize, usize, usize)> {
    let mut coord = tuple((
        terminated(positive_integer, tag(",")),
        terminated(positive_integer, tag(",")),
        positive_integer));
    coord(s)
}

fn parse_block(s: &str) -> IResult<&str, Block> {
    map(separated_pair(parse_coord, tag("~"), parse_coord), |((x1, y1, z1), (x2, y2, z2))| {
        debug_assert!(z1 <= z2);
        Block { id: 0, x1, y1, z1, x2, y2, z2 }
    })(s)
}


impl Problem for Solution {
    fn parse(&mut self, s: &str) -> Result<()> {
        let (_s, mut data) = separated_list1(line_ending, parse_block)(s)
            .map_err(|e| anyhow!("parse error: {e}"))?;
        // assign ids
        data.iter_mut().enumerate().for_each(|(i, b)| b.id = i + 1);

        self.data = data;
        Ok(())
    }

    fn part1(&self) -> Result<String> {

        let mut blocks = self.data.clone();
        //get the dimensions of the space
        let max_x = blocks.iter().map(|b| b.x1.max(b.x2)).max().unwrap();
        let max_y = blocks.iter().map(|b| b.y1.max(b.y2)).max().unwrap();
        let max_z = blocks.iter().map(|b| b.z1.max(b.z2)).max().unwrap();

        let mut column = Array3::zeros((max_x + 1, max_y + 1, max_z + 2));

        for block in self.data.iter() {
            for x in block.x1..=block.x2 {
                for y in block.y1..=block.y2 {
                    for z in block.z1..=block.z2 {
                        column[[x, y, z]] = block.id;
                    }
                }
            }
        }

        fall_blocks(&mut column, &mut blocks);

        Ok((0..blocks.len()).filter(|idx| can_delete(&column, &blocks, *idx)).count().to_string())

    }

    fn part2(&self) -> Result<String> {
        let mut blocks = self.data.clone();
        //get the dimensions of the space
        let max_x = blocks.iter().map(|b| b.x1.max(b.x2)).max().unwrap();
        let max_y = blocks.iter().map(|b| b.y1.max(b.y2)).max().unwrap();
        let max_z = blocks.iter().map(|b| b.z1.max(b.z2)).max().unwrap();

        let mut column = Array3::zeros((max_x + 1, max_y + 1, max_z + 2));

        for block in self.data.iter() {
            for x in block.x1..=block.x2 {
                for y in block.y1..=block.y2 {
                    for z in block.z1..=block.z2 {
                        column[[x, y, z]] = block.id;
                    }
                }
            }
        }

        fall_blocks(&mut column, &mut blocks);

        // destroy everu block and count the falling ones
        Ok((0..blocks.len()).map(|idx| {
            let mut column = column.clone();
            let mut blocks = blocks.clone();
            let rem = blocks.remove(idx);

            //destroy the block in the matrix
            for x in rem.x1..=rem.x2 {
                for y in rem.y1..=rem.y2 {
                    for z in rem.z1..=rem.z2 {
                        column[[x, y, z]] = 0;
                    }
                }
            }
            fall_blocks(&mut column, &mut blocks)
        }).sum::<usize>().to_string())

    }
}

