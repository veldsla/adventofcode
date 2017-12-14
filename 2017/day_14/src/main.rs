use std::collections::HashSet;

mod knot_hash;
use knot_hash::knot_hash;

fn reduce_clusters(mut grid: Vec<Vec<Option<u32>>>) -> usize {

    let dim = grid.len();
    let edge = dim - 1;
    let mut reducing = true;
    while reducing {
        reducing = false;
        for x in 0..128 {
            for y in 0..128 {
                if x > 0 && grid[x-1][y].is_some() && grid[x-1][y] < grid[x][y] {
                    grid[x][y] = grid[x-1][y];
                    reducing = true
                }
                if x < edge && grid[x+1][y].is_some() && grid[x+1][y] < grid[x][y] {
                    grid[x][y] = grid[x+1][y];
                    reducing = true
                }
                if y > 0 && grid[x][y-1].is_some() && grid[x][y-1] < grid[x][y] {
                    grid[x][y] = grid[x][y-1];
                    reducing = true
                }
                if y < edge && grid[x][y+1].is_some() && grid[x][y+1] < grid[x][y] {
                    grid[x][y] = grid[x][y+1];
                    reducing = true
                }
            }
        }
    }
    
    let h: HashSet<u32> = grid.iter().flat_map(|r| r.iter().map(|v| v.unwrap_or(0))).collect();
    h.len() - 1
}

fn main() {
    let input = "stpzcrnm";
    //test input:
    //let input = "flqrgnkx";

    let hashes:Vec<String> = (0..128)
        .map(|i| format!("{}-{}", input, i))
        .map(|s| knot_hash(&s)).collect();

    let sum: u32 = hashes.iter()
        .flat_map(|h| {
            h.chars().map(|b| b.to_digit(16).unwrap().count_ones())
        })
        .sum();
    println!("14a: {} squares are used.", sum);

    //convert used squares to increasing u32's making every square a unique group
    let mut num = 0;
    let grid: Vec<Vec<Option<u32>>> = hashes.iter()
        .map(|h| {
            h.chars().flat_map(|b| {
                let v = b.to_digit(16).unwrap();
                let mut out = Vec::new();
                for t in [8,4,2,1].iter() {
                    if v & t != 0 {
                        num += 1;
                        out.push(Some(num));
                    } else {
                        out.push(None);
                    }
                }
                out
            }).collect::<Vec<Option<u32>>>()
        }).collect();
    println!("14b: Grid has {} groups", reduce_clusters(grid));
}

